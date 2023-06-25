use data_structures_and_algorithms_in_rust::_8_tree::_8_2_binary_heap::binary_heap::BinaryHeap;

#[test]
fn binary_heap_test() {
    let mut heap = BinaryHeap::new();
    let nums = [-1, 0, 2, 3, 4];

    heap.push(10);
    assert_eq!(heap, [10][..]);
    heap.push(9);
    assert_eq!(heap, [9, 10][..]);
    heap.push(8);
    assert_eq!(heap, [8, 10, 9][..]);
    heap.push(7);
    assert_eq!(heap, [7, 8, 9, 10][..]);
    heap.push(6);
    assert_eq!(heap, [6, 7, 9, 10, 8][..]);

    heap.build_add(&nums);
    assert_eq!(heap, [-1, 2, 0, 3, 4, 9, 6, 10, 7, 8][..]);

    assert_eq!(heap.is_empty(), false);
    assert_eq!(heap.min(), Some(-1));
    assert_eq!(heap.pop(), Some(-1));
    assert_eq!(heap, [0, 2, 6, 3, 4, 9, 8, 10, 7][..]);

    heap.build_add(&nums);
    assert_eq!(heap, [-1, 0, 2, 3, 0, 3, 4, 10, 7, 4, 2, 9, 6, 8][..]);
}
