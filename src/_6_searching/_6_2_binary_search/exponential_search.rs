use crate::_6_searching::binary_search1;

pub fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 {
        return false;
    }

    // 逐步找到上界
    let mut high = 1_usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }

    // 上界的一半一定可以作为下界
    let low = high >> 1;

    // 使用前面实现的二分查找
    binary_search1(&nums[low..size.min(high + 1)], target)
}
