use crate::_4_basic_data_structures::_4_3_deque::Deque;
/// 回文字符串检测
pub fn pal_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        let _ = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        let head = d.remove_front();
        let tail = d.remove_rear();
        if head != tail {
            // 比较首位字符，若不同则非回文
            is_pal = false;
        }
    }

    is_pal
}
