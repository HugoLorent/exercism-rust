use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_chars: Vec<char> = word_lower.chars().collect();
    // Use unstable because we don't care about order of equal elements
    word_chars.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|&candidate| {
            let candidate_lower = candidate.to_lowercase();
            if candidate_lower == word_lower {
                return false;
            }
            let mut candidate_chars: Vec<char> = candidate_lower.chars().collect();
            candidate_chars.sort_unstable();
            word_chars == candidate_chars
        })
        .copied()
        .collect()
}
