# rustic-lifestyle

My first RUST project. 

##  Useful commands

- `rustc <main.rs>` - compiles the rust code for you. need to run `./<main.rs>` after to run the code
- `cargo new <project_name>` - creates a cargo hello world project for you
- `cargo build` - builds the rust folder for you. can run by either `cd target/debuild` then `./main.rs` or simply `cargo run`
- `cargo check` - will check if code will run so you don't have to wait for compile beforehand
- `rustfmt <main.rs>` - automatically formats code for you
ap
## Interesting facts
 - main.rs must have a main function called main
 - changine var in new scope doesn't change it in the parent scope
 - you can redefine types as strings
 - consts should be CAPITAL_SNAKE_CASE
 - can use parent scope vars to edit child scope vars
 - &mut give you a mutable reference so you are allowed to change the value
 - Can't add types that aren't the EXACT same
 - Math on integers like division only results in whole numbers and wont return float
 - Using `as` in type casting calculations can result in uncaught overflow. So always CONVERT SMALLER VALUE TO THE LARGER
 - function ordering doesnt matter
 - If you don't return from a function you will just get empty brackets