use data_structures_and_algorithms_in_rust::_8_tree::_8_4_balanced_binary_search_tree::avl::AvlTree;

/// cargo test -- --show-output avl_test
#[test]
fn avl_test() {
    let mut avl = AvlTree::new();
    for i in 0..10 {
        avl.insert(i);
    }

    //         3
    //      /     \
    //     1       7
    //    / \     / \
    //   0   2   5   8
    //          / \   \
    //         4   6   9
    assert_eq!(false, avl.is_empty());
    assert_eq!(10, avl.len());
    assert_eq!(5, avl.leaf_len());
    assert_eq!(5, avl.non_leaf_len());
    assert_eq!(4, avl.depth());
    assert_eq!(true, avl.search(&9));
    assert_eq!(Some(&0), avl.min());
    assert_eq!(Some(&9), avl.max());

    println!("preorder: ");
    avl.preorder();
    // val: 3
    // val: 1
    // val: 0
    // val: 2
    // val: 7
    // val: 5
    // val: 4
    // val: 6
    // val: 8
    // val: 9

    println!(" inorder: ");
    avl.inorder();
    // val: 0
    // val: 1
    // val: 2
    // val: 3
    // val: 4
    // val: 5
    // val: 6
    // val: 7
    // val: 8
    // val: 9

    println!("postorder: ");
    avl.postorder();
    // val: 0
    // val: 2
    // val: 1
    // val: 4
    // val: 6
    // val: 5
    // val: 9
    // val: 8
    // val: 7
    // val: 3

    println!("level order: ");
    avl.level_order();
    // val: 3
    // val: 1
    // val: 7
    // val: 0
    // val: 2
    // val: 5
    // val: 8
    // val: 4
    // val: 6
    // val: 9
}
