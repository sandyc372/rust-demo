use math;

pub fn runner() {
    let a = [10, 20, 30, 37, 50, 7, 99, 1007];

    for element in a.iter() {
        println!(
            "{} is {}",
            *element,
            if is_prime(*element) == true {
                "prime"
            } else {
                "not prime"
            }
        );
    }
}

pub fn lpf(n: u32) -> u32 {
    
    1
}

pub fn is_prime(n: u32) -> bool {
    let root = (n as f64).sqrt();
    let root = math::round::floor(root, 0) as u32;
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
