use iceberg::writer::IcebergWriterBuilder;
use iceberg::writer::IcebergWriter;
use iceberg::spec::DataFile;
use anyhow::{Context, Result};
use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema as ArrowSchema};
use iceberg::io::{
    FileIO, FileIOBuilder, S3_ACCESS_KEY_ID, S3_ENDPOINT, S3_PATH_STYLE_ACCESS, S3_REGION,
    S3_SECRET_ACCESS_KEY,
};
use iceberg::spec::{NestedField, PrimitiveType, Schema, Type};
use iceberg::transaction::{ApplyTransactionAction, Transaction};
use iceberg::{Catalog, CatalogBuilder, NamespaceIdent, TableCreation, TableIdent};

use iceberg_catalog_rest::{
    RestCatalogBuilder, REST_CATALOG_PROP_URI, REST_CATALOG_PROP_WAREHOUSE,
};

use iceberg::writer::base_writer::data_file_writer::DataFileWriterBuilder;
use iceberg::writer::file_writer::location_generator::{
    DefaultFileNameGenerator, DefaultLocationGenerator,
};
use iceberg::writer::file_writer::ParquetWriterBuilder;

use parquet::basic::{Compression, ZstdLevel};
use parquet::file::properties::WriterProperties;

use std::collections::HashMap;
use std::sync::Arc;

const CATALOG_URI: &str = "http://localhost:8181/catalog";
const GARAGE_S3_ENDPOINT: &str = "http://localhost:3900";

// Must match what you configured in Lakekeeper:
const WAREHOUSE_NAME: &str = "genomics-warehouse";

// Namespace / table we will create (or reuse)
const NAMESPACE: &str = "genomics";
const TABLE: &str = "variants_demo";

// Garage specifics
const S3_REGION_NAME: &str = "garage";
// Garage typically requires path-style URLs:
const S3_PATH_STYLE: &str = "true";

fn iceberg_schema() -> Schema {
    Schema::builder()
        .with_schema_id(1)
        .with_fields(vec![
            NestedField::required(1, "sample_id", Type::Primitive(PrimitiveType::String)).into(),
            NestedField::required(2, "chrom", Type::Primitive(PrimitiveType::String)).into(),
            NestedField::required(3, "pos", Type::Primitive(PrimitiveType::Int)).into(),
            NestedField::required(4, "ref", Type::Primitive(PrimitiveType::String)).into(),
            NestedField::required(5, "alt", Type::Primitive(PrimitiveType::String)).into(),
        ])
        .build()
        .expect("schema build")
}

fn f(name: &str, dt: DataType, nullable: bool, id: i32) -> Field {
    let mut md = HashMap::new();
    md.insert("PARQUET:field_id".to_string(), id.to_string());
    Field::new(name, dt, nullable).with_metadata(md)
}

fn arrow_batch() -> Result<RecordBatch> {
    let schema = ArrowSchema::new(vec![
        f("sample_id", DataType::Utf8, false, 1),
        f("chrom",     DataType::Utf8, false, 2),
        f("pos",       DataType::Int32, false, 3),
        f("ref",       DataType::Utf8, false, 4),
        f("alt",       DataType::Utf8, false, 5),
    ]);


    let sample = StringArray::from(vec!["S1", "S1", "S2"]);
    let chrom = StringArray::from(vec!["7", "17", "7"]);
    let pos = Int32Array::from(vec![140453136, 7579472, 55249071]);
    let rref = StringArray::from(vec!["A", "C", "G"]);
    let alt = StringArray::from(vec!["T", "T", "A"]);

    let cols: Vec<ArrayRef> = vec![
        Arc::new(sample),
        Arc::new(chrom),
        Arc::new(pos),
        Arc::new(rref),
        Arc::new(alt),
    ];

    Ok(RecordBatch::try_new(Arc::new(schema), cols)?)
}

