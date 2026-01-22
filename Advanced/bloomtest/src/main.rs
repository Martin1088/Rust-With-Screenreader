use group::{hash_test::siphash_example, reader::csv_reader};

mod group;

fn main() {
    println!(
        "Current working directory: {:?}",
        std::env::current_dir().unwrap()
    );
    csv_reader();

    let value = "Erwin";
    let hash = siphash_example(&value);
    println!("SipHash-2-4 of '{}': {}", value, hash);
}
