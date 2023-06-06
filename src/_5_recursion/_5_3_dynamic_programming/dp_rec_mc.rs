/// 使用最少的纸币/硬币来找零
///
/// 动态规划版
///
pub fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 动态收集从1到amount的最小找零币值数量
    // 然后从小到大凑出找零纸币数量
    for denm in 1..=amount {
        let mut min_cashes_num = denm;

        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<_>>() {
            let idx = (denm - c) as usize;

            let cashes_num = 1 + min_cashes[idx];

            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
            }
        }

        min_cashes[denm as usize] = min_cashes_num;
    }

    //因为收集了各个值的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}
