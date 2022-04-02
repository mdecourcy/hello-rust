// Arrays - fixed list where elems are the same data type

use std::mem;

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re assign
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // get array len
    println!("Array length {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("Array occupies {} bytes", mem::size_of_val(&slice));
}