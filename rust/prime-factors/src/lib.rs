pub fn factors(n: u64) -> Vec<u64> {
    let end = (n as f64).sqrt() as u64 + 1;

    let mut result: Vec<u64> = Vec::new();

    let mut num: u64 = n;

    let range = (2..=end).into_iter();

    range.fold(&mut result, |acc, i| {
        while num % i == 0 {
            acc.push(i);
            num = num / i;
        }

        acc
    });

    if num > 1 {
        result.push(num);
    }

    result
}
