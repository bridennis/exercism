pub fn raindrops(n: u32) -> String {
    let raindrops = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|&(number, raindrop)| n.is_multiple_of(number).then_some(raindrop))
        .collect::<String>();

    if raindrops.is_empty() {
        return n.to_string();
    }

    raindrops
}
