// This  is a slightly modified version respect to the one in the course
// To make it more interesting I made it to loop indefinitely
// So that it only stops when saying "Good Bye"

use std::io;

fn main() {
    let mut name = String::new();
    loop {
        println!("Please enter a greeting:");
        name.clear();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        match name.trim() {
            "Good Bye" => {
                println!("Sorry to see you go.");
                break;
            }
            "Hello" => {
                println!("Hi, nice to meet you!");
                continue;
            }
            _ => {
                println!("I can't find a greeting. Greet well");
                continue;
            }
        }
    }
}
