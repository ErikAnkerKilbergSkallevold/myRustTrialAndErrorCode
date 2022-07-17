fn main() {

    let s1 = String::from("s1");
    let s2 = s1;

    println!("{}, was moved to s2!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let str = "hello";
    print!("{}", str);

    {
        let str = " world";
        println!("{}", str);
    }
    
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    {
        let s = String::from("hello"); // s is valid from this point forward
        println!("{}", s);
        // do stuff with s
    }                                   // this scope is now over, and is is no
                                        // longer valid
        
}
