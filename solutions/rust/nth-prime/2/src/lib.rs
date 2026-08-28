pub fn nth(n: u32) -> u32 {
    let mut prime = 2;
    let mut i = 0;

    loop {
        if is_prime(prime) {
            if i == n {
                return prime;
            }

            i += 1;
        }

        prime += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
