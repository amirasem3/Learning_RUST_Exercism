

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for ch in string.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }

    // If the stack is empty, all brackets were properly closed
    stack.is_empty()
}
// pub fn brackets_are_balanced(string: &str) -> bool {
//     let bool_result = string_validator(string);
//     println!("Validator_result : {}", bool_result);
//     if string.is_empty() || bool_result {
//          return true;
//     }
//     false
//
//
// }

pub fn count_valid_close_bracket(string  : &str, closed_bracket: char, open_bracket : char) -> i32{
    let mut valid_close_bracket_number = 0;

    for i in string.chars() {
        if i == closed_bracket &&  string.find(closed_bracket) > string.find(open_bracket)   {
            valid_close_bracket_number +=1;
        }
    }
    valid_close_bracket_number
}
pub fn string_validator(string: &str) -> bool {
    let open_b1_number = string.chars().filter(|&c| c == '[').count();
    println!("b1_open : {}", open_b1_number);
    let open_b2_number = string.chars().filter(|&c| c == '{').count();
    println!("b2_open : {}", open_b2_number);
    let open_paran_number = string.chars().filter(|&c| c == '(').count();
    println!("paran_open : {}", open_paran_number);
    let valid_b1_close = count_valid_close_bracket(string, ']', '[');
    println!("b1_close : {}", valid_b1_close);
    let valid_b2_close = count_valid_close_bracket(string, '}', '{');
    println!("b2_close: {}", valid_b2_close);
    let valid_paran_closed = count_valid_close_bracket(string, ')', '(');
    println!("paran_close : {}", valid_paran_closed);
    if  open_b1_number == valid_b1_close as usize
        && open_b2_number == valid_b2_close as usize
        && open_paran_number == valid_paran_closed as usize {
       return  true;
    }
    false
}