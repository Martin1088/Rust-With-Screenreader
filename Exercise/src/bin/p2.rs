// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.
mod help_fn;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    outfile: PathBuf,
}

fn print_type_of<T>(_:&T) {
    println!("Type: {:?}", std::any::type_name::<T>())
}


fn read_file(result: &mut HashMap<i32, Contact>) -> std::io::Result<()> {
    let opts = Opt::from_args();
    let mut file = File::open(opts.infile)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line)?;
        match len {
            0 => break,
            _ => {
                let data = line.split(",");
                for d in data {
                    //print_type_of(&d);
                    if d.eq("") {
                        break;
                    }
                    print!("{} ", d);
                }
                line.clear();
                println!();
            }
        }
    }
    Ok(())

}

struct Contact {
    name: String,
    email: String,
}

fn main() {
    //let mut opt = Opt { infile: PathBuf::from("contacts_test.csv"), outfile: PathBuf::from("contacts_finish.csv"), };
    //opt.infile = PathBuf::from("contacts_test.csv");
    let mut contacts: HashMap<i32, Contact> = HashMap::new();
    let mut number:i32 = 1;
    let mut id:i32 = 501;
    match read_file(&mut contacts) {
        Ok(o) => println!(" {:?}", o),
        Err(e) => println!("{:?}", e),
    }

    while number != 5 {
        help_fn::menu();
        number = help_fn::input().parse::<i32>().expect("not a number");
        match number {
            1 => help_fn::menu(),
            2 => {
            } 
            5 => number = 5,
            _ => println!("invalid selection"),
        }
        id += 1;
    }

}
