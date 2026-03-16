pub fn is_armstrong_number(num: u32) -> bool {
    let mut number_of_digits: usize = 0;
    let mut remains: u32 = num;

    let mut digits = [0u32; 10];
    while remains > 0 {
        digits[number_of_digits] = remains % 10;
        number_of_digits += 1;
        remains /= 10;
    }

    let mut sum: u32 = 0;
    for i in 0..number_of_digits {
        sum += digits[i].pow(number_of_digits as u32);
    }

    sum == num
}
