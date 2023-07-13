use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut mul_of_factors: HashSet<u32> = HashSet::new();

    for f in factors {
        let mut i = *f;

        while i < limit {
            mul_of_factors.insert(i);

            i += *f;
        }
    }

    mul_of_factors.iter().sum::<u32>()
}
