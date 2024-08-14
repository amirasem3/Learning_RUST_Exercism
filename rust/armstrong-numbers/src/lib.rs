pub fn is_armstrong_number(num: u32) -> bool {
    let string_version_of_the_num = num.to_string();
    let str_version_of_num = string_version_of_the_num.as_str();
    let each_digit_power = str_version_of_num.len();
    let mut armstrong_result = 0;
    for ch in str_version_of_num.chars(){
        let mut numeric_value_of_char = 0;
        if let Some(digit)  = ch.to_digit(10) {
            numeric_value_of_char  = digit as i32;
            armstrong_result += numeric_value_of_char.pow(each_digit_power as u32);
        }
        else{
            break;
        }
    }
    if armstrong_result as u32 == num {
        true
        }
    else {
        false
    }

}
