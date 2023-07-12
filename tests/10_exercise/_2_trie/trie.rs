use data_structures_and_algorithms_in_rust::_10_exercise::_2_trie::trie::Trie;

/// cargo test test_trie
#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.insert("box");
    trie.insert("insert");
    trie.insert("apple");
    trie.insert("appeal");

    assert_eq!(trie.search("apple"), true);
    assert_eq!(trie.search("APPLE"), true);
    assert_eq!(trie.search("apples"), false);
    assert_eq!(trie.search("APPLES"), false);
    assert_eq!(trie.search("app"), false);
    assert_eq!(trie.search("APP"), false);

    assert_eq!(trie.start_with("app"), true);
    assert_eq!(trie.start_with("APP"), true);
    assert_eq!(trie.start_with("ins"), true);
    assert_eq!(trie.start_with("INS"), true);
    assert_eq!(trie.start_with("ina"), false);
    assert_eq!(trie.start_with("Ina"), false);
    assert_eq!(trie.start_with("apple"), true);
    assert_eq!(trie.start_with("APPLE"), true);
}
