use std::io;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::LineWriter;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;
use itertools::Itertools;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    pub infile: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    pub outfile: PathBuf,
}

fn print_type_of<T>(_:&T) {
    println!("Type: {:?}", std::any::type_name::<T>())
}


pub fn write_file(opts: Opt ,result: &mut HashMap<i32, Contact>) -> std::io::Result<()> {
    let file = File::create(opts.outfile)?;
    let mut file_write = LineWriter::new(file);


}

pub fn read_file(opts: Opt,result: &mut HashMap<i32, Contact>) -> std::io::Result<()> {
    let mut file = File::open(opts.infile)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line)?;
        match len {
            0 => break,
            _ => {
                let data: Vec<&str> = line.split(",").collect();
                let id: i32; 
                let mut name = String::new();
                let mut email = String::new();
                if data[0].ends_with('\n') {continue}
                match data.len() {
                    3 => {
                        match parse_num(data[0].trim()){
                            Some(o) => id = o,
                            None => continue,
                        }
                        name = data[1].to_string();
                        email = data[2].to_string();
                        email.pop();
                        let r = Contact{
                            name: name,
                            email: email,
                        };
                        result.insert(id, r);
                        //print!("{:?}", id);
                        //print!("{:?}", name);
                        //print!("{:?}", email);
                    }
                    _ => print!(" Len: {:?} ", data.len()),

                }
                line.clear();
            }
        }
    }
    Ok(())

}


#[derive(Debug)]
pub struct Contact {
    pub name: String,
    pub email: String,
}


pub fn input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("faild to read");
    return user_input.trim().to_owned();
}

pub fn input_num() -> Option<i32>  {
    match input().parse::<i32>() {
        Ok(num) => Some(num),
        Err(e) => None,
    }
}

fn parse_num(input: &str) -> Option<i32>  {
    match input.parse::<i32>() {
        Ok(num) => Some(num),
        Err(e) => None,
    }
}

pub fn menu() {
    println!("1) add a contact");
    println!("2) show all contacts");
    println!("3) remove a contact");
    println!("4) edit existing contacts");
    println!("5) Search with Name");
    println!("6) quit");
    println!("type your selection");
}
pub fn show_contacts(results: &mut HashMap<i32, Contact>) {
    println!("contacts");
    for (id, val) in results.iter().sorted_by_key(|x| x.0)  {
        print!("{:?}",id);
        print!("{:?}", val.name);
        print!("{:?}", val.email);
        println!();
    }
}

pub fn add_contacts(results: &mut HashMap<i32, Contact>, last_id: i32) {
    println!("Type Name: ");
    let name = input();
    println!("Type Email ");
    let email = input();
    let r = Contact{
        name,
        email,
    };
    results.insert(last_id + 1, r);
}

pub fn search_contacts(results: &mut HashMap<i32, Contact>) {
    println!("Search for a Name: ");
    let name = input();
    for (key, val) in results.iter() {
        if name == val.name {
            println!("Result: {} , {}, {}", key, val.name, val.email );
        }
    }
    //match results.iter().find_map(|(key, val)| if val.name == name { Some(()) } else { None } ) {
        //Some(s) => println!(" Result: {:?}",s),
        //None => println!("No Result"),
    //}
}

pub fn remove_contact(results: &mut HashMap<i32, Contact>) {
    println!("Type the id of the Contact to remove: ");
    let id: i32;
    match input_num(){
        Some(s) => id = s,
        None => return,
    }
    if results.contains_key(&id) {
        println!("Enrry reboed {:?}", results.remove_entry(&id));
    }
}

pub fn edit_contact(results: &mut HashMap<i32, Contact>) {
    println!("Type the id of the Contact to edit: ");
    let id: i32;
    match input_num(){
        Some(s) => id = s,
        None => return,
    }
    println!("Type Name: ");
    let name = input();
    println!("Type Email ");
    let email = input();
    let r = Contact{
        name,
        email,
    };
    if results.contains_key(&id) {
        results.insert(id, r);
    }

}


