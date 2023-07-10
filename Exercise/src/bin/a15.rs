// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket_type {
    Vip(f64, String),
    Backstage(f64, String),
    Standard(String),
}

fn main() {
    let events: Vec<Ticket_type> = vec![
        Ticket_type::Backstage(100.00, "Sam Rider".to_owned()),
        Ticket_type::Standard("Tim Klsein".to_owned()),
        Ticket_type::Vip(88.88, "Helli Toom".to_owned()),
    ];
    for event in events {
        match event {
            Ticket_type::Vip(price, name) => println!("{:?}, {:?}", name, price),
            Ticket_type::Backstage(price, name) => println!("{:?}, {:?}", name, price),
            Ticket_type::Standard(name) => println!("{:?} ", name)
        }
    }
}
