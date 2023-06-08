pub fn cbic_sort1(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    for i in 0..len {
        for j in 0..len {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

pub fn cbic_sort2(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    for i in 1..len {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}
