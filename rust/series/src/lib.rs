use std::fmt::format;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut digits_string = String::from(digits);
    let mut string_result = Vec::new();
    if len > digits.len() {
      return string_result;
    }
    for i in 0..=digits.len() - len{
        string_result.push(digits[i..i+len].to_string());
    }

    string_result


}
