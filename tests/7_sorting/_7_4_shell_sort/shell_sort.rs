use data_structures_and_algorithms_in_rust::_7_sorting::_7_4_the_shell_sort::shell_sort::shell_sort;
///
/// cargo test shell_sort_test
///
#[test]
fn shell_sort_test() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];

    shell_sort(&mut nums);

    assert_eq!(nums, [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]);
}
