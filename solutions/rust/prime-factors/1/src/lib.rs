pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut remainder = n;
    let mut candidate = 2_u64;
    while remainder >= candidate {
        if remainder.is_multiple_of(candidate) {
            prime_factors.push(candidate);
            remainder /= candidate;
        } else {
            candidate += if candidate == 2 { 1 } else { 2 };
        }
    }

    prime_factors
}
