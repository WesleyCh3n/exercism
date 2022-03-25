use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    match _first_list.len().cmp(&_second_list.len()) {
        Greater => match compare(_second_list, _first_list) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        },
        Less => match compare(_first_list, _second_list) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        },
        Equal => match _first_list == _second_list {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
    }
}

pub fn compare<T: PartialEq>(_sm: &[T], _lg: &[T]) -> bool {
    let lg_len = _lg.len();
    let sm_len = _sm.len();
    for s in 0..lg_len - sm_len + 1 {
        if _sm == &_lg[s..s + sm_len] {
            return true;
        }
    }
    false
}
