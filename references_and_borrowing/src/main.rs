fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    change(&mut s);

    println!("The length of '{}' is {}.", s1, len);
    println!("s is '{}'", s);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");
    s
}