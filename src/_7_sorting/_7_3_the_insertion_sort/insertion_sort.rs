///
/// 插入排序
///
pub fn insertion_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    for i in 1..len {
        let mut pos = i;
        let curr = nums[pos];

        while pos > 0 && curr < nums[pos - 1] {
            // 向后移动数据
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }

        // 插入数据
        nums[pos] = curr;
    }
}
