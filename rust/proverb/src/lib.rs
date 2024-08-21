pub fn build_proverb(list: &[&str]) -> String {
    let mut result_string = String::new();
   if list.is_empty() { return String::new(); }
    else {
        if list.len() == 1 {
            return String::from("And all for the want of a ".to_owned() + list[0] + ".")
        }
        else {
            for i in 0..list.len() -1 {
                result_string.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i+1]));
            }
            result_string.push_str(&format!("And all for the want of a {}.", list[0]));

            result_string

        }
    }
}
