///
/// 计数排序
///
/// nums = [0, 7, 1, 7, 3, 1, 5, 8, 4, 4, 5]
///
/// bucket count = nums_max - nums_min + 1 = 8 - 0 + 1 = 9
///
/// //         0  1  2  3  4  5  6  7  8
/// buckets = [1, 2, 0, 1, 2, 2, 0, 2, 1]
///
/// nums = [0, 1, 1, 3, 4, 4, 5, 5, 7, 7, 8]
///
pub fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {
        return;
    }

    // 桶数量为nums中最大值加1，保证数据都有桶放
    let max_bct_num = nums.iter().max().unwrap() + 1;
    let mut counter = vec![0; max_bct_num];
    for &v in nums.iter() {
        // 将数据标记到桶
        counter[v] += 1;
    }

    // 数据写回原nums切片
    let mut j = 0;
    for i in 0..max_bct_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}
