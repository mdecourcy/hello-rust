pub fn run(){
    let mut count = 0;

    // infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 3{
    //         break;
    //     }

    // }

    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("Fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else { println!("Count: {}", count); }
        count += 1;
    }

    // for range

    for x in 0..5 {
        println!("Count: {}", x);
    }
}