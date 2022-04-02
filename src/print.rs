pub fn run(){
    // print to console
    println!("hello from the print.rs file");

    // Basic formatting 
    println!("{} favorite number is {}", "Mac's", 13);

    // Positional args
    println!(
        "{0} is from {1} and {0} likes to {2}", 
        "Mac", "Seattle", "code"
    );

    // named args
    println!(
        "{name} likes to {activity}",
        name = "Mac",
        activity = "Snowboard"
    );

    // Placeholder traits
    println!(
        "Binary: {:b} Hex: {:x} Octal: {:o}", 
        10, 10, 10
    );

    // Placeholder for debug trait 
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);

    
}