fn main() {
    // operators >, <, >=, <=, ==, !=
    let cond = 2 as f32 <= 2.2;
    println!("{}", cond);

    // compound operators &&, ||, !
    let cond2 = !(true && cond);
    println!("{}", cond2);

    // logical operators should be done in this order: ! && ||
    // unless using brackets

    let food = "banana";
    if food == "apple" {
        println!("food is apple");  
    } else if food == "banana" {
        println!("food is banana");
    } else {
        println!("food is not apple or banana");
    }
}
