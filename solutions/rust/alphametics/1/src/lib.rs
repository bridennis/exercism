use indexmap::IndexSet;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    if let Some(terms) = parse_input(input) {
        if term_longer_than_sum(&terms.0, terms.1) {
            return None;
        }

        return collate(terms.0, terms.1);
    }

    None
}

fn collate(terms: HashMap<&str, u8>, sum: &str) -> Option<HashMap<char, u8>> {
    let (chars, first_chars) = collect_chars(&terms, sum);

    for digits in (0..=9)
        .collect::<Vec<u8>>()
        .iter()
        .permutations(chars.len())
    {
        if first_char_callated_to_zero(&first_chars, &chars, &digits) {
            continue;
        }

        if sum_eq_terms_sum(sum, &terms, &chars, &digits) {
            return Some(
                chars
                    .iter()
                    .zip(digits.iter())
                    .map(|(&c, &&d)| (c, d))
                    .collect(),
            );
        }
    }

    None
}

fn collect_chars(terms: &HashMap<&str, u8>, sum: &str) -> (IndexSet<char>, HashSet<char>) {
    let mut chars: IndexSet<char> = IndexSet::new();
    let mut first_chars: HashSet<char> = HashSet::new();
    terms
        .iter()
        .flat_map(|(key, _)| key.chars().enumerate())
        .chain(sum.chars().enumerate())
        .for_each(|(i, c)| {
            chars.insert(c);
            if i == 0 {
                first_chars.insert(c);
            }
        });

    (chars, first_chars)
}

fn sum_eq_terms_sum(
    sum: &str,
    terms: &HashMap<&str, u8>,
    chars: &IndexSet<char>,
    digits: &Vec<&u8>,
) -> bool {
    let sum_number = word_to_number(sum, &1, chars, digits);
    let mut terms_sum = 0;
    for term in terms {
        terms_sum += word_to_number(term.0, term.1, chars, digits);
        if terms_sum > sum_number {
            break;
        }
    }

    terms_sum == sum_number
}

fn first_char_callated_to_zero(
    first_chars: &HashSet<char>,
    chars: &IndexSet<char>,
    digits: &Vec<&u8>,
) -> bool {
    digits
        .iter()
        .enumerate()
        .any(|(i, m)| **m == 0 && first_chars.contains(&chars[i]))
}

fn word_to_number(word: &str, multiplier: &u8, chars: &IndexSet<char>, digits: &[&u8]) -> u64 {
    word.chars().fold(0, |acc, ch| {
        acc * 10 + *digits[chars.iter().position(|x| *x == ch).unwrap()] as u64
    }) * *multiplier as u64
}

fn term_longer_than_sum(terms: &HashMap<&str, u8>, sum: &str) -> bool {
    terms.iter().any(|(s, _)| s.len() > sum.len())
}

fn parse_input(input: &str) -> Option<(HashMap<&str, u8>, &str)> {
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
