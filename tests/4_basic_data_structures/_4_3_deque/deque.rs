use data_structures_and_algorithms_in_rust::_4_basic_data_structures::_4_3_deque::Deque;

#[test]
fn deque_test() {
    let mut d = Deque::new(4);
    let _r1 = d.add_front(1); //    1
    let _r2 = d.add_front(2); //    1 2
    let _r3 = d.add_rear(3); //   3 1 2
    let _r4 = d.add_rear(4); // 4 3 1 2

    assert_eq!(Err("No space available".to_string()), d.add_front(5));
    assert_eq!(false, d.is_empty());
    assert_eq!(4, d.size());

    assert_eq!(Some(4), d.remove_rear()); //  3 1 2
    assert_eq!(Some(2), d.remove_front()); // 3 1
    assert_eq!(Some(3), d.remove_rear()); //    1
    assert_eq!(Some(1), d.remove_front()); //
    assert_eq!(None, d.remove_rear());
    assert_eq!(None, d.remove_front());

    assert_eq!(0, d.size());
    assert_eq!(true, d.is_empty());
}
