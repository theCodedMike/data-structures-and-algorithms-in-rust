use crate::_4_basic_data_structures::_4_1_stack::Stack;

/// 圆括号"()"匹配
pub fn par_checker1(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true; // 括号是否匹配（平衡）指示
    let mut stack = Stack::new(); // 使用前面实现的栈
    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c {
            // 如果为开符号，入栈
            stack.push(c);
        } else {
            // 如果为闭括号，判断栈是否为空
            if stack.is_empty() {
                balance = false; // 为空则不平衡
            } else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }

    // 平衡且栈为空，括号表达式才是匹配的
    balance && stack.is_empty()
}
