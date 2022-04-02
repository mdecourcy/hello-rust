pub fn run() {
    greeting("Hello", "Jane");
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    //closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str){
    println!("{}, {}, nice to meet you!", greet, name);
}

// we dont use a semi colon here because that value is what we want to return
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}