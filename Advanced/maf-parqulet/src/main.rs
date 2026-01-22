use anyhow::{Context, Result};
use polars::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<()> {
    let maf_path = std::env::args().nth(1).context("Usage: maf_to_parquet <file.maf.tsv>")?;
    let out_path = std::env::args().nth(2).unwrap_or_else(|| "maf.parquet".to_string());

    let file = File::open(&maf_path).with_context(|| format!("Cannot open {maf_path}"))?;
    let reader = BufReader::new(file);

    let df: DataFrame = CsvReader::new(reader)
        .with_options(
            CsvReadOptions::default()
                .with_has_header(true)
                .with_parse_options(
                    CsvParseOptions::default()
                        .with_separator(b'\t')
                        .with_comment_prefix(Some("#")),
                ),
        )
        .finish()
        .context("Failed to read MAF/TSV")?;
    
    
    let re = Regex::new(r"_T_.*$")?;
    let tumor_col = df
        .column("Tumor_Sample_Barcode")
        .context("Missing column: Tumor_Sample_Barcode")?
        .str()
        .context("Tumor_Sample_Barcode is not UTF8")?;

    let patient_id: ChunkedArray<StringType> = tumor_col
        .into_iter()
        .map(|opt| opt.map(|s| re.replace(s, "").to_string()))
        .collect();

    let mut df = df.clone();
    let mut patient_series = patient_id.into_series();
    patient_series.rename("patient_id".into());
    df.with_column(patient_series)?;
    
    let n_rows = df.height();
    let n_patients = df.column("patient_id")?.n_unique()?;
    println!("Unique patients: {n_patients}");
    println!("Rows: {n_rows}, Unique patients: {n_patients}");

    println!("Columns ({}):", df.get_columns().len());
    for c in df.get_columns() {
        println!(" - {}: {:?}", c.name(), c.dtype());
    }

    println!("\n=== TOP patients ===");
    
    let top = df
        .clone()
        .lazy()
        .group_by([col("patient_id")])
        .agg([col("patient_id").count().alias("rows")])
        .sort(["rows"], SortMultipleOptions::default()
            .with_order_descending(true))
        .limit(20)
        .collect()?;
    println!("patient_id\trows");

    let patient_ids = top.column("patient_id")?.str()?;
    let rows = top.column("rows")?.u32()?;

    for i in 0..top.height() {
        println!(
            "{}\t{}",
            patient_ids.get(i).unwrap_or(""),
            rows.get(i).unwrap_or(0)
        );
    }

    println!("\n=== TOP genes ===");
    let top_genes = df
        .clone()
        .lazy()
        .filter(
            col("Hugo_Symbol")
                .is_not_null()
                .and(col("Hugo_Symbol").neq(lit("Unknown")))
                .and(col("Hugo_Symbol").neq(lit("")))
        )
        .group_by([col("Hugo_Symbol")])
        .agg([len().alias("rows")])
        .sort(
            ["rows"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .limit(20)
        .collect()?;

    println!("Hugo_Symbol\trows");
    let genes = top_genes.column("Hugo_Symbol")?.str()?;
    let rows = top_genes.column("rows")?.u32()?;
    for i in 0..top_genes.height() {
        println!(
            "{}\t{}",
            genes.get(i).unwrap_or(""),
            rows.get(i).unwrap_or(0)
        );
    }
    
    let file = File::create(&out_path)?;
    ParquetWriter::new(file)
        .with_statistics(StatisticsOptions::default())
        .with_compression(ParquetCompression::Snappy)
        .finish(&mut df)?;

    println!("Wrote {out_path}");
    Ok(())
}