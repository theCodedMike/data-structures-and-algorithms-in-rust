mod base_converter;
mod divide_by_two;
mod infix_to_postfix;
mod par_checker1;
mod par_checker2;
mod par_checker3;
mod postfix_eval;
mod stack;
mod stack_exercise;

pub use base_converter::*;
pub use divide_by_two::*;
pub use infix_to_postfix::*;
pub use par_checker1::*;
pub use par_checker2::*;
pub use par_checker3::*;
pub use postfix_eval::*;
pub use stack::*;
pub use stack_exercise::*;

/// 同时检测多种开闭符号是否匹配
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn get_target_char(rem: usize) -> char {
    if rem >= 16 {
        panic!("idx must be [0, 15]");
    }
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    digits[rem]
}
