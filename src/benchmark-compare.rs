use std::time::SystemTime;

fn some_function_to_time() {
    // The contents of the function that we want to time.
}

fn main() {
    let x = 500000000u64;
    let mut i = 0u64;

    let while_before = SystemTime::now();

    while i < x {
        some_function_to_time();
        i += 1;
    }

    let while_after = SystemTime::now();
    let while_total_time = while_after.duration_since(while_before).expect("Ok").as_secs();

    i = 0;

    let iter_before = SystemTime::now();
    for _ in 0..x {
        some_function_to_time();
        i += 1;
    }
    let iter_after = SystemTime::now();
    let iter_total_time = iter_after.duration_since(iter_before).expect("Ok").as_secs();

    println!("While: {}, Iter: {}", while_total_time, iter_total_time);
}
