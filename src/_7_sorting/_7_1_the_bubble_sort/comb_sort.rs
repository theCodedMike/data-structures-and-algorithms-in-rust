pub fn comb_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    let mut i;
    let mut gap = len;
    // 大致排序，数据基本有序
    while gap > 0 {
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < len {
            if nums[i - gap] > nums[i] {
                nums.swap(i - gap, i);
            }
            i += 1;
        }
    }

    // 细致调节部分无序数据，exchange控制是否继续交换数据
    let mut exchange = true;
    while exchange {
        exchange = false;
        i = 0;
        while i < len - 1 {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                exchange = true;
            }
            i += 1;
        }
    }
}
