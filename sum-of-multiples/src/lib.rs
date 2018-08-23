pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit).filter(|x| is_multiple_of_any(x, factors)).sum()
}

fn is_multiple_of_any(number: &u32, factors: &[u32]) -> bool {
    factors.iter().any(|x| number % x == 0)
}
