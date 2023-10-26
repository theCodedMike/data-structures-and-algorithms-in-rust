use data_structures_and_algorithms_in_rust::_7_sorting::_7_2_the_quick_sort::{
    quick_sort::quick_sort, quick_sort_practice::my_quick_sort,
};

///
/// cargo test quick_sort_test
///
#[test]
fn quick_sort_test() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    let len = nums.len();

    quick_sort(&mut nums, 0, len - 1);

    assert_eq!(nums, [17, 20, 26, 31, 44, 54, 55, 77, 93]);
}

#[test]
fn my_quick_sort_test_1() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    my_quick_sort(&mut nums);
    assert_eq!(nums, [17, 20, 26, 31, 44, 54, 55, 77, 93]);
}

#[test]
fn my_quick_sort_test_2() {
    let mut nums = [54, 32, 99, 18, 75];
    my_quick_sort(&mut nums);
    assert_eq!(nums, [18, 32, 54, 75, 99]);
}

#[test]
fn my_quick_sort_test_3() {
    let mut nums = [18, 32, 54, 75, 99];
    my_quick_sort(&mut nums);
    assert_eq!(nums, [18, 32, 54, 75, 99]);
}

#[test]
fn my_quick_sort_test_4() {
    let mut nums = [18];
    my_quick_sort(&mut nums);
    assert_eq!(nums, [18]);
}

#[test]
fn my_quick_sort_test_5() {
    let mut nums = [18, 17, 16, 15];
    my_quick_sort(&mut nums);
    assert_eq!(nums, [15, 16, 17, 18]);
}

#[test]
fn my_quick_sort_test_6() {
    let mut nums = [18, 18, 18, 18];
    my_quick_sort(&mut nums);
    assert_eq!(nums, [18, 18, 18, 18]);
}
