pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit).fold(0, |acc, element| {
        match factors
            .iter()
            .find(|factor| *factor != &0 && (element % *factor) == 0)
        {
            None => acc,
            Some(_) => acc + element,
        }
    })
}
