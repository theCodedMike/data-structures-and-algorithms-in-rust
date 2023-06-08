use data_structures_and_algorithms_in_rust::_7_sorting::_7_8_the_bucket_sort::bucket_sort::bucket_sort;

///
/// cargo test bucket_sort_test
///
#[test]
fn bucket_sort_test() {
    let mut nums = [80, 92, 66, 56, 44, 31, 70, 20, 24];

    bucket_sort(&mut nums, |t| t / 5);

    assert_eq!(nums, [20, 24, 31, 44, 56, 66, 70, 80, 92]);
}
