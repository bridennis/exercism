#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    const MIN_BASE: u32 = 2;

    if from_base < MIN_BASE {
        return Err(Error::InvalidInputBase);
    }

    if to_base < MIN_BASE {
        return Err(Error::InvalidOutputBase);
    }

    if number.is_empty() {
        return Ok(vec![0]);
    }

    if from_base == to_base {
        return Ok(number.to_vec());
    }

    for &n in number {
        if n > from_base - 1 {
            return Err(Error::InvalidDigit(n));
        }
    }

    let mut base_number = number.iter().fold(0, |acc, n| acc * from_base + n);

    let mut to_number: Vec<u32> = vec![];
    while base_number > 0 {
        to_number.push(base_number % to_base);
        base_number /= to_base;
    }

    if to_number.is_empty() {
        to_number.push(0);
    }

    to_number.reverse();
    Ok(to_number)
}
