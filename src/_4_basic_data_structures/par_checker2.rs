use crate::_4_basic_data_structures::{par_match, Stack};

/// 括号"(){}[]"匹配
pub fn par_checker2(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        // 同时判断三种开符号
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                // 比较当前括号和栈顶括号是否匹配
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }

        index += 1;
    }

    balance && stack.is_empty()
}
