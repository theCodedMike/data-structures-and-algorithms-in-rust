/// 第二种解法是检查第一个字符串中的字符是否出现在第二个字符串中
pub fn anagram_solution2(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    // s1和s2中的字符分别加入alist、blist
    let mut alist = Vec::new();
    let mut blist = Vec::new();
    for c in s1.chars() {
        alist.push(c);
    }
    for c in s2.chars() {
        blist.push(c);
    }

    let mut pos1: usize = 0; // pos1、pos2 索引字符
    let mut ok = true; // 乱序字符串标示、控制循环进程

    while pos1 < s1.len() && ok {
        let mut pos2: usize = 0;
        // found标示字符是否在s2中
        let mut found = false;
        while pos2 < blist.len() && !found {
            if alist[pos1] == blist[pos2] {
                found = true;
            } else {
                pos2 += 1;
            }
        }
        // 某字符存在于 s2 中，将其替换成' '避免再次比较
        if found {
            blist[pos2] = ' ';
        } else {
            ok = false;
        }
        // 处理 s1 中的下一个字符
        pos1 += 1;
    }
    ok
}

pub fn custom_anagram_solution2(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut find_outer_char_in_s2;

    for outer_char in s1.chars() {
        find_outer_char_in_s2 = false;

        for inner_char in s2.chars() {
            if outer_char == inner_char {
                find_outer_char_in_s2 = true;
                break;
            }
        }

        if !find_outer_char_in_s2 {
            return false;
        }
    }

    return true;
}
