pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    // Use len() over chars.count() because we have only ASCII characters
    let power = num_str.len() as u32;

    let sum: u32 = num_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|digit| digit.pow(power))
        .sum();

    sum == num
}
