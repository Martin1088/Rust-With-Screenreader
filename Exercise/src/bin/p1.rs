// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.
use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

impl Bill{
    fn new(name: &str, amount: &f64) -> Bill {
        Bill { name: name.to_string(), amount: amount.clone()}
    }
}

fn input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("faild to read");
    return user_input.trim().to_owned();
}

fn menu() {
    println!("1) add a bill");
    println!("2) show all bills");
    println!("3) remove abill");
    println!("4) edit existing bill");
    println!("5) quit");
    println!("type your selection");
}

fn add_bill() -> Bill {
    println!("Type the name of your bill");
    let name: String = input();
    println!("Type the amount with . :");
    let amount: f64 = input().parse::<f64>().expect("not a valid number");
    return Bill::new(&name, &amount)
}

fn show_bills(result: &HashMap<i32, Bill>) {
    for (key, val) in result {
        print!("{:?}  ", key);
        print!("bill: {:?} {:?} Euro", val.name, val.amount);
        println!();
    }
}

fn print_type_of<T>(_:&T) {
    println!("Type: {:?}", std::any::type_name::<T>())
}

fn remove_bill(result: &mut HashMap<i32, Bill>) {
    println!("Type the is to remove the Entry");
    let id: i32 = input().parse::<i32>().expect("not a Number");
    if id == 0{
        return;
    }
    println!("is a Entry:  {:?}", result.contains_key(&id));
    println!("Entry:{:?} ", result.remove_entry(&id));
}

fn main() {
    let mut bills: HashMap<i32, Bill> = HashMap::new();
    let mut number:i32 = 1;
    let mut id:i32 = 1;
    while number != 5 {
        menu();
        number = input().parse::<i32>().expect("not a number");
        match number {
            1 => {
                let result = add_bill();
                print_type_of(&result);
                bills.insert(id, result);
            },
            2 => {
                print_type_of(&bills);
                show_bills(&bills);
            },
            3 => remove_bill(&mut bills),
            4 => {
                println!("Type the is to edot the Entry");
                let id: i32 = input().parse::<i32>().expect("not aNumber");
                if id == 0{
                    return;
                }
                bills.insert(id, add_bill());

            } 
            5 => number = 5,
            _ => println!("not a valid selection "),
        };
    id += 1;
    }
}
