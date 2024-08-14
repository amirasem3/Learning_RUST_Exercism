use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize(word);
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let normalized_candidate = normalize(candidate);
            normalized_candidate == normalized_word && candidate.to_lowercase() != word.to_lowercase()
        })
        .cloned()
        .collect()
}

// Helper function to normalize a word by sorting its characters
fn normalize(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

