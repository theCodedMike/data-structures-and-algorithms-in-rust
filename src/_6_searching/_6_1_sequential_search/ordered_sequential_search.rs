/// 顺序查找
///
/// 在已排序数据集上
///
pub fn order_sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;
    let mut stop = false; // 控制遇到有序数据时退出

    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true;
        } else if num < nums[pos] {
            // 数据有序，退出
            stop = true;
        } else {
            pos += 1;
        }
    }

    found
}
