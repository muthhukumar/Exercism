pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    let end = (n as f64).sqrt() as u32 + 1;

    for i in 2..end {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut i = 0;
    let mut num = 0;
    let mut prime_no = 0;

    while i <= n {
        if is_prime(num) {
            i += 1;
            prime_no = num;
        }
        num += 1;
    }

    prime_no
}
