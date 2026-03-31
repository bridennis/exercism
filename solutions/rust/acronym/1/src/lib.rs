pub fn abbreviate(phrase: &str) -> String {
    const SEP: [char; 2] = [' ', '-'];
    phrase
        .chars()
        .fold((String::new(), SEP[0]), |(mut acronym, prev_ch), ch| {
            if ch.is_uppercase() && !prev_ch.is_uppercase()
                || ch.is_alphabetic() && SEP.contains(&prev_ch)
            {
                acronym.push(ch.to_uppercase().next().unwrap());
            }

            (acronym, ch)
        })
        .0
}
