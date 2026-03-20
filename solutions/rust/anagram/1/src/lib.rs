use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let word_freq = freq(&lowercase_word);

    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for possible_anagram in possible_anagrams {
        let lowercase_anagram = possible_anagram.to_lowercase();
        if lowercase_word == lowercase_anagram {
            continue;
        }

        if freq(&lowercase_anagram).eq(&word_freq) {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}

fn freq(word: &str) -> HashMap<char, u32> {
    let mut freq_map = HashMap::new();
    for ch in word.chars() {
        *freq_map.entry(ch).or_insert(0) += 1;
    }

    freq_map
}
