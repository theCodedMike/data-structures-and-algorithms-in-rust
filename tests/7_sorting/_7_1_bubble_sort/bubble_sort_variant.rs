use data_structures_and_algorithms_in_rust::_7_sorting::_7_1_the_bubble_sort::cant_believe_it_can_sort::{cbic_sort1, cbic_sort2};
use data_structures_and_algorithms_in_rust::_7_sorting::_7_1_the_bubble_sort::cocktail_sort::cocktail_sort;
use data_structures_and_algorithms_in_rust::_7_sorting::_7_1_the_bubble_sort::comb_sort::comb_sort;

///
/// cargo test cocktail_sort_test
///
#[test]
fn cocktail_sort_test() {
    let mut nums = [1, 3, 2, 8, 3, 6, 4, 9, 5, 10, 6, 7];

    cocktail_sort(&mut nums);

    assert_eq!(nums, [1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9, 10]);
}

///
/// cargo test comb_sort_test
///
#[test]
fn comb_sort_test() {
    let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];

    comb_sort(&mut nums);

    assert_eq!(nums, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

///
/// cargo test cbic_sort1_test
///
#[test]
fn cbic_sort1_test() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];

    cbic_sort1(&mut nums);

    assert_eq!(nums, [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]);
}

///
/// cargo test cbic_sort2_test
///
#[test]
fn cbic_sort2_test() {
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];

    cbic_sort2(&mut nums);

    assert_eq!(nums, [18, 21, 22, 31, 32, 43, 54, 56, 75, 99]);
}
