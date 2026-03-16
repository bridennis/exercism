pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    for i in 0..list.len() {
        if i + 1 < list.len() {
            proverb += &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
        } else {
            proverb += &format!("And all for the want of a {}.", list[0]);
        }
    }

    proverb
}
