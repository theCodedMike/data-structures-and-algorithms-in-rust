use data_structures_and_algorithms_in_rust::_7_sorting::_7_8_the_bucket_sort::bucket_sort::bucket_sort;

///
/// cargo test bucket_sort_test
///
#[test]
fn bucket_sort_test() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];

    bucket_sort(&mut nums, |t| t / 5);

    assert_eq!(nums, [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]);
}
