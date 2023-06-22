use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    println!("input: {input:?}");

    for str in input.iter() {
        for c in str
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .filter(|c| c.is_alphanumeric() && !c.is_digit(10))
        {
            let count = char_count.entry(c).or_insert(0);

            *count += 1;
        }
    }

    let mut handles = vec![];

    for _ in 1..worker_count + 1 {
        let handle = thread::spawn(move || {});

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    char_count
}
