use std::time::{Duration, SystemTime};

fn main() {
    let now = SystemTime::now();
    println!("{:?}", now);
    let later = now
        + Duration::from_secs(1_000_000_000); // add a gigasecond

    println!("{:?}", later);
}
