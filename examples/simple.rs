use clipboard_files::{read, write};
use std::path::PathBuf;

fn main() {
    let relative_path = PathBuf::from(file!());
    let absolute_path = std::fs::canonicalize(relative_path.clone()).unwrap();

    println!("{:?}", read()); 
    println!("{:?}", write(vec![relative_path, absolute_path]));
    println!("{:?}", read()); 
}
