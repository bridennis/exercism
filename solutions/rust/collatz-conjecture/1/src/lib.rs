pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        _ => {
            let mut x = 0_u64;
            let mut number = n;
            while number != 1 {
                number = if number & 1 == 0 {
                    number / 2
                } else {
                    number * 3 + 1
                };

                x += 1;
            }
            Some(x)
        }
    }
}
