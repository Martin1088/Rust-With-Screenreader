use bloomfilter::Bloom;
use serde::Deserialize;
use std::{fs::File, io::BufReader};

#[derive(Debug, Deserialize)]
pub struct Patients {
    extid: i32,
    name_prefix: String,
    vorname: String,
    nachname: String,
    gender: String,
    geburtsnames: String,
    geburtstag: String,
    geburtsmonat: String,
    geburtsjahr: String,
    ort: String,
    state: String,
    plz: String,
}

pub fn csv_reader() {
    let file = File::open("./idat-10k.csv").expect("Provide idat-10k.csv file for testing");
    //let mut reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    let items_num = 10000;
    let fp_rate = 0.001;

    let mut bloom_filter = Bloom::<String>::new_for_fp_rate(items_num, fp_rate).unwrap();
    for result in rdr.deserialize::<Patients>() {
        match result {
            Ok(record) => {
                if bloom_filter.check(&record.vorname) {
                    println!("Duplicate found: {}", record.vorname);
                } else {
                    bloom_filter.set(&record.vorname);
                }
            }
            Err(e) => {
                eprintln!("Error parsing record: {}", e);
            }
        }
    }
    dbg!(bloom_filter);

    // for res in rdr.deserialize::<Patients>() {
    //     let record: Patients = res.expect("not parseable!");
    //     println!("{:?}", record);
    // }
}
