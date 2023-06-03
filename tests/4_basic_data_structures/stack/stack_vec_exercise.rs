use data_structures_and_algorithms_in_rust::_4_basic_data_structures::exercise::{
    ExerciseStack, Peek, VecHeadStack, VecTailStack,
};

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
