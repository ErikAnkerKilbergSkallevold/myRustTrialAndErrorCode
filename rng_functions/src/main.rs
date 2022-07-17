fn main() {
    println!("This program will choose and run 3 different functions at random. It runs 1000 times");
    
    for _i in 0..1000 {
        let rng = fastrand::i32(1..4);
        match rng {
            1 => func1(),
            2 => func2(),
            3 => func3(),
            _ => println!("Oh fuggg wha happen????")
        };
    }
}

fn func1() {
    println!("This is function 1");
}

fn func2() {
    println!("This is function 2");
}

fn func3() {
    println!("This is function 3");
}

