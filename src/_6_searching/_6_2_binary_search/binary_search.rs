/// 二分查找
///
/// 迭代版
///
pub fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        let mid = (low + high) >> 1;
        // 若 low + high 溢出，可以转换为减法
        //let mid = low + ((high - low) >> 1);

        if num > nums[mid] {
            // num > 中间值，省去前半部数据
            low = mid + 1;
        } else if num < nums[mid] {
            // num < 中间值，省去后半部数据
            high = mid - 1;
        } else {
            found = true
        }
    }

    found
}

/// 二分查找
///
/// 递归版
///
pub fn binary_search2(nums: &[i32], num: i32) -> bool {
    // 基本情况1：项不存在
    if nums.len() == 0 {
        return false;
    }

    let mid = nums.len() >> 1;
    return if num > nums[mid] {
        binary_search2(&nums[mid + 1..], num)
    } else if num < nums[mid] {
        binary_search2(&nums[..mid - 1], num)
    } else {
        // 基本情况2：项存在
        true
    };
}
