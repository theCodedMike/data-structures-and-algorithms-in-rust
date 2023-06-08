use data_structures_and_algorithms_in_rust::_7_sorting::_7_2_the_quick_sort::quick_sort::quick_sort1;
///
/// cargo test -- --show-output quick_sort_test
///
#[test]
fn quick_sort_test() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    let len = nums.len();
    quick_sort1(&mut nums, 0, len - 1);
    println!("{:?}", nums);
    // [17, 20, 26, 31, 44, 54, 55, 77, 93]
}
