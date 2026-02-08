// WASM-optimized NLP operations for NUJ Analyzer
// AOT compilation to WebAssembly for maximum performance

use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Sentiment analysis using optimized word-based scoring
#[wasm_bindgen]
pub fn analyze_sentiment(text: &str) -> f64 {
    let positive_words = vec![
        "protect", "safety", "support", "right", "privacy",
        "transparent", "clear", "fair", "help", "secure"
    ];

    let negative_words = vec![
        "ban", "prohibit", "restrict", "remove", "suspend",
        "terminate", "violation", "penalty", "liable", "harm"
    ];

    let words: Vec<&str> = text.to_lowercase().split_whitespace().collect();
    let total = words.len() as f64;

    if total == 0.0 {
        return 0.5; // Neutral
    }

    let mut positive_count = 0;
    let mut negative_count = 0;

    for word in &words {
        if positive_words.iter().any(|pw| word.contains(pw)) {
            positive_count += 1;
        }
        if negative_words.iter().any(|nw| word.contains(nw)) {
            negative_count += 1;
        }
    }

    let sentiment = (positive_count as f64 - negative_count as f64) / total;
    // Normalize to 0-1 range
    (sentiment + 1.0) / 2.0
}

/// Extract key terms using TF-IDF-like scoring
#[wasm_bindgen]
pub fn extract_key_terms(text: &str, top_n: usize) -> JsValue {
    let stop_words: Vec<&str> = vec![
        "the", "a", "an", "and", "or", "but", "in", "on", "at",
        "to", "for", "of", "with", "by", "from", "is", "are", "was", "were"
    ];

    let words: Vec<String> = text
        .to_lowercase()
        .split_whitespace()
        .filter(|w| w.len() > 3 && !stop_words.contains(w))
        .map(|w| w.to_string())
        .collect();

    let mut freq: HashMap<String, usize> = HashMap::new();
    for word in &words {
        *freq.entry(word.clone()).or_insert(0) += 1;
    }

    let mut terms: Vec<(String, usize)> = freq.into_iter().collect();
    terms.sort_by(|a, b| b.1.cmp(&a.1));

    let result: Vec<String> = terms
        .into_iter()
        .take(top_n)
        .map(|(word, _)| word)
        .collect();

    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Detect changes between two texts using Levenshtein distance
#[wasm_bindgen]
pub fn detect_changes(old_text: &str, new_text: &str) -> f64 {
    let distance = levenshtein_distance(old_text, new_text);
    let max_len = old_text.len().max(new_text.len()) as f64;

    if max_len == 0.0 {
        return 0.0;
    }

    // Return change score (0.0 = identical, 1.0 = completely different)
    distance as f64 / max_len
}

/// Optimized Levenshtein distance for WASM
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    let mut prev_row: Vec<usize> = (0..=len2).collect();
    let mut curr_row = vec![0; len2 + 1];

    for i in 1..=len1 {
        curr_row[0] = i;

        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };

            curr_row[j] = (curr_row[j - 1] + 1)
                .min(prev_row[j] + 1)
                .min(prev_row[j - 1] + cost);
        }

        std::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentiment_analysis() {
        let positive = "We will protect your privacy and support journalists' rights.";
        let negative = "Accounts may be suspended or terminated for violations.";

        assert!(analyze_sentiment(positive) > 0.5);
        assert!(analyze_sentiment(negative) < 0.5);
    }

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
    }
}
