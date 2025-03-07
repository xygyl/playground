use std::collections::HashMap;
use text_io::read;

pub fn find_most_frequent_letter() {
    let input: String = read!();

    let mut freq: HashMap<char, usize> = HashMap::new();

    // Makes a frequency map for the input string, for example:
    // "banana" will give this:
    // {'b': 1, 'a': 3, 'n': 2}
    for ch in input.chars() {
        if ch.is_alphabetic() {
            *freq.entry(ch).or_insert(0) += 1;
        }
    }

    let max_freq = freq
        .values() // Iterator over the values of freq
        .copied() // .values() gives a reference to the usize values in the hashmap (&usize). This converts them into owned values instead of references
        .max() // Finds the maximum value from the iterator. For the "banana" example,  it will return 'a'
        .unwrap_or(0); // Returns 0 if the input is empty

    for ch in input.chars() {
        if ch.is_alphabetic() && freq[&ch] == max_freq {
            println!("Most frequent letter: {}", ch);
            break;
        }
    }
}
