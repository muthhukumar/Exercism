pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    };

    let mut digits: Vec<u32> = vec![];

    let len: u32 = {
        let mut len = 0;
        let mut number = num;

        while number != 0 {
            digits.push((number % 10).into());

            number /= 10;
            len += 1;
        }

        len
    };

    digits.iter().map(|v| v.pow(len) as u64).sum::<u64>() == num.into()
}
