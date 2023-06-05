use data_structures_and_algorithms_in_rust::_4_basic_data_structures::_4_2_queue::Queue;

#[test]
fn queue_test() {
    let mut q = Queue::new(3);
    let _r1 = q.enqueue(1);
    let _r2 = q.enqueue(2);
    let _r3 = q.enqueue(3);

    assert_eq!(Err("No space available".to_string()), q.enqueue(4));

    assert_eq!(Some(1), q.dequeue());
    assert_eq!(2, q.size());
    assert_eq!(false, q.is_empty());

    assert_eq!(Some(2), q.dequeue());
    assert_eq!(Some(3), q.dequeue());
    assert_eq!(true, q.is_empty());

    assert_eq!(None, q.dequeue());
}
