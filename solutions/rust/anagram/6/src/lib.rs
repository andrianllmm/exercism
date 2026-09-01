use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    let word_lower = word.to_lowercase();

    // sort word
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort();

    // for each candidate
    for candidate in possible_anagrams {
        let candidate_lower = candidate.to_lowercase();

        // skip candidates equal to original
        if word_lower == candidate_lower {
            continue;
        }

        // sort candidate
        let mut candidate_sorted: Vec<char> = candidate_lower.chars().collect();
        candidate_sorted.sort();

        // equal -> anagram
        if word_sorted == candidate_sorted {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
