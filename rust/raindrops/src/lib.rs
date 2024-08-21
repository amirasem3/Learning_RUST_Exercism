use std::fmt::format;

pub fn raindrops(n: u32) -> String {
    let mut result_string = String::new();
    if n %3 !=0 && n%5 !=0 && n%7 !=0{
        result_string.push_str(&format!("{}", n));
        result_string
    }
    else {
        if n % 3 == 0 {
            result_string.push_str(&format!("Pling"));
        }
        if n %5 == 0 {
            result_string.push_str("Plang");
        }
        if n % 7 == 0 {
            result_string.push_str("Plong");
        }
        result_string
    }


}
