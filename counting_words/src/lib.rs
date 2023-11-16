use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    let words = words.to_lowercase();
    let words = words
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '\'')
        .collect::<String>();

    for word in words.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }

    counts
}
