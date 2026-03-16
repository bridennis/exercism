pub fn brackets_are_balanced(string: &str) -> bool {
    let open_chars = ['[', '{', '('];
    let close_chars = [']', '}', ')'];
    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        if open_chars.contains(&c) {
            stack.push(c);
            continue;
        }

        if let Some(pos) = close_chars.iter().position(|&ch| ch == c) {
            if stack.is_empty() || stack.last().unwrap() != &open_chars[pos] {
                return false;
            }

            stack.pop();
        }
    }

    stack.is_empty()
}