async fn connect_catalog() -> Result<impl Catalog> {
    let catalog = RestCatalogBuilder::default()
        .load(
            "rest",
            HashMap::from([
                (REST_CATALOG_PROP_URI.to_string(), CATALOG_URI.to_string()),
                (REST_CATALOG_PROP_WAREHOUSE.to_string(), WAREHOUSE_NAME.to_string()),
            ]),
        )
        .await
        .context("connect REST catalog")?;
    Ok(catalog)
}

fn build_garage_file_io() -> Result<FileIO> {
    let mut b = FileIOBuilder::new("s3")
        .with_prop(S3_ENDPOINT, GARAGE_S3_ENDPOINT)
        .with_prop(S3_REGION, S3_REGION_NAME)
        .with_prop(S3_PATH_STYLE_ACCESS, S3_PATH_STYLE);

    // creds from env (recommended); also set explicitly if present:
    if let Ok(ak) = std::env::var("AWS_ACCESS_KEY_ID") {
        b = b.with_prop(S3_ACCESS_KEY_ID, ak.as_str());
    }
    if let Ok(sk) = std::env::var("AWS_SECRET_ACCESS_KEY") {
        b = b.with_prop(S3_SECRET_ACCESS_KEY, sk.as_str());
    }

    Ok(b.build()?)
}

async fn ensure_table<C: Catalog>(catalog: &C) -> Result<TableIdent> {
    let ns = NamespaceIdent::from_vec(vec![NAMESPACE.to_string()])?;
    if !catalog.namespace_exists(&ns).await? {
        catalog.create_namespace(&ns, HashMap::new()).await?;
    }

    let ident = TableIdent::new(ns.clone(), TABLE.to_string());

    if !catalog.table_exists(&ident).await? {
        let schema = iceberg_schema();
        let creation = TableCreation::builder()
            .name(TABLE.to_string())
            .schema(schema)
            .properties(HashMap::new())
            .build();

        catalog.create_table(&ident.namespace, creation).await?;
    }

    Ok(ident)
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1) Connect to REST catalog (Lakekeeper)
    let catalog = connect_catalog().await?;
    println!("Connected to catalog: {}", CATALOG_URI);

    // 2) Create or load table
    let table_ident = ensure_table(&catalog).await?;
    let table = catalog.load_table(&table_ident).await?;
    println!("Loaded table: {}.{}", NAMESPACE, TABLE);

    // 3) Build S3 FileIO for Garage
    let file_io = build_garage_file_io()?;

    // 4) Parquet writer properties with ZSTD compression
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(ZstdLevel::try_new(3)?))
        .build();

    // 5) Build location + name generators (writes into table location)
    let location_generator =
        DefaultLocationGenerator::new(table.metadata().clone()).expect("location generator");
    let file_name_generator = DefaultFileNameGenerator::new(
        "demo".to_string(),
        None,
        iceberg::spec::DataFileFormat::Parquet,
    );

    // 6) Build Parquet writer + DataFile writer
    let parquet_writer = ParquetWriterBuilder::new(
        props,
        table.metadata().current_schema().clone(),
        None,
        file_io,
        location_generator,
        file_name_generator,
    );

    // Build Parquet + Iceberg writer
    let mut dfw: Box<dyn IcebergWriter> =
        Box::new(DataFileWriterBuilder::new(parquet_writer, None, 0)
            .build()
            .await?);
    let batch = arrow_batch()?;
    // Write Arrow batch
    dfw.write(batch).await?;

    // Close writer → Iceberg DataFiles
    let data_files: Vec<DataFile> = dfw.close().await?;
    println!("Wrote {} data file(s).", &data_files.len());

    // Commit via Iceberg transaction
    let tx = Transaction::new(&table);
    let action = tx.fast_append().add_data_files(data_files.clone());
    let tx = action.apply(tx)?;
    let _updated = tx.commit(&catalog).await?;
    println!("Wrote {} data file(s).", &data_files.len());

    // 8) Commit via fast append transaction
    let tx = Transaction::new(&table);
    let action = tx.fast_append().add_data_files(data_files);
    let tx = action.apply(tx)?;
    let _updated = tx.commit(&catalog).await?;
    println!("Committed snapshot OK.");

    Ok(())
}