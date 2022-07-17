use std::io;

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10; // 15

    // subtraction
    let difference = 95.5 - 4.3; // 91.2

    // multiplication
    let product = 4 * 30; // 4 * 30 = 120

    // division
    let quotient = 56.7 / 32.2; // 56.7 / 32.2 = 1.8
    let floored = 2 / 3; // Results in 0 

    // remainder
    let remainder = 43 % 5; // Results in 3


    // Shadowing bools
    let t = true;
    let f: bool = false; // with explicit type annotation

    //chars
    let c = 'z'; // char
    let z = 'ℤ'; // 4 byte char
    let heart_eyed_cat = '😻'; // unicode character

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple with 3 elements
    let (tup1, tup2, tup3) = tup; // destructuring tuples
    let five_hundred = tup.0; // destructuring tuples
    let six_point_four = tup.1; // destructuring tuples
    let one = tup.2; // destructuring tuples

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 5 elements of type i32
    let first = a[0]; // first element of array a
    let second = a[1]; // second element of array a
    let a_repeated = [3; 5]; // [3, 3, 3, 3, 3]
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"]; // 12 elements of type &str 

    //Print all the values to the console
    println!("x is: {}", x);
    println!("y is: {}", y);
    println!("sum is: {}", sum);
    println!("difference is: {}", difference);
    println!("product is: {}", product);
    println!("quotient is: {}", quotient);
    println!("floored is: {}", floored);
    println!("remainder is: {}", remainder);
    println!("true is: {}", t);
    println!("false is: {}", f);
    println!("z is: {}", c);
    println!("Z is: {}", z);
    println!("heart_eyed_cat is: {}", heart_eyed_cat);
    println!("tup is: {:?}", tup);
    println!("tup1 is: {}", tup1);
    println!("tup2 is: {}", tup2);
    println!("tup3 is: {}", tup3);
    println!("five_hundred is: {}", five_hundred);
    println!("six_point_four is: {}", six_point_four);
    println!("one is: {}", one);
    println!("first is: {}", first);
    println!("second is: {}", second);
    println!("a_repeated is: {:?}", a_repeated);
    println!("months is: {:?}", months);


    // Input
    println!("");
    println!("a is: {:?}", a);
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    )
}
