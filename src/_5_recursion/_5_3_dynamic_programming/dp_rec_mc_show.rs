pub fn dp_rec_mc_show(
    cashes: &[u32],
    amount: u32,
    min_cashes: &mut [u32],
    cashes_used: &mut [u32],
) -> u32 {
    for denm in 1..=amount {
        let mut min_cashes_num = denm;
        //收集使用过的各面额纸币，最小面额为1
        let mut used_cashes = 1;

        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<_>>() {
            let idx = (denm - c) as usize;
            let cashes_num = 1 + min_cashes[idx];

            if cashes_num < min_cashes_num {
                min_cashes_num = cashes_num;
                used_cashes = *c;
            }
        }

        // 更新各金额对应的最小纸币数
        min_cashes[denm as usize] = min_cashes_num;
        cashes_used[denm as usize] = used_cashes;
    }

    min_cashes[amount as usize]
}
/// 打印输出各面额纸币
pub fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("${}", curr);
        amount -= curr;
    }
}
