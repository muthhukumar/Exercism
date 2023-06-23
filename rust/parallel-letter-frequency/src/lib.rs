use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

// TODO - use a separate thread for sending message. This thread should convert the input like
// lowercase and send that result to other thread for calcuation.

fn str_char_count(str: &str, count: &mut HashMap<char, usize>) {
    for c in str
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_alphanumeric() && !c.is_digit(10))
    {
        let count = count.entry(c).or_insert(0);

        *count += 1;
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for str in input.iter() {
        str_char_count(str, &mut char_count);
        // for c in str
        //     .chars()
        //     .map(|c| c.to_ascii_lowercase())
        //     .filter(|c| c.is_alphanumeric() && !c.is_digit(10))
        // {
        //     let count = char_count.entry(c).or_insert(0);
        //
        //     *count += 1;
        // }
    }

    char_count
    //
    // let mut handles = vec![];
    //
    // let idx = Arc::new(Mutex::new(0));
    // let input = Arc::new(&input);
    // let char_count: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    //
    // for _ in 1..worker_count + 1 {
    //     let idx = Arc::clone(&idx);
    //     let char_count = Arc::clone(&char_count);
    //     let input = Arc::clone(&input);
    //
    //     let handle = thread::spawn(move || {
    //         // let result = idx.lock().unwrap();
    //         // if let MutexGuard(idx) = idx.lock().unwrap() {
    //         if let Some(input) = input.get(0) {
    //             for c in input
    //                 .chars()
    //                 .map(|c| c.to_ascii_lowercase())
    //                 .filter(|c| c.is_alphanumeric() && !c.is_digit(10))
    //             {
    //                 char_count.lock().unwrap().entry(c).or_insert(0);
    //
    //                 // *count += 1;
    //             }
    //         };
    //     });
    //
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    //
    // match char_count.lock() {
    //     Ok(char_count) => char_count.clone(),
    //     Err(_) => HashMap::new() as HashMap<char, usize>,
    // }
}
