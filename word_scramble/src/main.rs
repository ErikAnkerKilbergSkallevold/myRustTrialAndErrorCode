use std::io;
fn main() {
    println!("This program takes a 4 word input, scrambles it, and shows all possible combinations!");
    let s = input();
    let p = permutations(s);
    println!("{:?}", p);
}

fn input() -> String {
    let mut s = String::new();
    println!("Please enter a 4 letter word");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim().to_string(); 

    if s.len() != 4  {
        println!("Please enter a 4 letter word");
        println!("The inputted string is {} chars long", s.len());
        input()
    } else {
        s
    }
}

fn permutations(word: String) -> Vec<String> {
    let length = word.len();

    if length <= 1 {
      // need return keyword for early return
      // otherwise error: expected () found String
      return vec![word];
    }
  
    // remove first character
    let trimmed = word.chars().skip(1).collect();
  
    // find all permutations of remaining letters
    let perms = permutations(trimmed);
    let current_char = word.chars().nth(0).unwrap();
    let mut result = Vec::new();
  
    // reinsert first letter in every possible place
    for perm in &perms {
      for i in 0..&perms.len() + 1 {
        let front: String = perm.chars().take(i).collect();
        let rest: String = perm.chars().skip(i).collect();
        result.push(format!("{}{}{}", front, current_char, rest));
      }
    }
  
    result.sort();
    result.dedup();
    result
}