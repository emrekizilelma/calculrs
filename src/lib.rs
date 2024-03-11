pub fn calculrs_menu() {
    print!("-----------------------------------------\n");
    println!("*Welcome the Cli Calculator Program!*\n");
    println!("1)Add\n2)Substract\n3)Multiply\n4)Divide\n5)Exit (q)\n");
    print!("-----------------------------------------\n")
}

pub fn add(num1: i8, num2: i8) -> i8 {
    num1 + num2
}

pub fn substract(num1: i8, num2: i8) -> i8 {
    num1 - num2
}

pub fn multiply(num1: i8, num2: i8) -> i8 {
    num1 * num2
}

pub fn divide(num1: i8, num2: i8) -> i8 {
    num1 / num2
}