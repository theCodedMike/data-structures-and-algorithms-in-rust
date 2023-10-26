use data_structures_and_algorithms_in_rust::_7_sorting::_7_5_the_merge_sort::{
    merge_sort::merge_sort, merge_sort_practice::my_merge_sort,
};

///
/// cargo test merge_sort_test
///
#[test]
fn merge_sort_test() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    merge_sort(&mut nums);
    assert_eq!(nums, [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]);
}

#[test]
fn my_merge_sort_test_1() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    my_merge_sort(&mut nums);
    assert_eq!(nums, [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]);
}

#[test]
fn my_merge_sort_test_2() {
    let mut nums = [54, 32, 99, 18, 75];
    my_merge_sort(&mut nums);
    assert_eq!(nums, [18, 32, 54, 75, 99]);
}

#[test]
fn my_merge_sort_test_3() {
    let mut nums = [18, 32, 54, 75, 99];
    my_merge_sort(&mut nums);
    assert_eq!(nums, [18, 32, 54, 75, 99]);
}

#[test]
fn my_merge_sort_test_4() {
    let mut nums = [18];
    my_merge_sort(&mut nums);
    assert_eq!(nums, [18]);
}

#[test]
fn my_merge_sort_test_5() {
    let mut nums = [18, 17, 16, 15];
    my_merge_sort(&mut nums);
    assert_eq!(nums, [15, 16, 17, 18]);
}

#[test]
fn my_merge_sort_test_6() {
    let mut nums = [18, 18, 18, 18];
    my_merge_sort(&mut nums);
    assert_eq!(nums, [18, 18, 18, 18]);
}
