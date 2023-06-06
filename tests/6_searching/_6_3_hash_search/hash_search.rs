use data_structures_and_algorithms_in_rust::_6_searching::hash::{hash1, hash2};
use data_structures_and_algorithms_in_rust::_6_searching::hashmap::HashMap;

#[test]
fn hash1_test() {
    let size = 11;
    let s1 = "rust";
    let s2 = "Rust";
    let p1 = hash1(s1, size);
    let p2 = hash1(s2, size);
    // (114 + 117 + 115 + 116) % 11 = 0
    assert_eq!(0, p1);
    // ( 82 + 117 + 115 + 116) % 11 = 1
    assert_eq!(1, p2);
}

#[test]
fn hash2_test() {
    let size = 11;
    let s1 = "rust";
    let s2 = "Rust";
    let p1 = hash2(s1, size);
    let p2 = hash2(s2, size);
    // (114 * 1 + 117 * 2 + 115 * 3 + 116 * 4) % 11 = 2
    assert_eq!(2, p1);
    // ( 82 * 1 + 117 * 2 + 115 * 3 + 116 * 4) % 11 = 3
    assert_eq!(3, p2);
}

#[test]
fn hash_map_test() {
    let mut map = HashMap::new(11);
    map.insert(10, "cat");
    map.insert(2, "dog");
    map.insert(3, "tiger");

    assert_eq!(3, map.len());
    assert_eq!(true, map.contains(2));
    assert_eq!(Some(&"tiger"), map.get(3));
    assert_eq!(Some("tiger"), map.remove(3));
    assert_eq!(None, map.remove(3));
}
