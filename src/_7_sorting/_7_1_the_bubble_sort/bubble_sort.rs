///
/// 冒泡排序
///
/// for循环版
///
pub fn bubble_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len < 2 {
        return;
    }
    // 外层循环用来控制遍历的次数
    for i in 1..len {
        for j in 0..len - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}
///
/// 冒泡排序
///
/// while循环版
///
pub fn bubble_sort2(nums: &mut [i32]) {
    let mut len = nums.len();
    if len < 2 {
        return;
    }

    while len > 0 {
        for j in 0..len - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }

        len -= 1;
    }
}
///
/// 冒泡排序
///
/// 优化: 如果数据本来就有序，仅遍历一次
///
pub fn bubble_sort3(nums: &mut [i32]) {
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        compare = false;
        for j in 0..len {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                // 数据无序时，才进行比较
                compare = true;
            }
        }

        len -= 1;
    }
}
