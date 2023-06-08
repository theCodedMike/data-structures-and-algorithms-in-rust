///
pub fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if nums.len() <= 1 {
        return;
    }

    if low < high {
        let split = partition(nums, low, high);
        if split > 1 {
            // 防止越界(split <= 1)和语法错误
            quick_sort(nums, low, split - 1);
        }
        quick_sort(nums, split + 1, high);
    }
}
///
/// 分区函数 partition，算法中取第一项做中枢值
///
/// 首先右移左标记，直到找到一个大于等于中枢值的值。
/// 然后左移递减右标记，直到找到小于等于中枢值的值。
/// 如果左标记值大于右标记值就交换值。
/// 然后重复该过程。直到左右标记相互越过对方。
/// 比较越过后左右标记值，若右小于左，则将此时右标记值和中枢值交换，
/// 右标记值作为分裂点，将集合分为左右两个区间，然后在左右区间递归调用快速排序，直到最后完成排序
///
fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low; //  左标记
    let mut rm = high; // 右标记

    loop {
        // 左标记不断右移
        while lm <= rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 右标记不断左移
        while lm <= rm && nums[rm] >= nums[low] {
            rm -= 1;
        }
        // 左标记越过右标记时退出并交换左右标记数据
        if lm > rm {
            break;
        } else {
            nums.swap(lm, rm);
        }
    }

    nums.swap(low, rm);

    rm
}
