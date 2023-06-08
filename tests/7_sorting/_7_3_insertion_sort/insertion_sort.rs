use data_structures_and_algorithms_in_rust::_7_sorting::_7_3_the_insertion_sort::bin_insertion_sort::bin_insertion_sort;
use data_structures_and_algorithms_in_rust::_7_sorting::_7_3_the_insertion_sort::insertion_sort::insertion_sort;
///
/// cargo test insertion_sort_test
///
#[test]
fn insertion_sort_test() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21];

    insertion_sort(&mut nums);

    assert_eq!(nums, [18, 21, 31, 32, 43, 54, 56, 75, 99]);
}

///
/// cargo test bin_insertion_sort_test
///
#[test]
fn bin_insertion_sort_test() {
    let mut nums = [1, 3, 2, 8, 6, 4, 9, 7, 5, 10];

    bin_insertion_sort(&mut nums);

    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
