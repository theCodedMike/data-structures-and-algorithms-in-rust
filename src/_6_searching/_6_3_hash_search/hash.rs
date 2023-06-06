/// ASCII哈希函数
pub fn hash1(a_str: &str, size: usize) -> usize {
    let mut sum = 0;

    for c in a_str.chars() {
        sum += c as usize;
    }

    sum % size
}
/// ASCII哈希函数
///
/// 带权重
///
pub fn hash2(a_str: &str, size: usize) -> usize {
    let mut sum = 0;

    for (idx, c) in a_str.chars().enumerate() {
        sum += (idx + 1) * (c as usize);
    }

    sum % size
}
