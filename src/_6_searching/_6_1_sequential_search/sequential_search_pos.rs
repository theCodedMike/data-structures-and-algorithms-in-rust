/// 顺序查找
///
/// 如果能找到则返回索引下标，如果找不到返回None
///
pub fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;
    let mut found = false;

    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }

    if found {
        Some(pos)
    } else {
        None
    }
}
