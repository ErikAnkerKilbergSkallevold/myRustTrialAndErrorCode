mod allocation;
const BLOCK_SIZE: usize = 4096;

struct inode {
    id: i32,
    name: String,
    is_directory: bool,
    is_readonly: bool,
    filesize: i32,
    num_entries: i32,
    entries: Vec<size_t>,
}

type size_t = i32;

fn main() {
    println!("Hello, world!");
}
