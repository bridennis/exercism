pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0_u32;
    for i in 0..factors.len() {
        if factors[i] == 0 {
            continue;
        }

        let mut multiplier = 1;
        let mut point = factors[i] * multiplier;
        while point < limit {
            if !factors.iter().take(i).any(|&x| point.is_multiple_of(x)) {
                sum += point;
            }

            multiplier += 1;
            point = factors[i] * multiplier;
        }
    }

    sum
}
