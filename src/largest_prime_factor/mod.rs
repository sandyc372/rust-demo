use math;

pub fn runner() {
    let a: [u64; 2] = [13195, 600851475143];

    for element in a.iter() {
        println!(
            "lpf of {} is {}",
            *element,
            calculate_lpf(*element)
        );
    }
}

pub fn calculate_lpf(n: u64) -> u64 {
    let root = (n as f64).sqrt();
    let root = math::round::ceil(root, 0) as u64;
    let mut counter = root;
    while counter >= 2 {
        if n % counter == 0 && is_prime(counter) == true {
            break;
        }
        counter -= 1;
    }
    return counter;
}

pub fn is_prime(n: u64) -> bool {
    let root = (n as f64).sqrt();
    let root = math::round::floor(root, 0) as u64;
    let mut counter = 2;
    let mut flag = true;
    while counter <= root {
        if n % counter == 0 {
            flag = false;
            break;
        }
        counter += 1;
    }
    return flag;
}
