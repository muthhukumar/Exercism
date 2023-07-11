fn is_prime(n: u64) -> bool {
    if n <= 2 {
        return true;
    }

    let end = (n as f64).sqrt() as u64 + 1;

    for i in 2..=end {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn factors(n: u64) -> Vec<u64> {
    let end = (n as f64).sqrt() as u64 + 1;

    let mut result: Vec<u64> = Vec::new();

    let mut num: u64 = n;

    let range = (2..=end).into_iter();

    range.fold(&mut result, |acc, i| {
        if is_prime(i) {
            while num % i == 0 {
                acc.push(i);
                num = num / i;
            }
        }

        acc
    });

    if num > 1 {
        result.push(num);
    }

    result
}
