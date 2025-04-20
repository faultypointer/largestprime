use std::time::Instant;
const MAX_N: usize = 1_000_000_000;
pub fn eratosthenes(time_limit: f64) -> usize {
    let timer = Instant::now();
    let mut primes = vec![true; MAX_N];
    for p in 2..primes.len() {
        if primes[p] {
            if timer.elapsed().as_secs_f64() > time_limit {
                return p;
            }
            for multiple in primes.iter_mut().skip(p * p).step_by(p) {
                *multiple = false;
            }
        }
    }
    for (i, p) in primes.iter().rev().enumerate() {
        if *p == true {
            return MAX_N - i - 1;
        }
    }
    return 2;
}
