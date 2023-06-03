use data_structures_and_algorithms_in_rust::_4_basic_data_structures::exercise::{
    DoubleListHeadStack, DoubleListTailStack, ExerciseStack, Peek, RefPeek, SingleListHeadStack,
    SingleListTailStack, VecHeadStack, VecTailStack,
};
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

#[test]
fn vec_head_stack_test() {
    let mut s = VecHeadStack::new();
    s.push(1); // [1]
    s.push(2); // [2, 1]
    s.push(4); // [4, 2, 1]
    assert_eq!(3, s.size());
    assert_eq!(false, s.is_empty());
    assert_eq!(Some(&4), s.peek());

    assert_eq!(Some(4), s.pop()); // [2, 1]
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop()); // [1]
    assert_eq!(Some(&mut 1), s.peek_mut());
    let top_ref = s.peek_mut().unwrap();
    *top_ref = 20;
    assert_eq!(Some(&20), s.peek());

    assert_eq!(Some(20), s.pop()); // []
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(None, s.peek_mut());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());
}

#[test]
fn vec_tail_stack_test() {
    let mut s = VecTailStack::new();
    s.push(1); // [1]
    s.push(2); // [1, 2]
    s.push(4); // [1, 2, 4]
    assert_eq!(3, s.size());
    assert_eq!(false, s.is_empty());
    assert_eq!(Some(&4), s.peek());

    assert_eq!(Some(4), s.pop()); // [1, 2]
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop()); // [1]
    assert_eq!(Some(&mut 1), s.peek_mut());
    let top_ref = s.peek_mut().unwrap();
    *top_ref = 20;
    assert_eq!(Some(&20), s.peek());

    assert_eq!(Some(20), s.pop()); // []
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(None, s.peek_mut());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());
}

#[test]
fn single_list_head_stack_test() {
    let mut s = SingleListHeadStack::new();
    s.push(1); // 1
    s.push(2); // 2 -> 1
    s.push(4); // 4 -> 2 -> 1
    assert_eq!(3, s.size());
    assert_eq!(false, s.is_empty());
    assert_eq!(Some(&4), s.peek());

    assert_eq!(Some(4), s.pop()); // 2 -> 1
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop()); // 1
    assert_eq!(Some(&mut 1), s.peek_mut());
    let top_ref = s.peek_mut().unwrap();
    *top_ref = 20;
    assert_eq!(Some(&20), s.peek());

    assert_eq!(Some(20), s.pop()); // None
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(None, s.peek_mut());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());

    s.push(5); // 5
    s.push(4); // 4 -> 5
    assert_eq!(2, s.size());
    assert_eq!(Some(&4), s.peek());
    assert_eq!(Some(&mut 4), s.peek_mut());

    assert_eq!(Some(4), s.pop());
    assert_eq!(Some(5), s.pop());
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(None, s.peek_mut());
}

#[test]
fn single_list_tail_stack_test() {
    let mut s = SingleListTailStack::new();
    s.push(1); // 1
    s.push(2); // 1 -> 2
    s.push(4); // 1 -> 2 -> 4
    assert_eq!(3, s.size());
    assert_eq!(false, s.is_empty());
    assert_eq!(Some(&4), s.peek());

    assert_eq!(Some(4), s.pop()); // 1 -> 2
    assert_eq!(2, s.size());
    assert_eq!(false, s.is_empty());

    assert_eq!(Some(2), s.pop()); // 1
    assert_eq!(Some(&mut 1), s.peek_mut());
    let top_ref = s.peek_mut().unwrap();
    *top_ref = 20;
    assert_eq!(Some(&20), s.peek());

    assert_eq!(Some(20), s.pop()); // None
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(None, s.peek_mut());
    assert_eq!(0, s.size());
    assert_eq!(true, s.is_empty());

    s.push(5); // 5
    s.push(4); // 5 -> 4
    assert_eq!(2, s.size());
    assert_eq!(Some(&4), s.peek());
    assert_eq!(Some(&mut 4), s.peek_mut());

    assert_eq!(Some(4), s.pop());
    assert_eq!(Some(5), s.pop());
    assert_eq!(None, s.pop());
    assert_eq!(None, s.peek());
    assert_eq!(None, s.peek_mut());
}

#[test]
fn double_list_head_stack_test() {
    let mut s = DoubleListHeadStack::new();
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

    if let Some(mut val) = s.peek_mut() {
        *val = 20;
    }
    assert_eq!(20, *s.peek().unwrap()); // Head -> 20 <- Tail

    assert_eq!(Some(20), s.pop()); // Head -> None <- Tail
    assert_eq!(None, s.pop());
    //  can't compare `Ref<'_, {integer}>` with `Ref<'_, {integer}>`
    //assert_eq!(None, s.peek());
    //assert_eq!(None, s.peek_mut());
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
fn double_list_tail_stack_test() {
    let mut s = DoubleListTailStack::new();
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

    if let Some(mut val) = s.peek_mut() {
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
