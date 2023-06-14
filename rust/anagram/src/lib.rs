use std::collections::{HashMap, HashSet};

// This one only works for alphabets
pub fn char_count(str: &str) -> [u8; 26] {
    let mut count: [u8; 26] = [0; 26];

    for c in str
        .chars()
        .filter(|c| c.to_ascii_lowercase().is_ascii_alphabetic())
    {
        let idx = (c as u8 - b'a') as usize;
        count[idx] += 1;
    }

    count
}

pub fn char_count_hash(str: &str) -> HashMap<String, u32> {
    let mut result: HashMap<String, u32> = HashMap::new();

    for c in str.chars().map(|c| c.to_lowercase()) {
        *result.entry(c.to_string()).or_insert(0) += 1;
    }

    result
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();

    let word_char_count = char_count_hash(word);

    for i in possible_anagrams {
        if *i.to_lowercase() != word.to_lowercase() && char_count_hash(*i) == word_char_count {
            result.insert(&i);
        }
    }

    result
}
