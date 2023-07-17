use data_structures_and_algorithms_in_rust::_10_exercise::_5_hash::conshash;
use data_structures_and_algorithms_in_rust::_10_exercise::_5_hash::conshash::{Node, Ring};

/// cargo test -- --show-output test_conshash
#[test]
fn test_conshash() {
    let replica = 3;
    let mut ring = Ring::with_capacity(replica);

    let node1 = Node::new("localhost1", "127.0.0.1", 23);
    ring.add(&node1);

    for i in 0..replica {
        let key = conshash::hash(&format!("{}{}", node1, i));
        let res = ring.get(key);
        assert_eq!(node1.host, res.unwrap().host);
    }
    println!("{:?}", ring);
    // Ring {
    //     replicas: 3,
    //     ring: {
    //            14996679801985760: Node { host: "localhost1", ip: "127.0.0.1", port: 23 },
    //          3024318707304079879: Node { host: "localhost1", ip: "127.0.0.1", port: 23 },
    //         15436196400020236321: Node { host: "localhost1", ip: "127.0.0.1", port: 23 }
    //     }
    // }

    let node2 = Node::new("localhost2", "127.0.0.2", 24);
    ring.add(&node2);
    let node3 = Node::new("localhost3", "127.0.0.3", 25);
    ring.add(&node3);
    let node4 = Node::new("localhost4", "127.0.0.4", 26);
    ring.add(&node4);
    let node5 = Node::new("localhost5", "127.0.0.5", 27);
    ring.add(&node5);
    println!("{:?}", ring);
    // Ring {
    //     replicas: 3,
    //     ring: {
    //            14996679801985760: Node { host: "localhost1", ip: "127.0.0.1", port: 23 },
    //          3024318707304079879: Node { host: "localhost1", ip: "127.0.0.1", port: 23 },
    //          3828307200184318242: Node { host: "localhost2", ip: "127.0.0.2", port: 24 },
    //          4314886693892024231: Node { host: "localhost5", ip: "127.0.0.5", port: 27 },
    //          4632678885051857333: Node { host: "localhost4", ip: "127.0.0.4", port: 26 },
    //          7158208341941200607: Node { host: "localhost5", ip: "127.0.0.5", port: 27 },
    //          8323175647977836540: Node { host: "localhost3", ip: "127.0.0.3", port: 25 },
    //         11257022674360639988: Node { host: "localhost4", ip: "127.0.0.4", port: 26 },
    //         12344407851791457645: Node { host: "localhost2", ip: "127.0.0.2", port: 24 },
    //         13513584697926927684: Node { host: "localhost4", ip: "127.0.0.4", port: 26 },
    //         14166330909176487401: Node { host: "localhost3", ip: "127.0.0.3", port: 25 },
    //         15143714800284639914: Node { host: "localhost3", ip: "127.0.0.3", port: 25 },
    //         15436196400020236321: Node { host: "localhost1", ip: "127.0.0.1", port: 23 },
    //         16719345092227915165: Node { host: "localhost5", ip: "127.0.0.5", port: 27 },
    //         17654520010738620068: Node { host: "localhost2", ip: "127.0.0.2", port: 24 }
    //     }
    // }
    let nodes = vec![node1, node2, node3, node4, node5];
    ring.remove_multi(&nodes);
    assert_eq!(true, ring.is_empty());
}
