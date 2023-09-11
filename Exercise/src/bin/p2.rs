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


fn main() {
    //let mut opt = Opt { infile: PathBuf::from("contacts_test.csv"), outfile: PathBuf::from("contacts_finish.csv"), };
    //opt.infile = PathBuf::from("contacts_test.csv");
    let mut contacts: HashMap<i32, help_fn::Contact> = HashMap::new();
    let mut number:i32 = 1;
    let mut id:i32 = 501;
    let opts = help_fn::Opt::from_args();
    match help_fn::read_file(opts ,&mut contacts) {
        Ok(o) => println!(" {:?}", o),
        Err(e) => println!("{:?}", e),
    }
    while number != 6 {
        id = contacts.iter().max_by_key(|x| x.0.abs()).unwrap().0.clone();
        dbg!(id);
        help_fn::menu();
        number = help_fn::input().parse::<i32>().expect("not a number");
        match number {
            1 => help_fn::add_contacts(&mut contacts, id),
            2 => help_fn::show_contacts(&mut contacts),
            3 => help_fn::remove_contact(&mut contacts),
            4 => help_fn::edit_contact(&mut contacts),
            5 => help_fn::search_contacts(&mut contacts),
            6 => number = 6,
            _ => println!("invalid selection"),
        }
    }

}
