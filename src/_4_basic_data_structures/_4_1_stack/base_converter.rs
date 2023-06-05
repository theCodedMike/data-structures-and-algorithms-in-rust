use crate::_4_basic_data_structures::_4_1_stack::Stack;
/// 十进制数字转任意进制
///
/// 为安全起见，base不要大于16
pub fn base_converter(mut dec_num: u32, base: u32) -> String {
    // digits对应各种余数的字符形式，尤其是10-15
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();
    // 余数入栈
    while dec_num != 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }
    // 余数出栈并取对应字符来拼接成字符串
    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }

    base_str
}
