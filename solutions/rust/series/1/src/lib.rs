pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vec = vec![];
    if digits.len() >= len {
        let chars: Vec<char> = digits.chars().collect();
        let mut start: usize = 0;
        while start + len <= digits.len() {
            vec.push(chars[start..(start + len)].iter().collect());
            start += 1;
        }
    }

    vec
}
