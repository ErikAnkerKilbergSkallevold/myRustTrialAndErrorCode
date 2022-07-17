use std::io;

const FAHRENHEIT_TO_CELSIUS: f32 = 5.0 / 9.0;
const CELSIUS_TO_FAHRENHEIT: f32 = 9.0 / 5.0;

fn main() {
    println!("A Celsius/Fahrenheit converter");
    println!("Will you be converting from Celsius to Fahrenheit or vice versa?");

    loop{
        println!("Please enter 'C' if you're inputting Celsius or 'F' if you're inputting Fahrenheit. Q to exit.");

        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: &str = input.trim();
        let input: String = input.to_uppercase().to_string();
        let input: &str = input.as_str();

        if input == "Q" {
            println!("Exiting...");
            break;
        }

        println!("Please enter your temperature: ");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: f32 = number.trim().parse().expect("Please type a number!");

    
        if input == "C" {
            println!("{} degrees Celsius is {} degrees Fahrenheit", number, convert_celsius_to_fahrenheit(number));
        } else if input == "F" {
            println!("{} degrees Fahrenheit is {} degrees Celsius", number, convert_fahrenheit_to_celsius(number));
        } else {
            println!("Wrong type detected. Please enter 'C' if you're inputting Celsius or 'F' if you're inputting Fahrenheit. Q to exit.");
        }
    }
    
}

fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    celsius * CELSIUS_TO_FAHRENHEIT + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * FAHRENHEIT_TO_CELSIUS
}
    