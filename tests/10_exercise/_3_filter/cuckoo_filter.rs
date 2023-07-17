use data_structures_and_algorithms_in_rust::_10_exercise::_3_filter::cuckoo_filter::CuckooFilter;
use std::collections::hash_map::DefaultHasher;

/// cargo test test_cuckoo_filter
#[test]
fn test_cuckoo_filter() {
    let mut cf = CuckooFilter::new();
    (0..20).for_each(|i| {
        let _ = cf.add(&i);
    });

    assert_eq!(cf.contains(&-10), false);
    assert_eq!(cf.contains(&-1), false);
    assert_eq!(cf.contains(&0), true);
    assert_eq!(cf.contains(&1), true);
    assert_eq!(cf.contains(&2), true);
    assert_eq!(cf.contains(&3), true);
    assert_eq!(cf.contains(&19), true);
    assert_eq!(cf.contains(&20), false);
    assert_eq!(cf.contains(&100), false);

    cf.delete(&0);
    assert_eq!(cf.contains(&0), false);
}

/// cargo test test_cuckoo_filter2
#[test]
fn test_cuckoo_filter2() {
    let mut cf: CuckooFilter<DefaultHasher> = CuckooFilter::with_capacity(100);
    let _ = cf.add("hello");
    let _ = cf.add("rust");
    let _ = cf.add("word like a string");
    let _ = cf.add("f32 is short");
    let _ = cf.add("f64");
    let _ = cf.add("hashmap is awesome");
    let _ = cf.add("trait");
    let _ = cf.add("rust fuck all others");

    assert_eq!(cf.contains(""), false);
    assert_eq!(cf.contains("hello"), true);
    assert_eq!(cf.contains("hell"), false);
    assert_eq!(cf.contains("hashmap is awesome"), true);
    assert_eq!(cf.contains("hashmap"), false);
    assert_eq!(cf.contains("awesome"), false);

    assert_eq!(cf.contains("rust"), true);
    cf.delete("rust");
    assert_eq!(cf.contains("rust"), false);
}
