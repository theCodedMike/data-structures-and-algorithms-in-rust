use data_structures_and_algorithms_in_rust::_10_exercise::_3_filter::bloom_filter::BloomFilter;

/// cargo test test_bloom_filter
#[test]
fn test_bloom_filter() {
    let mut bf = BloomFilter::new(100, 0.08);
    (0..20).for_each(|i| bf.insert(&i));

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

/// cargo test test_bloom_filter2
#[test]
fn test_bloom_filter2() {
    let mut bf = BloomFilter::new(100, 0.08);
    bf.insert("hello");
    bf.insert("rust");
    bf.insert("word like a string");
    bf.insert("f32 is short");
    bf.insert("f64");
    bf.insert("hashmap is awesome");
    bf.insert("trait");
    bf.insert("rust fuck all others");

    assert_eq!(bf.contains(""), false);
    assert_eq!(bf.contains("hello"), true);
    assert_eq!(bf.contains("hell"), false);
    assert_eq!(bf.contains("hashmap is awesome"), true);
    assert_eq!(bf.contains("hashmap"), false);
    assert_eq!(bf.contains("awesome"), false);
}
