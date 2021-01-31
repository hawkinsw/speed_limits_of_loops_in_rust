use std::time::SystemTime;

fn some_function_to_time() {
    // The contents of the function that we want to time.
}

fn main() {
    let x = 500000000u64;
    let mut i = 0u64;

    let before = SystemTime::now();

    while i < x {
        some_function_to_time();
        i += 1;
    }

    let after = SystemTime::now();
    let average = after.duration_since(before).expect("Ok").as_secs();

    println!("Time between: {}", average);
}
