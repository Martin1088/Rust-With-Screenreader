#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug)]

struct Employee {
    position: Position,
    working_hours: i32,
}

fn main() {
    let me = Employee {
        position: Position::Supervisor,
        working_hours: 60,
    };
    println!("{:?}", me);
}
