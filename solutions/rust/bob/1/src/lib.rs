pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    match (
        msg.ends_with("?"),
        msg.to_uppercase() == msg && msg.chars().any(|c| c.is_alphabetic()),
        msg.is_empty(),
    ) {
        (true, false, false) => "Sure.",
        (true, true, false) => "Calm down, I know what I'm doing!",
        (false, true, false) => "Whoa, chill out!",
        (false, false, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
