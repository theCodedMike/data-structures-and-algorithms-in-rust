/// 求和-迭代版
pub fn nums_sum(nums: &[i32]) -> i32 {
    let mut sum = 0;

    for num in nums {
        sum += num;
    }

    sum
}

/// 求和-递归版1
pub fn nums_sum1(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let first = nums[0];
    first + nums_sum1(&nums[1..])
}

/// 求和-递归版2
pub fn nums_sum2(nums: &[i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let last = nums[nums.len() - 1];
    nums_sum2(&nums[..nums.len() - 1]) + last
}
