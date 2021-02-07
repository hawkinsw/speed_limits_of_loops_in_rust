use std::time::SystemTime;
use volatile::Volatile;

fn main() {
    let x = 500000000u64;
    let mut i = 0u64;
    let mut j = 0;
    let mut mutable_j = Volatile::new(&mut j);

    let iter_before = SystemTime::now();
    for _ in 0..x {
        mutable_j.write(mutable_j.read() + 1);
        i += 1;
    }
    let iter_after = SystemTime::now();
    let iter_total_time = iter_after.duration_since(iter_before).expect("Ok").as_micros();

    i = 0;
    mutable_j.write(0);

    let while_before = SystemTime::now();
    while i < x {
        mutable_j.write(mutable_j.read() + 1);
        i += 1;
    }
    let while_after = SystemTime::now();
    let while_total_time = while_after.duration_since(while_before).expect("Ok").as_micros();

    println!("While: {}, Iter: {}", while_total_time, iter_total_time);
}
