pub fn is_valid(code: &str) -> bool {
   let without_spaces = remove_space(code);
    let mut validation_indicator =0;


    if without_spaces.len() > 1 && is_all_digit(without_spaces.as_str()){
            for (index,ch) in without_spaces.chars().rev().enumerate() {
                if index % 2 != 0 {
                    let mut numeric_velue_of_char = 0;
                    if let Some(digit) = ch.to_digit(10){
                        numeric_velue_of_char = digit as i32;
                        println!("chosen number for doubling {}", numeric_velue_of_char );
                        let doubled_digit = numeric_velue_of_char * 2;
                        if doubled_digit > 9 { validation_indicator += doubled_digit - 9 }
                        else {
                            validation_indicator += doubled_digit;
                        }
                    }
                    else {
                        break;
                    }

                }
                else if let Some(digit) = ch.to_digit(10){
                    println!("not_chosen_number {}", digit as i32);
                    validation_indicator += digit as i32;
                }
                else {
                    break;
                }
            }
            if validation_indicator % 10 == 0 { return true; }
            return false;

    }
    false
}

pub fn is_all_digit (code :&str) -> bool {
    code.chars().all(|c| c.is_digit(10))
}

pub fn remove_space(code: &str) -> String {
    code.chars().filter(|&c| c != ' ').collect()
}
