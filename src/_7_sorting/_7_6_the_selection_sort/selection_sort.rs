///
/// 选择排序
///
pub fn selection_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    // 待排序数据下标
    let mut left = len - 1;
    while left > 0 {
        let mut pos_max = 0;

        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                // 选择当前轮次最大值下标
                pos_max = i;
            }
        }

        // 数据交换，完成一个数据的排序，待排序数据量减1
        nums.swap(pos_max, left);
        left -= 1;
    }
}
