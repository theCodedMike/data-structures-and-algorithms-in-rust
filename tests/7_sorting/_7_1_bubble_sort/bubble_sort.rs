use data_structures_and_algorithms_in_rust::_7_sorting::_7_1_the_bubble_sort::bubble_sort::{
    bubble_sort, bubble_sort2, bubble_sort3,
};

///
/// cargo test -- --show-output bubble_sort_test
///
#[test]
fn bubble_sort_test() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    bubble_sort(&mut nums);
    println!("{:?}", nums);
    // [17, 20, 26, 31, 44, 54, 55, 77, 93]
}

///
/// cargo test -- --show-output bubble_sort2_test
///
#[test]
fn bubble_sort2_test() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    bubble_sort2(&mut nums);
    println!("{:?}", nums);
    // [17, 20, 26, 31, 44, 54, 55, 77, 93]
}

///
/// cargo test -- --show-output bubble_sort3_test
///
#[test]
fn bubble_sort3_test() {
    let mut nums = [54, 26, 93, 17, 77, 31, 44, 55, 20];
    bubble_sort3(&mut nums);
    println!("{:?}", nums);
    // [17, 20, 26, 31, 44, 54, 55, 77, 93]
}
