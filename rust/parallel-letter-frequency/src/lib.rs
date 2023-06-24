use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

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

fn cal_thread_work(no_of_thread: usize, no_of_works: usize) -> Vec<[usize; 2]> {
    let work_per_thread = no_of_works / no_of_thread;
    let remainder = no_of_works % no_of_thread;

    let mut i = 0;
    let mut works = vec![];

    while i < no_of_works {
        let start = i;

        let end = no_of_works.min(i + (work_per_thread));

        if (end + remainder) == no_of_works {
            works.push([start, end + remainder]);

            return works;
        }

        works.push([start, end]);

        i += work_per_thread;
    }

    works
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let works = cal_thread_work(worker_count, input.len());

    let tf_input: Arc<RwLock<Vec<String>>> =
        Arc::new(RwLock::new(input.iter().map(|v| v.to_string()).collect()));

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
