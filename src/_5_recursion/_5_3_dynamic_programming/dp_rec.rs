/// 斐波那契数列
///
/// 递归版
///
pub fn fibonacci_rec(n: u32) -> u32 {
    match n {
        1 | 2 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}

/// 斐波那契数列
///
/// 动态规划版
///
pub fn fibonacci_dp(n: u32) -> u32 {
    // 只用2个位置来保存值，节约内存
    let mut dp = [1, 1];

    for i in 2..n {
        let idx1 = (i % 2) as usize;
        let idx2 = ((i - 1) % 2) as usize;
        let idx3 = ((i - 2) % 2) as usize;

        dp[idx1] = dp[idx2] + dp[idx3];
    }

    dp[((n - 1) % 2) as usize]
}
