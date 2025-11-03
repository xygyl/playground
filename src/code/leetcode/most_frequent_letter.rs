use inquire::Text;
use std::collections::HashMap;

/// Returns the most frequent letter from a given string. Uppercase and lowercase numbers are treated as different.
pub fn most_frequent_letter() -> Option<()> {
    let input = Text::new("Enter string:").prompt().ok()?;

    let mut freq: HashMap<char, usize> = HashMap::new();

    for ch in input.chars() {
        if ch.is_alphabetic() {
            *freq.entry(ch).or_insert(0) += 1;
        }
    }

    let max_freq = freq.values().copied().max().unwrap_or(0);

    for ch in input.chars() {
        if ch.is_alphabetic() && freq[&ch] == max_freq {
            println!("Most frequent letter: {}", ch);
        }
    }

    Some(())
}
