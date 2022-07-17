#[derive(Debug)] // so we can inspect the state in a minute
//Enum usStates that contains the names of all 50 US states.
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("add fancy hat");
}
fn remove_fancy_hat() {
    println!("Removing fancy hat");
}
fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces", num_spaces);
}

fn reroll() {}

fn match_dice_roll(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //Does nothing, emtpy tuple
        //_ => reroll(), doesn't bind any value
        //other => moveplayer(other), takes all other values and passes them to move_player
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let dice_roll = 3;
    match_dice_roll(dice_roll);
    let dice_roll = 7;
    match_dice_roll(dice_roll);
    let dice_roll = 9;
    match_dice_roll(dice_roll);
    
}
