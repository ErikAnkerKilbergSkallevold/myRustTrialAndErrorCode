use std::io;
fn main() {
    println!("This program will input your penis lenght and compare it to the National Penis Database");
    println!("Please input your Penis Size in CM");
    loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        if input.trim() == "quit" {
            break;
        }

        let input :f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, please try again!");
                continue
            },
        };
        
        penis_measure(input);
        println!("Type \"quit\" to end the program");
        println!("Enter your Penis Size to measure again: ");
    }
}

fn penis_measure(f: f64) {
    println!("Wow only {}cm! That's really small lol 😂🤣", f);
}

