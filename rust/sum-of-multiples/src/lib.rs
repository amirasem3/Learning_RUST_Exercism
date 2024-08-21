use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if limit == 1 {
        return 0;
    }

    let mut multiples = HashSet::new();

    for &factor in factors {
        if factor == 0 {
            continue; // Skip factor 0 to avoid infinite loops.
        }

        let mut multiple = factor;
        while multiple < limit {
            multiples.insert(multiple);
            multiple += factor;
        }
    }

    multiples.iter().sum()
}