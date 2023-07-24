fn main() {

    // Scalar Types
    let x: i32 = 4; // int 32 data type
    // all bytes for ints i8 , i16, i32, i64, i128
    // u8, u16, u32, u64, u128 - unsigned integers
    let y: u32 = 5; // u32 is unsigned so cant be negitive number
    // u8 range is  0 to 2^8 - 1 (255)
    // i8 range is -2^7 to 2^7 - 1 (127)

    // flaots - f32, f64
    let z: f32 = 6.9;
    
    // bool - true or false;
    let w: bool = true;

    // char - single char
    let c: char = 'z'; // must use single quotes


    // Compound Types
    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let tup2: (char, bool, u64) = ('a', false, 1); // tuple

    println!("The value of tup is: {}", tup.0); // cannot print whole tuple in one


    // arrays
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // array
    println!("The value of array is: {}", array[0]);
}
