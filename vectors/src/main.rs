fn main() {
    println!("Hello, world!");
 

    let v0: Vec<i32> = Vec::new();
    println!("v0 is {:?}", v0);

    let v1 = vec![1, 2, 3, 4, 5];
    let mut v = Vec::new();
    println!("v is {:?}", v);

    {
        let v = vec![1, 2, 3, 4];
        println!("v is another scope {:?}", v);
        // do stuff with v
    } // <- v goes out of scope and is freed here


    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v has been pushed and is now {:?}", v);

    let third: &i32 = &v1[2];
    println!("The third element is {}", third);

    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //let does_not_exist = &v1[100];
    //let does_not_exist = v1.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0].clone();

    v.push(6);

    println!("The first element is: {}", first); 

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i); //Iterates through the vector and prints each element
    }

    println!("{:?}", v);

    let mut v = vec![100, 32, 57]; 
    for i in &mut v {
        *i += 50; //Iterates through the vector and adds 50 to each element. Needs to be dereferenced to be mutable
    }

    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);






}
