pub fn reply(message: &str) -> &str {
    match (
        is_question(&message),
        is_yelling(&message),
        is_empty(&message),
    ) {
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

fn is_question(message: &str) -> bool {
    message.trim().chars().last().unwrap_or(' ') == '?'
}

fn is_yelling(message: &str) -> bool {
    message.chars().any(|char| char.is_alphabetic()) && message.to_uppercase() == message
}

fn is_empty(message: &str) -> bool {
    message.trim().is_empty()
}
