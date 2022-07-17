#![allow(unused)]
use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> { //We can now use ? on let f = File::Open because Main returns a result as Int
    let f = File::open("world.txt"); 
    //let fe = f.as_ref().expect("Failed to open world.txt");
    //let fu = f.as_ref().unwrap();

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("world.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("{:?}", read_username_from_file().unwrap());
    println!("{:?}", better_read_username_from_file().expect("Couldn't read username"));

    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("world.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn better_read_username_from_file() -> Result<String, io::Error> {
    //let mut f = File::open("world.txt")?;
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?; //does the same, but in a shorter way
    //f.read_to_string(&mut s)?; 
    Ok(s)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


