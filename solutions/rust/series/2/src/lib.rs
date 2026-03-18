pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vec = vec![];
    let mut start: usize = 0;
    while start + len <= digits.len() {
        vec.push(digits[start..(start + len)].to_string());
        start += 1;
    }

    vec
}
