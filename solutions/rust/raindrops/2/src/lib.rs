pub fn raindrops(n: u32) -> String {
    let mut raindrops = String::new();
    for number in [3, 5, 7] {
        raindrops += match (number, n.is_multiple_of(number)) {
            (3, true) => "Pling",
            (5, true) => "Plang",
            (7, true) => "Plong",
            _ => "",
        };
    }

    if raindrops.is_empty() {
        return n.to_string();
    }

    raindrops
}
