use data_structures_and_algorithms_in_rust::_8_tree::_8_1_what_is_tree::binary_tree::BinaryTree;

///
/// cargo test -- --show-output binary_tree_test
///
#[test]
fn binary_tree_test() {
    let mut bt = BinaryTree::new('a');

    let root = bt.get_key();
    assert_eq!(root, 'a');

    let left = bt.get_left();
    println!(" left child is {:?}", left); // None
    let right = bt.get_right();
    println!("right child is {:?}", right); // None

    bt.insert_left_tree('b');
    bt.insert_right_tree('e');

    let left = bt.get_left();
    println!(" left child is {:#?}", left);
    // Some(
    //     BinaryTree {
    //         key: 'b',
    //         left: None,
    //         right: None,
    //     },
    // )
    let right = bt.get_right();
    println!("right child is {:#?}", right);
    // Some(
    //     BinaryTree {
    //         key: 'e',
    //         left: None,
    //         right: None,
    //     },
    // )
}
