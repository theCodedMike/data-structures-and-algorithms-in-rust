/// 内插查找
pub fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut low = 0;
    let mut high = nums.len() - 1;
    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        if high <= low || high_val < target || target < low_val {
            break;
        }

        // 计算插值位置
        let offset = (target - low_val) * (high - low) as i32 / (high_val - low_val);
        let interpolator = low + offset as usize;

        // 更新上下界 high、low
        if target > nums[interpolator] {
            low = interpolator + 1;
        } else if target < nums[interpolator] {
            high = interpolator - 1;
        } else {
            break;
        }
    }

    // 判断最终确定的上界是否是target
    if target == nums[high] {
        true
    } else {
        false
    }
}
