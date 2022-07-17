use std::io;
use crossterm::event::{read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    println!("This program will calculate the factorial of a number");
    println!("A factorial is a function that multiplies a number by every number below it. For example 5!= 5*4*3*2*1=120\n");
    let n = input();
    let result = calculate_factorial(n);
    print!("\nThe factorial of {} is {}\n", n, result);
    exit();
}

fn input() -> u64 {
    println!("Please enter a positive number");
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    match n.trim().parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input");
            input()
        },
    }
}

fn calculate_factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * calculate_factorial(n - 1)
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
