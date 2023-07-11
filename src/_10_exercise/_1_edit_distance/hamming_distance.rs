///
/// 计算数字的汉明距离
///
pub fn hamming_distance_of_number(source: usize, target: usize) -> usize {
    let mut count = 0;
    let mut xor = source ^ target;

    // 异或取值
    while xor != 0 {
        count += xor & 1;
        xor >>= 1;
    }

    count
}

pub fn hamming_distance_of_number2(source: usize, target: usize) -> usize {
    (source ^ target).count_ones() as usize
}

///
/// 计算字符串的汉明距离
///
pub fn hamming_distance_of_str(source: &str, target: &str) -> Option<usize> {
    let mut count = 0;
    let mut source = source.chars();
    let mut target = target.chars();

    // 2个字符串逐字符比较，可能出现如下4种情况
    loop {
        match (source.next(), target.next()) {
            (Some(s), Some(t)) if s != t => count += 1,
            (Some(_), None) | (None, Some(_)) => return None,
            (None, None) => break,
            _ => continue,
        }
    }

    Some(count)
}
