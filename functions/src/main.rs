fn main() {
    println!("Hello, world!");
    test_one();
    println!("{}", add(1, 2));


    // expression
    let number = {
        let x = 1;
        x + 1 // this is being returned
    };
    println!("Number: {}", number);

    println!("{}", subtract(7, 2));
}

fn test_one() {
    println!("test one");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}


fn subtract(a: i32, b: i32) -> i32 {
    let result = a - b;
    if result < 0 {
        return 0;
    }
    result
}
