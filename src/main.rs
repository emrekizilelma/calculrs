use calculrs;
use std::io;
fn main() {
    
    calculrs::calculrs_menu();

    let mut op = String::new();

    println!("Enter your option: ");

    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read line");

    
}
