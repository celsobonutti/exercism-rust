const OPENING_BRACKETS: &str = "([{";
const CLOSING_BRACKETS: &str = ")]}";

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut open_brackets: Vec<char> = vec![];

    for char in string.chars().into_iter() {
        match char {
            value if OPENING_BRACKETS.contains(value) => open_brackets.push(value),
            '}' if open_brackets.pop() == Some('{') => (),
            ')' if open_brackets.pop() == Some('(') => (),
            ']' if open_brackets.pop() == Some('[') => (),
            value if CLOSING_BRACKETS.contains(value) => return false,
            _ => (),
        }
    }

    open_brackets.len() == 0
}
