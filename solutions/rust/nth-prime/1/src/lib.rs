pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();

    let mut candidate = 1_u32;
    while primes.len() as u32 <= n {
        candidate += 1;

        let sqrt = candidate.isqrt();
        if candidate == 2
            || (candidate & 1 == 1 // is odd
                && !primes.iter().any(|&x| x <= sqrt && candidate.is_multiple_of(x)))
        {
            primes.push(candidate);
        }
    }

    candidate
}
