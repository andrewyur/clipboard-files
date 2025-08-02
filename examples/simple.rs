use clipboard_files::{read, write};
use std::{fs::canonicalize, path::PathBuf};

fn main() {
    let file_path = canonicalize(file!()).unwrap();

    println!("{:?}", file_path);
    println!("{:?}", read()); 
    println!("{:?}", write(vec![PathBuf::from(file_path)]));
    println!("{:?}", read()); 
}
