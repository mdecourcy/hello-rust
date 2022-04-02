// Variales hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name = "Mac";
    let mut age = 26; // as age is a changing variable, we use "mut" to signal that it is mutable and can change

    println!("My name is {} and I am {}", name, age);

    age = 27;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    // When defining consts we must explicitly define a type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars at once

    let (my_name, my_age) = ("Mac", 26);
    println!("{} is {}", my_name, my_age);

}