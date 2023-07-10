// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let number: Vec<i32> = vec![10, 20, 30, 40];
    println!("lenght: {}", number.len());
    for n in &number {
        if *n == 30 { // as i32
            print!(" thirty ");
            continue;
        }
        print!(" {}", n);
    }
    println!();
    for num in &number {
         match num {
            10 => print!(" ten "),
            _ => print!("{:?}",num ),
        }
    println!();
    }
   }
