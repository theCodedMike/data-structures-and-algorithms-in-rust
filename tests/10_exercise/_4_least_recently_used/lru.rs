use data_structures_and_algorithms_in_rust::_10_exercise::_4_least_recently_used::lru::LRUCache;

/// cargo test -- --show-output test_lru
#[test]
fn test_lru() {
    let mut lru = LRUCache::with_capacity(5);
    lru.insert("first", 1);
    lru.insert("second", 2);
    lru.insert("third", 3);
    lru.insert("fourth", 4);
    lru.insert("fifth", 5);

    println!("{:?}", lru);
    // LRUCache {
    //     cap: 5,
    //     head: Some(4),
    //     tail: Some(0),
    //     map: {"first": 0, "second": 1, "third": 2, "fourth": 3, "fifth": 4},
    //     entries: [
    //         Entry { key: "first",  val: Some(1), next: Some(1), prev: None },
    //         Entry { key: "second", val: Some(2), next: Some(2), prev: Some(0) },
    //         Entry { key: "third",  val: Some(3), next: Some(3), prev: Some(1) },
    //         Entry { key: "fourth", val: Some(4), next: Some(4), prev: Some(2) },
    //         Entry { key: "fifth",  val: Some(5), next: None,    prev: Some(3) }
    //     ]
    // }
    //
    // tail                           head
    //  |                              |
    //  v                              v
    // first  second  third  fourth  fifth
    //  0       1       2       3      4
    assert_eq!(lru.len(), 5);
    assert_eq!(lru.is_empty(), false);
    assert_eq!(lru.is_full(), true);

    lru.insert("third", 88);
    println!("{:?}", lru);
    assert_eq!(lru.len(), 5);
    assert_eq!(lru.is_empty(), false);
    assert_eq!(lru.is_full(), true);
    // LRUCache {
    //     cap: 5,
    //     head: Some(2),
    //     tail: Some(0),
    //     map: {"first": 0, "second": 1, "third": 2, "fourth": 3, "fifth": 4},
    //     entries: [
    //         Entry { key: "first",  val: Some(1),  next: Some(1), prev: None },
    //         Entry { key: "second", val: Some(2),  next: Some(3), prev: Some(0) },
    //         Entry { key: "third",  val: Some(88), next: None,    prev: Some(4) },
    //         Entry { key: "fourth", val: Some(4),  next: Some(4), prev: Some(1) },
    //         Entry { key: "fifth",  val: Some(5),  next: Some(2), prev: Some(3) }
    //     ]
    // }
    // tail                           head
    //  |                              |
    //  v                              v
    // first  second  fourth  fifth  third
    //  0      1       3       4       2

    lru.insert("first", 100);
    println!("{:?}", lru);
    assert_eq!(lru.len(), 5);
    assert_eq!(lru.is_empty(), false);
    assert_eq!(lru.is_full(), true);
    // LRUCache {
    //     cap: 5,
    //     head: Some(0),
    //     tail: Some(0),
    //     map: {"first": 0, "second": 1, "third": 2, "fourth": 3, "fifth": 4},
    //     entries: [
    //         Entry { key: "first",  val: Some(100),  next: None,    prev: Some(2) },
    //         Entry { key: "second", val: Some(2),    next: Some(3), prev: None },
    //         Entry { key: "third",  val: Some(88),   next: Some(0), prev: Some(4) },
    //         Entry { key: "fourth", val: Some(4),    next: Some(4), prev: Some(1) },
    //         Entry { key: "fifth",  val: Some(5),    next: Some(2), prev: Some(3) }
    //     ]
    // }
    // tail                            head
    //  |                               |
    //  v                               v
    // second  fourth  fifth  third  first
    //  1       3       4      2       0
}
