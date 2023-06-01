use data_structures_and_algorithms_in_rust::_4_basic_data_structures::linked_list::List;

#[test]
fn linked_list_basic_test() {
    let mut list = List::new();
    assert_eq!(true, list.is_empty());
    assert_eq!(0, list.size());

    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(Some(3), list.pop());
    assert_eq!(Some(&2), list.peek());
    assert_eq!(Some(&mut 2), list.peek_mut());

    list.peek_mut().map(|val| {
        *val = 4;
    });
    assert_eq!(Some(&4), list.peek());

    assert_eq!(Some(4), list.pop());
    assert_eq!(Some(1), list.pop());
    assert_eq!(None, list.pop());
    assert_eq!(true, list.is_empty());
    assert_eq!(0, list.size());
}

#[test]
fn linked_list_into_iter_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(3, list.size());

    let mut iter = list.into_iter();
    assert_eq!(Some(3), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(1), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn linked_list_iter_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(3, list.size());

    let mut iter = list.iter();
    assert_eq!(Some(&3), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&1), iter.next());
    assert_eq!(None, iter.next());
}

#[test]
fn linked_list_iter_mut_test() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(3, list.size());

    let mut iter = list.iter_mut();
    assert_eq!(Some(&mut 3), iter.next());
    assert_eq!(Some(&mut 2), iter.next());
    assert_eq!(Some(&mut 1), iter.next());
    assert_eq!(None, iter.next());
}
