pub fn square_of_sum(n: u32) -> u32 {
    // Formula: (n(n+1)/2)²
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    // Formula: n(n+1)(2n+1)/6
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n).saturating_sub(sum_of_squares(n))
}
