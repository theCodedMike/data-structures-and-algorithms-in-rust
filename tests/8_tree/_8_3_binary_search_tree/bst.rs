use data_structures_and_algorithms_in_rust::_8_tree::_8_3_binary_search_tree::bst::BST;

#[test]
fn binary_search_tree_test() {
    let mut bst = BST::<i32, char>::new();

    bst.insert(8, 'e');
    bst.insert(6, 'c');
    bst.insert(7, 'd');
    bst.insert(5, 'b');
    bst.insert(10, 'g');
    bst.insert(9, 'f');
    bst.insert(11, 'h');
    bst.insert(4, 'a');
    //         8
    //       /   \
    //      6     10
    //     / \   /  \
    //    5   7 9    11
    //   /
    //  4
    assert_eq!(false, bst.is_empty());
    assert_eq!(8, bst.len());

    assert_eq!((Some(&11), Some(&'h')), bst.max());
    assert_eq!((Some(&4), Some(&'a')), bst.min());

    assert_eq!(Some(&'b'), bst.get(&5));
    assert_eq!(true, bst.search(&5));
    assert_eq!(false, bst.search(&20));

    println!("preorder: ");
    bst.preorder();
    // key:Some(8), val:Some('e')
    // key:Some(6), val:Some('c')
    // key:Some(5), val:Some('b')
    // key:Some(4), val:Some('a')
    // key:Some(7), val:Some('d')
    // key:Some(10), val:Some('g')
    // key:Some(9), val:Some('f')
    // key:Some(11), val:Some('h')

    println!(" inorder: ");
    bst.inorder();
    // key:Some(4), val:Some('a')
    // key:Some(5), val:Some('b')
    // key:Some(6), val:Some('c')
    // key:Some(7), val:Some('d')
    // key:Some(8), val:Some('e')
    // key:Some(9), val:Some('f')
    // key:Some(10), val:Some('g')
    // key:Some(11), val:Some('h')

    println!("postorder: ");
    bst.postorder();
    // key:Some(4), val:Some('a')
    // key:Some(5), val:Some('b')
    // key:Some(7), val:Some('d')
    // key:Some(6), val:Some('c')
    // key:Some(9), val:Some('f')
    // key:Some(11), val:Some('h')
    // key:Some(10), val:Some('g')
    // key:Some(8), val:Some('e')
}
