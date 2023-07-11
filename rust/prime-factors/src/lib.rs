pub fn factors(n: u64) -> Vec<u64> {
    let end = (n as f64).sqrt() as u64 + 1;

    let mut result: Vec<u64> = Vec::new();

    let mut num: u64 = n;

    for i in 2..=end {
        while num % i == 0 {
            result.push(i);
            num /= i;
        }
    }

    if num > 1 {
        result.push(num);
    }

    result
}
