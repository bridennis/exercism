/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() < 2 {
        return false;
    }

    let mut sum = 0;
    for (i, ch) in code.chars().filter(|c| *c != ' ').rev().enumerate() {
        if !ch.is_numeric() {
            return false;
        }

        let digit = ch.to_digit(10).unwrap();
        sum += match (i & 1 == 1, digit * 2 > 9) {
            (true, true) => (digit * 2) - 9,
            (true, false) => digit * 2,
            _ => digit,
        }
    }

    sum.is_multiple_of(10)
}
