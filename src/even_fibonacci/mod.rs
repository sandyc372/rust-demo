pub fn runner() {
    let limit = 999_399_999_999;
    println!("even fibonacci sum <= {} is {}", limit, fibonacci_sum(limit));
}

fn fibonacci_sum(limit: u64) -> u64 {
    let mut a = 1;
    let mut b = 2;
    let mut c = a + b;
    let mut sum = 2;
    while c <= limit {
        a = b;
        b = c;
        sum = sum + if c % 2 == 0 {c} else {0};
        c = a + b;
    }
    return sum;
}