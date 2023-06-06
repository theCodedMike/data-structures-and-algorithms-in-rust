use crate::_4_basic_data_structures::_4_1_stack::Stack;

pub fn num2str_stk(mut num: i32, base: i32) -> String {
    let digits: [&str; 16] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];

    let mut rem_stack = Stack::new();
    while num > 0 {
        if num < base {
            // 不超过base直接入栈
            rem_stack.push(num);
        } else {
            // 超过base余数入栈
            rem_stack.push(num % base);
        }

        num /= base;
    }

    // 余数出栈并组成字符串
    let mut num_str = "".to_string();
    while !rem_stack.is_empty() {
        num_str += digits[rem_stack.pop().unwrap() as usize];
    }

    num_str
}
