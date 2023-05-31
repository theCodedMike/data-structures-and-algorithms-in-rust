use crate::_4_basic_data_structures::{par_match, Stack};

/// 表达式中的括号匹配
///
/// eg:
/// (2+3)*(3-1)
pub fn par_checker3(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        // 开符号入栈
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        }
        if c == ')' || c == ']' || c == '}' {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        // 非括号直接跳过
        index += 1;
    }

    balance && stack.is_empty()
}
