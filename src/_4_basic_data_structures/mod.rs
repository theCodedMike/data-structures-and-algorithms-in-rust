mod custom_stack;
mod divide_by_two;
mod par_checker1;
mod par_checker2;
mod par_checker3;
mod stack;

pub use custom_stack::*;
pub use divide_by_two::*;
pub use par_checker1::*;
pub use par_checker2::*;
pub use par_checker3::*;
pub use stack::*;

/// 同时检测多种开闭符号是否匹配
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}
