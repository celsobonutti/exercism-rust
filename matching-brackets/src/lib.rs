const BRACKETS: &str = "()[]{}";

pub fn brackets_are_balanced(string: &str) -> bool {
    let filtered_string: String = string.chars().filter(|x: &char| {
        BRACKETS.contains(*x)
    }).collect();

    println!("{}", filtered_string);
    false
}