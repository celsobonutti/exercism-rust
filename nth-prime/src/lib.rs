struct Prime {
    value: u32,
}

impl Prime {
    fn new() -> Prime {
        Prime { value: 2 }
    }
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = self.value;

        loop {
            value += 1;
            let has_divisor = (2..value).any(|number| value != number && value % number == 0);

            if !has_divisor {
                self.value = value;
                return Some(value);
            }
        }
    }
}

pub fn nth(n: u32) -> u32 {
    let mut iterator = Prime::new();
    for _ in 0..n {
        iterator.next();
    }
    iterator.value
}
