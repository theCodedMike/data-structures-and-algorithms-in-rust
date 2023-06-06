use data_structures_and_algorithms_in_rust::_6_searching::{
    order_sequential_search, sequential_search, sequential_search_pos,
};

#[test]
fn sequential_search_test() {
    let num = 8;
    let nums = [9, 3, 7, 4, 1, 6, 2, 8, 5];
    let found = sequential_search(&nums, num);

    assert_eq!(true, found);
}

#[test]
fn sequential_search_pos_test() {
    let num = 8;
    let nums = [9, 3, 7, 4, 1, 6, 2, 8, 5];
    let found = sequential_search_pos(&nums, num);

    assert_eq!(Some(7), found);
}

#[test]
fn order_sequential_search_test() {
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];

    let num = 44;
    let found = order_sequential_search(&nums, num);
    assert_eq!(true, found);

    let num = 49;
    let found = order_sequential_search(&nums, num);
    assert_eq!(false, found);
}
