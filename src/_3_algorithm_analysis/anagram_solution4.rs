use std::collections::HashMap;

/// 上面的方法中，总是要创建alist和blist，这非常费内存。
/// 如果s1和s2达到百万字符，此时alist和blist就非常大。
/// 通过分析可知，s1和s2只含26个小写字母，所以用两个长度为26的列表，统计各个字符出现的频次就可以了。
pub fn anagram_solution4(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    // 大小为26的集合，用于将字符映射为ASCII值
    let mut c1 = [0; 26];
    let mut c2 = [0; 26];
    for c in s1.chars() {
        let pos = (c as usize) - 97; // 97为a的ASCII值
        c1[pos] += 1;
    }
    for c in s2.chars() {
        let pos = (c as usize) - 97;
        c2[pos] += 1;
    }
    // 逐个比较ascii值
    let mut pos = 0;
    let mut ok = true;
    while pos < 26 && ok {
        if c1[pos] == c2[pos] {
            pos += 1;
        } else {
            ok = false;
        }
    }

    ok
}

pub fn custom_anagram_solution4(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1_map = HashMap::new();
    let mut s2_map = HashMap::new();
    for c in s1.chars() {
        s1_map.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }
    for c in s2.chars() {
        s2_map.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }

    for (key, s1_count) in s1_map {
        match s2_map.get(&key) {
            None => return false,
            Some(&s2_count) if s1_count != s2_count => return false,
            _ => {}
        }
    }

    true
}
