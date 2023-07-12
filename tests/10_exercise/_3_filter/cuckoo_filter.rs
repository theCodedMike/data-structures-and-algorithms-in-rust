use data_structures_and_algorithms_in_rust::_10_exercise::_3_filter::cuckoo_filter::CuckooFilter;
use std::collections::hash_map::DefaultHasher;

/// cargo test test_cuckoo_filter
#[test]
fn test_cuckoo_filter() {
    let mut bf = CuckooFilter::new();
    (0..20).for_each(|i| {
        let _ = bf.add(&i);
    });

    assert_eq!(bf.contains(&-10), false);
    assert_eq!(bf.contains(&-1), false);
    assert_eq!(bf.contains(&0), true);
    assert_eq!(bf.contains(&1), true);
    assert_eq!(bf.contains(&2), true);
    assert_eq!(bf.contains(&3), true);
    assert_eq!(bf.contains(&19), true);
    assert_eq!(bf.contains(&20), false);
    assert_eq!(bf.contains(&100), false);
}

/// cargo test test_cuckoo_filter2
#[test]
fn test_cuckoo_filter2() {
    let mut bf: CuckooFilter<DefaultHasher> = CuckooFilter::with_capacity(100);
    let _ = bf.add("hello");
    let _ = bf.add("rust");
    let _ = bf.add("word like a string");
    let _ = bf.add("f32 is short");
    let _ = bf.add("f64");
    let _ = bf.add("hashmap is awesome");
    let _ = bf.add("trait");
    let _ = bf.add("rust fuck all others");

    assert_eq!(bf.contains(""), false);
    assert_eq!(bf.contains("hello"), true);
    assert_eq!(bf.contains("hell"), false);
    assert_eq!(bf.contains("hashmap is awesome"), true);
    assert_eq!(bf.contains("hashmap"), false);
    assert_eq!(bf.contains("awesome"), false);
}
