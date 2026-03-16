pub fn reverse(input: &str) -> String {
    let mut reverse_vec: Vec<char> = vec![' '; input.chars().count()];

    let mut vec_idx: usize = input.chars().count();
    for c in input.chars() {
        vec_idx -= 1;
        reverse_vec[ vec_idx ] = c;
    }

    return reverse_vec.iter().collect();
}
