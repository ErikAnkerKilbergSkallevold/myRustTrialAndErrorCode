use std::io;
use crossterm::event::{read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const NOK_TO_EUR: f64 = 0.095;
const EUR_TO_NOK: f64 = 10.51;

fn main() {
    println!("Welcome to the currency converter!");
    let n = input();
    let c = choose_currency();
    println!("You have chosen to convert {} {}", n, c);
    let result = currency_converter(n, c);
    println!("{} {} is {} {}", n, c, result.0, result.1);
    exit();
}

fn input() -> f64 {
    println!("Enter the amount you want to convert:\n");  
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    match n.trim().parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a floating point number");
            input()
        }
    }
}

fn choose_currency () -> &'static str {
    println!("Choose from the following options (type number to begin):");
    println!("(1) NOK to EUR");
    println!("(2) EUR to NOK");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");

    let num = match input_text.trim().parse::<u64>() {
        Ok(num) => {
            if num == 1 {
               return "NOK";
            } else if num == 2 {
                return "EUR";
            } else {
                println!("Please choose an option from the list");
                choose_currency()
            }
        },
        Err(_) => {
            println!("Please choose an option from the list");
            choose_currency()
        }
    };
    num
}

fn currency_converter(amount: f64, choice: &str) -> (f64, &str) {
    match choice {
        "NOK" => (amount * NOK_TO_EUR, "EUR"),
        "EUR" => (amount * EUR_TO_NOK, "NOK"),
        _ => (amount, "Failed to convert!"),
    }
}

fn exit() {
    println!("\n\nProgram is finished, press any key to exit");

    //going into raw mode
    enable_raw_mode().unwrap();
    //key detection
    loop {
        //matching the key
        match read().unwrap() {
            _ => break,
        }
    }
    //disabling raw mode
    disable_raw_mode().unwrap();
}


