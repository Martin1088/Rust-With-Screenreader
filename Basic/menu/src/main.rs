use std::io; 

#[derive(Debug)]
enum MenuChoice{
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("no choice like this found".to_owned())
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice: {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn menu() {
    println!("1) MainMenu");
    println!("2) Start the Program");
    println!("3) Quit ");
    println!("Type a nummer to select! ");
}

fn input() -> i32 {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed on input");
    let number: i32 = user_input.trim().parse::<i32>().expect("invalid");
    return number;
}

fn main() {
    let mut num = 1;
    while num != 3 {
        menu();
        num = input();
        match num {
            1 => println!("{:?}", pick_choice("mainmenu")),
            2 => println!("{:?}", pick_choice("start")),
            3 => println!("{:?}", pick_choice("quit")),
            _ => println!("cannot be selected!")
        }
    }
    //let result = pick_choice("start");
    //println!("choice: {:?}", result);
    //let choice: Result<MenuChoice, _> = get_choice("quit");
    //match choice {
        //Ok(inner_choice) => print_choice(&inner_choice),
        //Err(e) => println!(" {:?}", e),
    //}
}
