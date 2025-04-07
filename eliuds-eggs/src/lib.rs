pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0;
    let mut number = display_value;

    while number != 0 {
        number = number & (number - 1);
        count += 1;
    }

    count
}
