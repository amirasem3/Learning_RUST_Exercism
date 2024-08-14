use std::ops::Add;

pub fn verse(n: u32) -> String {
    let mut  n_minus_1 =0;
    if n != 0 {
        n_minus_1 = n-1;
        match n {
            0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall."),
            1 => String::from(n.to_string() + " bottle of beer on the wall, " + n.to_string().as_str() + " bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall."),
            2 => String::from(n.to_string() + " bottles of beer on the wall, " + n.to_string().as_str() + " bottles of beer.\nTake one down and pass it around, "+ n_minus_1.to_string().as_str() +  " bottle of beer on the wall."),
            _ => String::from(n.to_string() + " bottles of beer on the wall, " + n.to_string().as_str() +  " bottles of beer.\nTake one down and pass it around, "+ n_minus_1.to_string().as_str() +  " bottles of beer on the wall.")
        }
    }
    else { String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.") }




}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    for i in (end..=start).rev() {
        result.push_str(&verse(i));
        if i != end {
            result.push_str("\n\n");
        }
    }
    result
}
