use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line"); 
    // without the &mut it would just send a copy and not the actual value
    // expect is a way to handle errors
    println!("You typed: {}", input);
}
