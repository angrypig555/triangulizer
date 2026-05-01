use std::io;
use std::io::Write;

use triangulizer::calc_hypotenuse;

fn readln() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Entered string incorrect");

    input.trim().to_string()
}


fn main() {
    println!("Triangulizer");
    println!("Please choose option:\n1. Calculate hypotenuse\n");
    let choice = readln();
    if choice == "1" {
        print!("Please enter side 1: ");
        let a: f64 = readln().parse().expect("Please enter a number");
        print!("Please enter side 2: ");
        let b: f64 = readln().parse().expect("Please enter a number");
        let hypotenuse = calc_hypotenuse(a, b);
        println!("Your hypotenuse is: {}", hypotenuse);
        println!("Brought to you by the pythagorean theorem");
    }
}
