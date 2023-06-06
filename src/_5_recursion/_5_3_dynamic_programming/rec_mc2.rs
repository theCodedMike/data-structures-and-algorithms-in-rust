/// 使用最少的纸币/硬币来找零
///
/// 递归版
///
/// 空间换时间，使用min_cashes存储中间值
pub fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    // 全用1元纸币的最小找零纸币数量
    let mut min_cashes_num = amount;

    if cashes.contains(&amount) {
        // 收集和当前待找零值相同的币种
        min_cashes[amount as usize] = 1;
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        // 找零值amount有最小找零纸币数，直接返回
        return min_cashes[amount as usize];
    } else {
        for c in cashes.iter().filter(|c| **c <= amount).collect::<Vec<_>>() {
            let cashes_num = 1 + rec_mc2(cashes, amount - c, min_cashes);
            // 更新最小找零纸币种
            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                min_cashes[amount as usize] = min_cashes_num;
            }
        }
    }

    min_cashes_num
}
