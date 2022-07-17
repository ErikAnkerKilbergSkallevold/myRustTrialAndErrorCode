use std::io;

fn main() {
    let s = input_text();
    let c = count_chars(s);
    println!("Vowel count: {}", c.0);
    println!("Consonant count: {}", c.1);
}

fn input_text() -> String {
    println!("Enter some alphabetic text:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim().to_lowercase();
    match input.len() {
        0 => {
            println!("Enter some text:");
            input_text()
            },
        _ => input,
    }     
}

fn count_chars(s: String) -> (i32, i32) {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut vowel_count = 0;
    let mut consonant_count = 0;
    for c in s.chars() {
        if c.is_alphabetic() {
            if vowels.contains(&c) {
                vowel_count += 1;
            } else {
                consonant_count += 1;
            }
        }
    }
    (vowel_count, consonant_count)
}
