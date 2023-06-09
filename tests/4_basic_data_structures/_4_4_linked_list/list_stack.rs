use data_structures_and_algorithms_in_rust::_4_basic_data_structures::_4_4_linked_list::Stack;

#[test]
fn list_stack_test() {
    let mut s = Stack::new();
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());

    s.push(1);
    s.push(2);
    s.push(4);
    assert_eq!(3, s.size());
    assert_eq!(Some(&4), s.peek());

    assert_eq!(Some(4), s.pop());
    assert_eq!(Some(2), s.pop());
    assert_eq!(false, s.is_empty());
    assert_eq!(1, s.size());

    assert_eq!(Some(1), s.pop());
    assert_eq!(true, s.is_empty());
    assert_eq!(0, s.size());
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
}
