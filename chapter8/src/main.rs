use std::{vec, collections::HashMap};
fn main() {
    let mut vec: Vec<i32> = vec![-10, 20, 34, 55, 89, 12, 34, 56, 78, 90, 34, 12, 99, 130];
    vec.sort();

    match vec.get(vec.len() / 2) {
        Some(median) => println!("Median: {}", median),
        None => println!("Median: None"),
    }

    let mut map = HashMap::new();
     for int in vec.iter() {
        let count = map.entry(int).or_insert(0);
        *count += 1;
     }
    let map_max_value = map.iter().max_by(|&(_, v1), &(_, v2)| v1.cmp(v2));
    println!("Mode: Int '{:?}' was found '{:?}' times.", map_max_value.unwrap().0, map_max_value.unwrap().1);

    pig_latin("The quick brown fox jumps over the lazy dog");
    let mut company = HashMap::new();
    company.insert("Erik Anker Kilberg Skallevold".to_string(), "IT".to_string());
    //get user input in string
    loop {
        let mut input = String::new();
        println!("Use commands to add persons to positions in company");
        println!("Use command 'list' to list all persons in company");
        println!("Use command 'remove' to remove person from company");
        println!("Use command 'add' to add person to company");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            "list" => {
                for (key, value) in company.iter() {
                    println!("{} is at {}", key, value);
                }
            }
            "add" => {
                let add = add_person();
                company.insert(add.0, add.1);
                println!("{:?}", company);
            }
            "remove" => {
                company.remove(remove_person().as_str());
            }
            _ => println!("Unknown command"),
        } 
    }
    
        
}

fn pig_latin(s: &str){
    let s = s.to_lowercase();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    for word in s.split_whitespace() {
        let mut new_word = String::new();
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();
        if vowels.contains(&first_char) { //Vowels
            new_word.push_str(word);
            new_word.push_str("-");
            new_word.push_str("hay");
        } else { //consonants
            let mut ord: String = word.to_string();
            ord.remove(0);
            new_word.push_str(&format!("{}", ord));
            new_word.push_str("-");
            new_word.push(first_char);
            new_word.push_str("ay");
        }
        println!("{}", new_word);
    }    
}

fn add_person() -> (String, String) {
    let position = vec!["Engineering", "Sales", "Marketing", "HR", "IT"];
    println!("Write person name");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("Write person position");
    println!("Available positions: Engineering, Sales, Marketing, HR, IT");
    let mut pos_inp = String::new();
    std::io::stdin().read_line(&mut pos_inp).expect("Failed to read line");
    let pos_inp = pos_inp.trim();
    if !position.contains(&pos_inp) {
        println!("Unknown position. Try again");
        add_person();
    }
    println!("{} is at {}. Is this correct? (y/n)", name, pos_inp);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input != "y" {
        println!("Person not added");
    }
    (name.to_string(), pos_inp.to_string())
}

fn remove_person() -> String {
    let position = vec!["Engineering", "Sales", "Marketing", "HR", "IT"];
    println!("Write person name");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();
    println!("Write person position");
    println!("Available positions: Engineering, Sales, Marketing, HR, IT");
    let mut pos_inp = String::new();
    std::io::stdin().read_line(&mut pos_inp).expect("Failed to read line");
    let pos_inp = pos_inp.trim();
    if !position.contains(&pos_inp) {
        println!("Unknown position. Try again");
        remove_person();
    }
    println!("{} is at {}. Is this correct? (y/n)", name, pos_inp);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    if input != "y" {
        println!("Person not removed");
    }
    name.to_string()
}

