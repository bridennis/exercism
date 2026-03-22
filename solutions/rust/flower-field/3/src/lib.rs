const FLOWER_CHAR: u8 = b'*';

// All the inputs and outputs are in ASCII
pub fn annotate(garden: &[&str]) -> Vec<String> {
    (0..garden.len())
        .map(|i| {
            (garden[i].as_bytes().iter().enumerate())
                .map(|(j, &c)| match c {
                    FLOWER_CHAR => c as char,
                    _ => flower_counter(garden, i as i32, j as i32),
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn flower_counter(garden: &[&str], i: i32, j: i32) -> char {
    let mut counter: u32 = 0;
    for h in -1_i32..=1 {
        for v in -1_i32..=1 {
            if let Some(row) = garden.get((i + h) as usize)
                && let Some(square) = row.as_bytes().get((j + v) as usize)
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
