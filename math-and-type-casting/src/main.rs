use std::io;

fn main() {
    // Type casting
    let x = 2.6f64; // 0 - 255
    let y = 10_i8; // -128 - 127 ... Can use underscores before type or not or as 
    let _w = 10.0 as f64;
    let z = x / (y as f64); // type casting to allow for calculations (dont need brackets)
    println!("z is {}", z);

    // Convett sting to number
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let int_input: i32 = input.trim().parse().unwrap(); 
    // trim removes whitespace
    // parse returns the resutlt
    // unwrap unwraps the result and converts to integer
    println!("input is {}", int_input);

}
