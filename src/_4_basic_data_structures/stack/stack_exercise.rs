use crate::_4_basic_data_structures::stack::{get_target_char, par_match};

pub struct MyStack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        MyStack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}
/// 圆括号匹配
pub fn par_checker1_exercise(s: &str) -> bool {
    // 判空
    if s.trim().is_empty() {
        return false;
    }

    let mut stack = MyStack::new();
    for c in s.chars() {
        if c == '(' {
            // 如果是左括号则入栈
            stack.push(c);
        } else {
            // 如果是右括号则出栈，出栈前需判空
            if stack.is_empty() {
                // 如果此时栈为空，说明括号不匹配
                return false;
            } else {
                stack.pop();
            }
        }
    }

    // 遍历完以后如果栈恰好为空，说明括号是匹配的
    stack.is_empty()
}

/// 括号"(){}[]"匹配
pub fn par_checker2_exercise(s: &str) -> bool {
    // 判空
    if s.trim().is_empty() {
        return false;
    }

    let mut stack = MyStack::new();
    for c in s.chars() {
        match c {
            //如果是左括号则入栈
            '(' | '{' | '[' => stack.push(c),
            // 如果是右括号则出栈，出栈前需判空
            _ => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    return false;
                }
            }
        }
    }

    // 遍历完以后如果栈恰好为空，说明括号是匹配的
    stack.is_empty()
}

/// 表达式中的括号匹配
///
/// eg:
/// (2+3)*(3-1)
pub fn par_checker3_exercise(s: &str) -> bool {
    // 判空
    if s.trim().is_empty() {
        return false;
    }

    let mut stack = MyStack::new();
    for c in s.chars() {
        match c {
            //如果是左括号则入栈
            '(' | '{' | '[' => stack.push(c),
            // 如果是右括号则出栈，出栈前需判空
            ')' | '}' | ']' => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    return false;
                }
            }
            // 如果是其他字符，do nothing
            _ => {}
        }
    }

    // 遍历完以后如果栈恰好为空，说明括号是匹配的
    stack.is_empty()
}

/// 十进制转二进制
pub fn divide_by_two_exercise(mut dec_num: usize) -> String {
    let mut rem_stack = MyStack::new();

    // 余数入栈
    let mut rem;
    while dec_num != 0 {
        rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    // 从栈顶开始取值并拼接
    let mut bin_str = String::new();
    while !rem_stack.is_empty() {
        rem = rem_stack.pop().unwrap();
        bin_str.push(get_target_char(rem));
    }

    bin_str
}

/// 十进制转任意进制(<=16)
pub fn base_converter_exercise(mut dec_num: usize, radix: usize) -> String {
    let mut rem_stack = MyStack::new();

    // 余数入栈
    let mut rem;
    while dec_num != 0 {
        rem = dec_num % radix;
        rem_stack.push(rem);
        dec_num /= radix;
    }

    // 从栈顶开始取值并拼接
    let mut radix_str = String::new();
    while !rem_stack.is_empty() {
        rem = rem_stack.pop().unwrap();
        radix_str.push(get_target_char(rem));
    }

    radix_str
}
