use arc_cache::cache::*;

use std::time::Duration;
use std::time::Instant;

fn main() {
    let ins = Instant::now();

    println!("Insatant: {} Duration: {}", ins, Duration::from_secs(700));
}
