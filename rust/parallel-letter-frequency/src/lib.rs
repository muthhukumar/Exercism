use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
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

pub fn frequency_without_worker(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for str in input.iter() {
        str_char_count(str, &mut char_count);
    }

    char_count
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut works = vec![];

    let mut start = 0;
    if input.len() >= worker_count {
        for i in 1..worker_count + 1 {
            let work_count = {
                if input.len() % worker_count == 0 {
                    input.len() / worker_count
                } else {
                    if i == worker_count {
                        (input.len() / worker_count) + (input.len() % worker_count)
                    } else {
                        input.len() / worker_count
                    }
                }
            };

            works.push([start, start + work_count]);

            start = start + work_count;
        }
    } else {
        works.push([0, input.len()]);
    }

    let input = input.clone().to_owned();

    let tf_input: Arc<RwLock<Vec<String>>> =
        Arc::new(RwLock::new(input.iter().map(|s| s.to_string()).collect()));

    let char_count: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = vec![];

    for [start, end] in works {
        let cloned_tf_input = Arc::clone(&tf_input);
        let char_count = Arc::clone(&char_count);

        let handle = thread::spawn(move || {
            let input = cloned_tf_input.read().unwrap();
            if let Some(val) = input.get(start..end) {
                for str in val {
                    let mut char_count = char_count.lock().unwrap();
                    str_char_count(str, &mut char_count);
                }
            };
        });

        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let x = char_count.lock().unwrap().to_owned();

    x
}
