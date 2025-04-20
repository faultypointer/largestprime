use std::time::Instant;

pub fn trial_div(time_limit: f64) -> u128 {
    let mut largest_prime_till_now = 3;
    let mut current_num = 3;
    let timer = Instant::now();
    loop {
        if timer.elapsed().as_secs_f64() > time_limit {
            break;
        }
        current_num += 2;
        largest_prime_till_now = if is_prime(current_num) {
            current_num
        } else {
            largest_prime_till_now
        };
    }
    largest_prime_till_now
}

fn is_prime(num: u128) -> bool {
    let sq_rt = num.isqrt();
    let mut divisor = 3;
    while divisor <= sq_rt {
        if num % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}
