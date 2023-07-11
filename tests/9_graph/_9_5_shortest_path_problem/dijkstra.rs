use data_structures_and_algorithms_in_rust::_9_graph::_9_5_shortest_path_problem::dijkstra;
use data_structures_and_algorithms_in_rust::_9_graph::_9_5_shortest_path_problem::dijkstra::Vertex;
use std::collections::HashMap;

/// cargo test -- --show-output dijkstra_test
#[test]
fn dijkstra_test() {
    let s = Vertex::new("s");
    let t = Vertex::new("t");
    let x = Vertex::new("x");
    let y = Vertex::new("y");
    let z = Vertex::new("z");

    let mut adj_list = HashMap::new();
    adj_list.insert(s, vec![(t, 10), (y, 5)]);
    adj_list.insert(t, vec![(y, 2), (x, 1)]);
    adj_list.insert(x, vec![(z, 4)]);
    adj_list.insert(y, vec![(t, 3), (x, 9), (z, 2)]);
    adj_list.insert(z, vec![(s, 7), (x, 6)]);

    let distances = dijkstra::dijkstra(s, &adj_list);
    for (v, d) in distances.iter() {
        println!("{} to {}, min distance: {}", s.name, v.name, d);
    }
    // s to t, min distance: 8
    // s to y, min distance: 5
    // s to x, min distance: 11
    // s to s, min distance: 0
    // s to z, min distance: 7

    assert_eq!(distances.get(&t), Some(&8));
    assert_eq!(distances.get(&s), Some(&0));
    assert_eq!(distances.get(&y), Some(&5));
    assert_eq!(distances.get(&x), Some(&11));
    assert_eq!(distances.get(&z), Some(&7));
}
