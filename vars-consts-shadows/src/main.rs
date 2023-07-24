fn main() {
    let x = 4; // x is implictly typed]
    let y:u32 = 5; // y is u32 typed
    println!("x is {}", x);
    println!("y is {}", y);

    let mut z = 6; // z is mutable
    println!("z is {}", z);
    z = 7;
    println!("z is now {}", z);
    {
        let x = 2; // define x in a new scope so it doesnt get changed in parent scope
        println!("x is recreated to {}", x);

        let y = y - 2;
        println!("y is now .. {} in the inner scope", y)
    }
    let x = 8; // recreate x 
    println!("x is recreated to {}", x);
    let x = x + 1; // recreate x 
    println!("x is recreated to {}", x);

    let x = "hello";
    println!("x is recreated to {}", x);

    const HELP_ME: &str = "help me";
    println!("HELP_ME is {}", HELP_ME);
}
