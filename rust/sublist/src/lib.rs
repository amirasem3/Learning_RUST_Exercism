use crate::Comparison::{Sublist, Superlist, Unequal};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    if _first_list.is_empty() && _second_list.is_empty() { return Comparison::Equal;}
    else if _first_list.is_empty() && !_second_list.is_empty() { return Sublist;}
    else if !_first_list.is_empty() && _second_list.is_empty() {return  Superlist;}
    else if _first_list.iter().eq(_second_list) { return Comparison::Equal; }
    else if _second_list.windows(_first_list.len()).any(|window| window == _first_list) {return Sublist;}
    else if _first_list.windows(_second_list.len()).any(|window| window == _second_list) { return Superlist; }
    else if !_first_list.iter().eq(_second_list) { return Unequal; }
    Unequal


}

