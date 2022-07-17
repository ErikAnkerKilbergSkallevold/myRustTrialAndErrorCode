//importing in execute! macro
extern crate crossterm;

use std::io;
use tabled::{Table, Tabled, Style, Modify, Format, object::Columns, Header};
use colored::*;
use crossterm::event::{read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Tabled)]
struct PyramidType {
    number: &'static str,
    name: &'static str,
    design: &'static str,
}

fn main() {
    println!("This program will type out a pyramid in the terminal");
    let n = choose_number();
    let c = choose_char();
    let p = choose_pyramid();

    match p {
        1 => print_pyramid(n, c),
        2 => print_pyramid_reversed(n, c),
        3 => print_pyramid_skip_even(n, c),
        _ => println!("Couldn't print pyramid. Please try again."),
    };

    exit();
}


//number input
fn choose_number() -> u64 {
    println!("Enter the height of the pyramid: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read numberline");
    
    let input: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number");
            choose_number()
        },
    };
    
    input
}

//char input
fn choose_char() -> char {
    println!("Enter the character to use for the pyramid");
    let mut char_input = String::new();
    io::stdin().read_line(&mut char_input).expect("Failed to read charline");

    match char_input.trim() {
        "" => char_input = {println!("No char selected, using '*'"); "*".to_string()},
        _ => char_input = char_input,
    };

    let char_input = char_input.trim().chars().nth(0).unwrap();
    char_input
}

fn choose_pyramid() -> i32 {
    let pyramid_types = vec![
        PyramidType {
            number: "1",
            name: "Regular",
            design: "*\n**\n***"
        },
        PyramidType {
            number: "2",
            name: "Reversed",
            design: "*****\n**\n*"
        },
        PyramidType {
            number: "3",
            name: "Regular Skip Even",
            design: "*\n***\n*****"
        },    
    ];

    let table = Table::new(&pyramid_types)
        .with(Header("Pyramid variants"))
        .with(Style::modern())
        .with(Modify::new(Columns::single(0)).with(Format::new(|s| s.red().to_string())))
        .with(Modify::new(Columns::single(1)).with(Format::new(|s| s.blue().to_string())))
        .with(Modify::new(Columns::new(2..)).with(Format::new(|s| s.green().to_string())));
    println!("{}", table);

    println!("Choose pyramid type. Write the corresponding number.\n1: Regular, 2: Reversed, 3: Regular Skip Even");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read pyramid type");
    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Please choose a number between 1 and 3"); choose_pyramid()}
    };
    n
}

fn print_pyramid(n: u64, c: char) {
    for i in 1..n + 1 {
        println!("{}", line(i, c));
      }  
}

fn print_pyramid_reversed(n: u64, c: char) {
    for i in 0..n {
        println!("{}", line(n - i, c));
      }
}

fn print_pyramid_skip_even(n: u64, c: char) {
    for i in 1..n + 1 {
        if i % 2 != 0 {
          println!("{}", line(i, c));
        }
      }
}


fn line(num: u64, c: char) -> String {
    let mut buf = String::with_capacity(num as usize);
  
    for _ in 0..num {
      buf.push(c);
    }
  
    buf
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

