pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut divisor = 2;
    let mut remaining = n;

    while divisor * divisor <= remaining {
        while remaining % divisor == 0 {
            prime_factors.push(divisor);
            remaining /= divisor;
        }

        divisor += 1;
    }

    if remaining > 1 {
        prime_factors.push(remaining);
    }

    prime_factors
}
