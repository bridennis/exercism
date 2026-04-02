use indexmap::IndexSet;
use itertools::Itertools;
use std::collections::HashMap;

// ASCII version: A-Z
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    if let Some(terms) = parse_input(input) {
        match term_longer_than_sum(&terms.0, terms.1) {
            true => return None,
            false => return collate(terms.0, terms.1),
        }
    }

    None
}

fn collate(terms: HashMap<&str, u64>, sum: &str) -> Option<HashMap<char, u8>> {
    let (chars, first_chars_length) = collect_chars(&terms, sum);
    let mut digits_map = [255u8; 26];

    for first in (1u8..=9).permutations(first_chars_length) {
        for last in (0u8..=9)
            .filter(|x| !first.contains(x))
            .permutations(chars.len() - first_chars_length)
        {
            for (i, &ch) in chars.iter().enumerate() {
                digits_map[(ch - b'A') as usize] = match i < first.len() {
                    true => first[i],
                    false => last[i - first.len()],
                };
            }

            if sum_eq_terms_sum(sum, &terms, &digits_map) {
                return Some(
                    chars
                        .iter()
                        .map(|&ch| (ch as char, digits_map[(ch - b'A') as usize]))
                        .collect(),
                );
            }
        }
    }

    None
}

fn sum_eq_terms_sum(sum: &str, terms: &HashMap<&str, u64>, digits_map: &[u8; 26]) -> bool {
    let sum_number = word_to_number(sum, 1, digits_map);
    let mut terms_sum = 0;
    for term in terms {
        terms_sum += word_to_number(term.0, *term.1, digits_map);
        if terms_sum > sum_number {
            break;
        }
    }

    terms_sum == sum_number
}

fn word_to_number(word: &str, multiplier: u64, digits_map: &[u8; 26]) -> u64 {
    word.bytes().fold(0, |acc, ch| {
        acc * 10 + digits_map[(ch - b'A') as usize] as u64
    }) * multiplier
}

fn collect_chars(terms: &HashMap<&str, u64>, sum: &str) -> (IndexSet<u8>, usize) {
    let mut chars: IndexSet<u8> = IndexSet::new();
    let mut first_chars_length = 0;
    terms
        .iter()
        .flat_map(|(key, _)| key.chars().enumerate())
        .chain(sum.chars().enumerate())
        .for_each(|(i, c)| {
            let ch = c as u8;
            match i == 0 && !chars.contains(&ch) {
                true => {
                    chars.insert_before(0, ch);
                    first_chars_length += 1;
                }
                false => {
                    chars.insert(ch);
                }
            }
        });

    (chars, first_chars_length)
}

fn term_longer_than_sum(terms: &HashMap<&str, u64>, sum: &str) -> bool {
    terms.iter().any(|(s, _)| s.len() > sum.len())
}

fn parse_input(input: &str) -> Option<(HashMap<&str, u64>, &str)> {
    input.split_once(" == ").map(|(left, right)| {
        (
            left.split(" + ").fold(HashMap::new(), |mut acc, s| {
                *acc.entry(s).or_insert(0) += 1;
                acc
            }),
            right,
        )
    })
}
