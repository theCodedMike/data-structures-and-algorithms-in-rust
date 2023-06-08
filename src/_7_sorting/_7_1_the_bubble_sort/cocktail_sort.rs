/// 鸡尾酒排序
///
/// 从左到右采取升序排序，从右到左采取降序排序，这种双向排序法称为鸡尾酒排序
///
pub fn cocktail_sort(nums: &mut [i32]) {
    let len = nums.len();
    if len <= 1 {
        return;
    }

    // bubble控制是否继续冒泡
    let mut bubble = true;
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;

            // 从左到右冒泡
            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }

            // 从右到左冒泡
            for j in ((i + 1)..=(len - i - 1)).rev() {
                if nums[j - 1] > nums[j] {
                    nums.swap(j - 1, j);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    }
}
