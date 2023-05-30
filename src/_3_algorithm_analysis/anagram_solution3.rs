/// 先按照字母顺序从 a 到 z 排列每个字符串，如果排列后的两个字
/// 符串相同，那这两个字符串就是乱序字符串
pub fn anagram_solution3(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    // s1和s2中的字符分别加入alist、blist并排序
    let mut alist = Vec::new();
    let mut blist = Vec::new();
    for c in s1.chars() {
        alist.push(c);
    }
    for c in s2.chars() {
        blist.push(c);
    }
    alist.sort();
    blist.sort();

    // 逐个比较排序后的集合，任何字符不匹配就退出循环
    let mut pos: usize = 0;
    let mut matched = true;
    while pos < alist.len() && matched {
        if alist[pos] == blist[pos] {
            pos += 1;
        } else {
            matched = false;
        }
    }

    matched
}

pub fn custom_anagram_solution3(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1_chars = s1.chars().collect::<Vec<_>>();
    let mut s2_chars = s2.chars().collect::<Vec<_>>();
    s1_chars.sort();
    s2_chars.sort();

    assert_eq!(s1_chars.len(), s2_chars.len());
    for idx in 0..s1_chars.len() {
        if s1_chars[idx] != s2_chars[idx] {
            return false;
        }
    }

    true
}
