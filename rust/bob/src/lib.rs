use std::ops::Index;

pub fn reply(message: &str) -> &str {
    if message.trim().is_empty() {
        return "Fine. Be that way!";
    }
    if message.chars().any(|c| c.is_alphabetic()) && message == message.to_uppercase() && !message.contains("?") && !message.trim().is_empty(){
        return "Whoa, chill out!";
    }
    if message.chars().any(|c| c.is_alphabetic()) && message == message.to_uppercase(){
        return "Calm down, I know what I'm doing!";
    }
    if message.trim_end().ends_with("?"){
        return "Sure."
    }


  return "Whatever."
}
