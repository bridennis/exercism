use std::vec;
const FLOWER_CHAR: u8 = b'*';

// All the inputs and outputs are in ASCII
pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut annotation: Vec<String> = vec![];
    for i in 0..garden.len() {
        let mut row: Vec<char> = vec![];
        for (j, &c) in garden[i].as_bytes().iter().enumerate() {
            row.push(match c {
                FLOWER_CHAR => c as char,
                _ => flower_counter(garden, i, j),
            });
        }

        annotation.insert(i, row.iter().collect());
    }

    annotation
}

fn flower_counter(garden: &[&str], i: usize, j: usize) -> char {
    let mut counter: u32 = 0;
    for h in -1_i32..=1 {
        for v in -1_i32..=1 {
            if let Some(row) = garden.get((i as i32 + h) as usize)
                && let Some(square) = row.as_bytes().get((j as i32 + v) as usize)
                && *square == FLOWER_CHAR
            {
                counter += 1;
            }
        }
    }

    match counter {
        0 => ' ',
        _ => char::from_digit(counter, 10).unwrap(),
    }
}
