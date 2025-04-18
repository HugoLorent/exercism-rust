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
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    // Check digits and convert to decimal
    let mut decimal_value: u32 = 0;

    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }

        decimal_value = decimal_value * from_base + digit;
    }

    // For any value 0 (whether an empty array or digits that give 0)
    if decimal_value == 0 {
        return Ok(vec![0]);
    }

    // Convert from decimal to target base
    let mut digits: Vec<u32> = Vec::new();
    let mut value = decimal_value;

    while value > 0 {
        let remainder = value % to_base;
        digits.push(remainder);
        value /= to_base;
    }

    digits.reverse();

    Ok(digits)
}
