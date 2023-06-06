pub fn nums_sum3(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return sum + nums[0];
    };
    // 使用sum来接收中间计算值
    nums_sum3(sum + nums[0], &nums[1..])
}

pub fn nums_sum4(sum: i32, nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return sum + nums[0];
    }
    nums_sum4(sum + nums[nums.len() - 1], &nums[..nums.len() - 1])
}
