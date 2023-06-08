pub fn bin_insertion_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..len {
        // 已排序数组左边界
        left = 0;
        // 已排序数组右边界
        right = i - 1;
        // 待排序数组
        temp = nums[i];

        // 二分法找到temp的位置
        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                if mid == 0 {
                    // 防止出现right = 0 - 1
                    break;
                }
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        // 将数据后移，留出空位
        for j in (left..=i - 1).rev() {
            nums.swap(j, j + 1);
        }

        // 将temp插入空位
        if left != i {
            nums[left] = temp;
        }
    }
}
