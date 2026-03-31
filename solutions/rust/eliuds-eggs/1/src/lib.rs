pub fn egg_count(display_value: u32) -> usize {
    let mut count: usize = 0;
    let mut mask: u32 = 1;
    while mask != 0 {
        if display_value & mask != 0 {
            count += 1;
        }

        mask <<= 1;
    }

    count
}
