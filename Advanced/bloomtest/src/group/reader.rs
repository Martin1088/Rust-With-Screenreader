use std::{fs::File, io::BufReader};

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
    let file = File::open("idat-10k.csv").expect("Provide idat-10k.csv file for testing");
    let mut reader = BufReader::new(file);
}
