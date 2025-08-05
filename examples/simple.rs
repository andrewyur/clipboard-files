use clipboard_files::{read, write};
use std::{fs::canonicalize, path::PathBuf};

fn main() {
    let file_path = canonicalize(file!()).unwrap();
    let path_buf_vec = vec![PathBuf::from(file_path)];
    
    println!("{:?}", read()); 
    println!("{:?}", write(&path_buf_vec));
    println!("{:?}", read()); 
}
