///
/// 基数排序
///
pub fn radix_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {
        return;
    }

    // 找到最大的数，它的位最多
    let max_num = match nums.iter().max() {
        None => return,
        Some(&x) => x,
    };
    // 找最接近且 >= nums 长度的2的次幂值作为桶大小，如：
    // 最接近且 >= 10 的2的次幂值是 2^4 = 16
    // 最接近且 >= 17 的2的次幂值是 2^5 = 32
    let radix = nums.len().next_power_of_two();

    // digit 代表小于某个位对应桶的所有数
    // 个、十、百、千分别为在1, 2, 3, 4位
    // 起始从个位开始, 所以是1
    let mut digit = 1;
    while digit <= max_num {
        // index_of计算数据在桶中的哪个位置
        let index_of = |x| x / digit % radix;

        // 计数器
        let mut counter = vec![0; radix];
        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }

        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        // 排序
        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }

        // 跨越桶
        digit *= radix;
    }
}
