pub fn nth(n: u32) -> u32 {
    let mut prime = 2;
    let mut i = 0;
    while i < n {
        if is_prime(prime) {
            i += 1;
        }
        prime += 1;
    }
    return prime;
}

fn is_prime(n: u32) -> bool {
    for i in 1..(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
