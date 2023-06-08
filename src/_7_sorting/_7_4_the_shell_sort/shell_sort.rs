///
/// 希尔排序
///
pub fn shell_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    // 每次让gap减少一半直到为1
    let mut gap = len / 2;
    while gap > 0 {
        for start in 0..gap {
            ist_sort(nums, start, gap);
        }

        gap /= 2;
    }
}

// 插入排序函数(内部)，数据相隔距离为gap
fn ist_sort(nums: &mut [i32], start: usize, gap: usize) {
    let mut i = start + gap;

    while i < nums.len() {
        let mut pos = i;
        let curr = nums[pos];

        while pos >= gap && curr < nums[pos - gap] {
            nums[pos] = nums[pos - gap];
            pos -= gap;
        }

        nums[pos] = curr;
        i += gap;
    }
}
