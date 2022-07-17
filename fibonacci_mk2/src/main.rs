use std::{io, vec};

fn main() {
    println!("This program will give you the all the fibonacci numbers to n");
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
        
        fibonacci(input);
        println!("Type \"quit\" to end the program");
        println!("Enter the iteration number of the fibonacci number you want to see: ");
    }
}


fn fibonacci(n: u32) {
    let mut n0 = 0;
    let mut n1 = 1;
    let mut nth = 0;
    let mut fibo_array = vec![0, 1];

    match n {
        0 => println!("{}", n0),
        1 => println!("{}", n1),
        _ => {
            while nth <= n {
                nth = n0 + n1;
                fibo_array.push(nth);
                //Update values
                n0 = n1;
                n1 = nth;
            }
            fibo_array.pop();
            fibo_array.push(n);
            println!("{:?}", fibo_array);
        }
    }
}
