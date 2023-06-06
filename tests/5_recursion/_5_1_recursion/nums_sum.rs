use data_structures_and_algorithms_in_rust::_5_recursion::{nums_sum, nums_sum1, nums_sum2};

#[test]
fn nums_sum_test() {
    let nums = [2, 1, 7, 4, 5];

    let sum_loop = nums_sum(&nums);
    let sum_cur_1 = nums_sum1(&nums);
    let sum_cur_2 = nums_sum2(&nums);

    assert_eq!(19, sum_loop);
    assert_eq!(19, sum_cur_1);
    assert_eq!(19, sum_cur_2);
    assert_eq!(sum_cur_1, sum_cur_2);
}
