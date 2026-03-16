pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2_u32];
    let nth = n as usize;

    let mut candidate = 3_u32;
    while primes.len() <= nth {
        let sqrt = candidate.isqrt();
        if !primes
            .iter()
            .any(|&prime| prime <= sqrt && candidate.is_multiple_of(prime))
        {
            primes.push(candidate);
        }

        candidate += 2;
    }

    primes[nth]
}
