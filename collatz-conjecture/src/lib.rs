pub fn collatz(n: u64) -> Option<u64> {
    let mut nb = n;
    let mut steps: u64 = 0;

    if n == 0 {
        return None;
    }

    while nb != 1 {
        if nb % 2 == 0 {
            nb /= 2;
        } else {
            nb = nb * 3 + 1
        }

        steps += 1
    }

    Some(steps)
}
