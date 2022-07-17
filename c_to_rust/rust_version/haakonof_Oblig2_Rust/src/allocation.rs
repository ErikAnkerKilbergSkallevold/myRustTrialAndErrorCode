use std::fs;

const NUM_BLOCKS: usize = 50;
const FILE_NAME: &str = "block_allocation_table";

fn read_table() -> String {
    let table = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    table
}

fn write_table(table: String) {
    fs::write(FILE_NAME, table)
        .expect("Something went wrong writing the file");
}

pub fn format_disk() {
    let table = String::new();
    let retval = write_table(table.clone());
}

pub fn allocate_block() {
    let table = read_table();
    for c in table.chars() {
        if c == '0' {
            let mut new_table = table.clone();
            new_table.push_str("1");
            write_table(new_table);
            return;
        }
    }
}