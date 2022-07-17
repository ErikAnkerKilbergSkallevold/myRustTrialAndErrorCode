use std::{cmp::Ordering, io::Result as IoResult, collections::*, fmt::Result as FmtResult};

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(1, 2);
}


fn function1() -> FmtResult {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}