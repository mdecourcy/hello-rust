/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in mem)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays - fixed length
*/

// Rust is a statically typed language, which means that it ust know the types of 
// all varibles at compile time, however, the compiler can usually infer what type we
// want based on the value and how we use it.

pub fn run(){

    // default is "i32"
    let x = 1; // underscore is because this is an intentionally unused variable

    // default is "f64"
    let y = 2.5;

    // add explicit type
    let z: i64 = 4545455444232;

    // find max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);
    println!("Max f32 {}", std::f32::MAX);
    println!("Max f64 {}", std::f64::MAX);

    // Boolean
    let is_active = true;

    // Boolean from expression
    let is_greater = 10 > 5;

    // Char
    let a1 = 'a';
    let emoji = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, emoji));



}