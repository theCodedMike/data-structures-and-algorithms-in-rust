use data_structures_and_algorithms_in_rust::_4_basic_data_structures::stack::{MyStack, Stack};

#[test]
fn stack_test() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(4);
    assert_eq!(3, s.size());
    assert_eq!(Some(&4), s.peek());

    assert_eq!(Some(4), s.pop());
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop());
    assert_eq!(Some(1), s.pop());
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());
}

#[test]
fn custom_stack_test() {
    let mut s = MyStack::new();
    s.push(1);
    s.push(2);
    s.push(4);
    assert_eq!(3, s.size());
    assert_eq!(Some(&4), s.peek());

    assert_eq!(Some(4), s.pop());
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop());
    assert_eq!(Some(1), s.pop());
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());
}
