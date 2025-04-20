mod trail;

fn main() {
    let limits = [1.0, 3.0, 5.0, 10.0];
    for limit in limits {
        println!("Largest Prime in {limit} sec: {}", trail::trial_div(limit));
    }
}
