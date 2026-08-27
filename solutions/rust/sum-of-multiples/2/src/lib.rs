use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    for factor in factors {
        if *factor == 0 {
            continue;
        }
        let mut m = *factor;
        while m < limit {
            multiples.insert(m);
            m += *factor;
        }
    }

    multiples.iter().sum()
}
