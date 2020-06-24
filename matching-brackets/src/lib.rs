const BRACKETS: &str = "()[]{}";
const OPENING_BRACKETS: &str = "([{";

pub fn brackets_are_balanced(string: &str) -> bool {
    let filtered_string: String = string
        .chars()
        .filter(|x: &char| BRACKETS.contains(*x))
        .collect();

    check(filtered_string)
}

fn check(input: String) -> bool {
    let mut open_brackets: Vec<char> = vec![];

    for char in input.chars().into_iter() {
        match char {
            value if OPENING_BRACKETS.contains(value) => open_brackets.push(value),
            '}' if open_brackets.pop() == Some('{') => (),
            ')' if open_brackets.pop() == Some('(') => (),
            ']' if open_brackets.pop() == Some('[') => (),
            _ => return false,
        }
    }

    open_brackets.len() == 0
}
