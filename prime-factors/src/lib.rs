pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut remaining = n;

    // Special case: 1 has no prime factors
    if n == 1 {
        return result;
    }

    // Start with the first prime number, 2
    let mut divisor = 2;

    // Continue until remaining is 1 or the divisor is greater
    // than the square root of the remaining number
    while divisor * divisor <= remaining {
        // As long as the divisor divides remaining exactly, add it as a factor
        while remaining % divisor == 0 {
            result.push(divisor);
            remaining /= divisor;
        }

        // Move to the next divisor
        // We could use divisor += 1, but for efficiency, we do:
        // - +1 after 2 (to go to 3)
        // - +2 for all others (to only try odd numbers)
        if divisor == 2 {
            divisor = 3;
        } else {
            divisor += 2;
        }
    }

    // If remaining > 1, it's a prime factor itself
    if remaining > 1 {
        result.push(remaining);
    }

    result
}
