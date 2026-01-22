use regex::Regex;
use serde_json::json;

pub fn r_regex(){
    let url = r"^https://e260-nb012\\.inet\\.dkfz-heidelberg\\.de/ccp-teiler/.*$";
    let test_json = json!(url);
    println!("r: {}", test_json);
    let original = "^https://e260-nb012\\.inet\\.dkfz-heidelberg\\.de/ccp-teiler/.*$";
    println!("original: {}", original);
}

pub fn test_crate_regex() {
    let url = "https://e260-nb012.inet.dkfz-heidelberg.de/opal/*";
    let wrong_url = Regex::new(url).unwrap();
    //let test_json = json!(wrong_url);
    println!("r: {}", wrong_url);
}