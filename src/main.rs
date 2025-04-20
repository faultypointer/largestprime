#![allow(dead_code)]
mod sieve;
mod trail;

fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(_) = args.next() {
        println!("Largest Prime in 1 sec: {}", sieve::eratosthenes(1.0));
    } else {
        let limits = [1.0, 3.0, 5.0, 10.0];
        for limit in limits {
            println!(
                "Largest Prime in {limit} sec: {}",
                sieve::eratosthenes(limit)
            );
        }
    }
}
