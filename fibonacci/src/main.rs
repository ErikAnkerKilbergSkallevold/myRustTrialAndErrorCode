use std::io;

fn main() {
    println!("This program will give you the nth fibonacci number");
    println!("Type \"quit\" to end the program");
    println!("Enter the iteration number of the fibonacci number you want to see: ");
    loop{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        if input.trim() == "quit" {
            break;
        }

        let input :u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("The {} fibonacci number is {}", input, fibonacci(input));
        println!("Type \"quit\" to end the program");
        println!("Enter the iteration number of the fibonacci number you want to see: ");
    }
    


}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}