use data_structures_and_algorithms_in_rust::_6_searching::{
    binary_search1, binary_search2, exponential_search, interpolation_search,
};

#[test]
fn binary_search1_test() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

    let target = 3;
    let found = binary_search1(&nums, target);
    assert_eq!(true, found);

    let target = 63;
    let found = binary_search1(&nums, target);
    assert_eq!(false, found);
}

#[test]
fn binary_search2_test() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

    let target = 3;
    let found = binary_search2(&nums, target);
    assert_eq!(true, found);

    let target = 63;
    let found = binary_search2(&nums, target);
    assert_eq!(false, found);
}

#[test]
fn interpolation_search_test() {
    let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];

    let target = 27;
    let found = interpolation_search(&nums, target);
    assert_eq!(true, found);
}

#[test]
fn exponential_search_test() {
    let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];

    let target = 27;
    let found = exponential_search(&nums, target);
    assert_eq!(true, found);
}
