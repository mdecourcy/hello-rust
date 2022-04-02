// Primative str = Immutable fixed-length string somewhere in mem
// String = growable, heap allocated data structure - use when need to modify

pub fn run(){
    let mut hello = String::from("Hello ");

    //Get length
    println!("Length: {}", hello.len());

    println!("{}", hello);

    // pushes char
    hello.push('w');

    // pushes string
    hello.push_str("orld!");

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Contains
    println!("Contains 'world' {}", hello.contains("world"));

    // Replace
    println!("Replace world: {}", hello.replace("world", "There"));

    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertiion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}