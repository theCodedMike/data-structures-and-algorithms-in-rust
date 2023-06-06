const BASE_STR: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

pub fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        return BASE_STR[num as usize].to_string();
    }
    // 余数加在末尾
    num2str_rec(num / base, base) + BASE_STR[(num % base) as usize]
}
