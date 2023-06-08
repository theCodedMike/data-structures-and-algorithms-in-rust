use data_structures_and_algorithms_in_rust::_7_sorting::_7_7_the_heap_sort::heap_sort::heap_sort;

///
/// cargo test heap_sort_test
///
#[test]
fn heap_sort_test() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];

    heap_sort(&mut nums);

    assert_eq!(nums, [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]);
}
