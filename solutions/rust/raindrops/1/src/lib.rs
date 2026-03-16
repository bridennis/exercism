pub fn raindrops(n: u32) -> String {
    let mut raindrops = String::new();
    for number in [3, 5, 7] {
        raindrops += match number {
            3 if n.is_multiple_of(number) => "Pling",
            5 if n.is_multiple_of(number) => "Plang",
            7 if n.is_multiple_of(number) => "Plong",
            _ => "",
        };
    }

    if raindrops.is_empty() {
        return n.to_string();
    }

    raindrops
}
