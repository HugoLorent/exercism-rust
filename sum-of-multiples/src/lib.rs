use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();

    for &factor in factors {
        // Ignore factor 0 to avoid infinite loop
        if factor == 0 {
            continue;
        }

        let mut multiple = factor;
        while multiple < limit {
            set.insert(multiple);
            multiple += factor;
        }
    }

    set.iter().sum()
}
