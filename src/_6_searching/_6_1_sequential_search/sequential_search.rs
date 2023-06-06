/// 顺序查找
pub fn sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0; // 索引，位置
    let mut found = false; // 是否找到

    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }

    found
}
