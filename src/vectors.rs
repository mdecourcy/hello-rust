// Vectors - resizable arrays
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // re assign
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // get vector len
    println!("vector length {}", numbers.len());

    // vectors are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("vector occupies {} bytes", mem::size_of_val(&slice));
}