use data_structures_and_algorithms_in_rust::_4_basic_data_structures::exercise::{
    DoubleListHeadRawPtrStack, DoubleListTailRawPtrStack, ExerciseStack, Peek,
};

#[test]
fn double_list_head_raw_ptr_stack_test() {
    let mut s = DoubleListHeadRawPtrStack::new();
    s.push(1); // Head -> 1 <- Tail
    s.push(2); // Head -> 2 ⇆ 1 <- Tail
    s.push(4); // Head -> 4 ⇆ 2 ⇆ 1 <- Tail
    assert_eq!(3, s.size());
    assert_eq!(false, s.is_empty());
    assert_eq!(4, *s.peek().unwrap());

    assert_eq!(Some(4), s.pop()); // Head -> 2 ⇆ 1 <- Tail
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop()); // Head -> 1 <- Tail
    assert_eq!(1, *s.peek().unwrap());

    if let Some(val) = s.peek_mut() {
        *val = 20;
    }
    assert_eq!(20, *s.peek().unwrap()); // Head -> 20 <- Tail

    assert_eq!(Some(20), s.pop()); // Head -> None <- Tail
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(None, s.peek_mut());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());

    s.push(10); // Head -> 10 <- Tail
    s.push(20); // Head -> 20 ⇆ 10 <- Tail
    s.push(30); // Head -> 30 ⇆ 20 ⇆ 10 <- Tail
    assert_eq!(3, s.size());
    assert_eq!(30, *s.peek().unwrap());
    assert_eq!(30, *s.peek_mut().unwrap());

    assert_eq!(Some(30), s.pop()); // Head -> 20 ⇆ 10 <- Tail
    assert_eq!(Some(20), s.pop()); // Head -> 10 <- Tail
    assert_eq!(Some(10), s.pop()); // Head -> None <- Tail
    assert_eq!(None, s.pop());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());
}

#[test]
fn double_list_tail_raw_ptr_stack_test() {
    let mut s = DoubleListTailRawPtrStack::new();
    s.push(1); // Head -> 1 <- Tail
    s.push(2); // Head -> 1 ⇆ 2 <- Tail
    s.push(4); // Head -> 1 ⇆ 2 ⇆ 4 <- Tail
    assert_eq!(3, s.size());
    assert_eq!(false, s.is_empty());
    assert_eq!(4, *s.peek().unwrap());

    assert_eq!(Some(4), s.pop()); // Head -> 1 ⇆ 2 <- Tail
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop()); // Head -> 1 <- Tail
    assert_eq!(1, *s.peek().unwrap());

    if let Some(val) = s.peek_mut() {
        *val = 20;
    }
    assert_eq!(20, *s.peek().unwrap()); // Head -> 20 <- Tail

    assert_eq!(Some(20), s.pop()); // Head -> None <- Tail
    assert_eq!(None, s.pop());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());

    s.push(10); // Head -> 10 <- Tail
    s.push(20); // Head -> 10 ⇆ 20 <- Tail
    s.push(30); // Head -> 10 ⇆ 20 ⇆ 30 <- Tail
    assert_eq!(3, s.size());
    assert_eq!(30, *s.peek().unwrap());
    assert_eq!(30, *s.peek_mut().unwrap());

    assert_eq!(Some(30), s.pop()); // Head -> 10 ⇆ 20 <- Tail
    assert_eq!(Some(20), s.pop()); // Head -> 10 <- Tail
    assert_eq!(Some(10), s.pop()); // Head -> None <- Tail
    assert_eq!(None, s.pop());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());
}
