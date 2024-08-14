extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    if input.eq("") { input.to_string(); };
    let input_word : &str = input;
    let drow: String  = input_word.graphemes(true).rev().collect();

    drow
}
