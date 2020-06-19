pub fn raindrops(n: u32) -> String {
    let mut drops = String::from("");
    for (number, text) in [(3, "Pling"), (5, "Plang"), (7, "Plong")].iter() {
        if n % number == 0 {
            drops.push_str(text);
        }
    }

    if drops.len() == 0 {
        n.to_string()
    } else {
        drops
    }
}
