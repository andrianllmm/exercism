use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    // sort word
    let mut word_vec: Vec<char> = word.to_lowercase().chars().collect();
    word_vec.sort();
    let word_sorted: String = word_vec.into_iter().collect();

    for candidate in possible_anagrams {
        // sort candidate
        let mut candidate_vector: Vec<char> = candidate.to_lowercase().chars().collect();
        candidate_vector.sort();
        let candidate_sorted: String = candidate_vector.into_iter().collect();

        if word.to_lowercase() != candidate.to_lowercase() && word_sorted == candidate_sorted {
            anagrams.insert(candidate);
        }
    }
    anagrams
}
