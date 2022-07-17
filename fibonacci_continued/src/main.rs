use std::io;

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
    let mut count = 0;
    let mut n0 = 0;
    let mut n1 = 1;

    match n {
        0 => println!("{}", n0),
        1 => println!("{}", n1),
        _ => {
            while count <= n {
                println!("{}", n0);
                let nth = n0 + n1;
                //Update values
                n0 = n1;
                n1 = nth;
                count += 1;
            }
        }
    }
}
