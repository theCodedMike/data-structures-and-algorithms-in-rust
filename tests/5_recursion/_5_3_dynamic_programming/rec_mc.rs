use data_structures_and_algorithms_in_rust::_5_recursion::{
    dp_rec_mc, dp_rec_mc_show, fibonacci_dp, fibonacci_rec, print_cashes, rec_mc1, rec_mc2,
};

#[test]
fn rec_mc1_test() {
    // cashes保存各种面额的纸币
    let cashes = [1, 5, 10, 20, 50];
    let amount = 31_u32;
    let cashes_num = rec_mc1(&cashes, amount);

    assert_eq!(3, cashes_num); // 1 + 10 + 20
}

#[test]
fn rec_mc2_test() {
    let cashes = [1, 5, 10, 20, 50];
    let amount = 81_u32;
    let mut min_cashes = [0_u32; 82];
    let cashes_num = rec_mc2(&cashes, amount, &mut min_cashes);

    assert_eq!(4, cashes_num); // 1 + 10 + 20 + 50
}

#[test]
fn dp_rec_mc_test() {
    let cashes = [1, 5, 10, 20, 50];
    let amount = 81_u32;
    let mut min_cashes = [0_u32; 82];
    let cashes_num = dp_rec_mc(&cashes, amount, &mut min_cashes);

    assert_eq!(4, cashes_num); // 1 + 10 + 20 + 50
}

/// cargo test -- --show-output dp_rec_mc_show_test
#[test]
fn dp_rec_mc_show_test() {
    let cashes = [1, 5, 10, 20, 50];
    let amount = 81_u32;
    let mut min_cashes = [0_u32; 82];
    let mut cashes_used = [0_u32; 82];

    let cashes_num = dp_rec_mc_show(&cashes, amount, &mut min_cashes, &mut cashes_used);
    assert_eq!(4, cashes_num);

    print_cashes(&cashes_used, amount);
    // $1
    // $10
    // $20
    // $50
}

#[test]
fn fibonacci_dp_rec_test() {
    assert_eq!(55, fibonacci_rec(10));
    assert_eq!(55, fibonacci_dp(10));
}
