pub fn is_armstrong_number(num: u32) -> bool {
    let n = num.to_string().len() as u32;
    let mut sum = 0;
    let mut current = num;
    while current > 0 {
        let digit = current % 10;
        sum += digit.pow(n);
        current /= 10;
    }
    return num == sum;
}
