pub fn factors(n: u64) -> Vec<u64> {
    let mut numbers = Vec::new();
    let mut divisor = 2;
    let mut number = n;
    while divisor * divisor <= n {
        while number % divisor == 0 {
            numbers.push(divisor);
            number /= divisor;
        }
        divisor += 1;
    }
    numbers
}
