use data_structures_and_algorithms_in_rust::_9_graph::_9_1_what_is_graph::graph_adjlist::Graph;

/// cargo test -- --show-output graph_adjlist_test
#[test]
fn graph_adjlist_test() {
    let mut g = Graph::new();

    for i in 0..6 {
        g.add_vertex(&i);
    }
    let mut vertices = g.vertex_keys();
    vertices.sort_unstable();
    println!("{:?}", vertices); // [0, 1, 2, 3, 4, 5]

    assert_eq!(false, g.is_empty());
    assert_eq!(true, g.contains(&0));
    assert_eq!(true, g.contains(&1));
    assert_eq!(true, g.contains(&2));
    assert_eq!(true, g.contains(&3));
    assert_eq!(true, g.contains(&4));
    assert_eq!(true, g.contains(&5));
    assert_eq!(false, g.contains(&6));
    assert_eq!(6, g.vertex_nums());
    assert_eq!(0, g.edge_nums());

    g.add_edge(&0, &1, 5);
    g.add_edge(&0, &5, 2);
    g.add_edge(&1, &2, 4);
    g.add_edge(&2, &3, 9);
    g.add_edge(&3, &4, 7);
    g.add_edge(&3, &5, 3);
    g.add_edge(&4, &0, 1);
    g.add_edge(&5, &1, 8);
    // 0  [(1, 5), (5, 2)]
    // 1  [(2, 4)]
    // 2  [(3, 9)]
    // 3  [(4, 7), (5, 3)]
    // 4  [(0, 1)]
    // 5  [(1, 8)]
    assert_eq!(8, g.edge_nums());
    g.vertex_keys().iter().for_each(|v| {
        println!("{}: {:?}", v, g.get_vertex(v));
    });
    // 0: Some(Vertex { key: 0, connects: [(1, 5), (5, 2)] })
    // 1: Some(Vertex { key: 1, connects: [(2, 4)] })
    // 2: Some(Vertex { key: 2, connects: [(3, 9)] })
    // 3: Some(Vertex { key: 3, connects: [(4, 7), (5, 3)] })
    // 4: Some(Vertex { key: 4, connects: [(0, 1)] })
    // 5: Some(Vertex { key: 5, connects: [(1, 8)] })

    let vertex = g.get_vertex(&0).unwrap();
    let conns = vertex.get_connects();
    println!("{:?}", conns); // [1, 5]
    assert_eq!(Some(5), vertex.get_nbr_weight(&1));
    assert_eq!(Some(2), vertex.get_nbr_weight(&5));
    assert_eq!(None, vertex.get_nbr_weight(&3));

    assert_eq!(true, g.adjacent(&0, &1));
    assert_eq!(true, g.adjacent(&0, &5));
    assert_eq!(false, g.adjacent(&3, &2));
    assert_eq!(false, g.adjacent(&4, &3));

    let rm = g.remove_vertex(&0).unwrap();
    println!("{:?}", rm);
    assert_eq!(5, g.vertex_nums());
    assert_eq!(5, g.edge_nums());
    assert_eq!(false, g.contains(&0));
    g.vertex_keys().iter().for_each(|v| {
        println!("{}: {:?}", v, g.get_vertex(v));
    });
    // 1: Some(Vertex { key: 1, connects: [(2, 4)] })
    // 2: Some(Vertex { key: 2, connects: [(3, 9)] })
    // 3: Some(Vertex { key: 3, connects: [(4, 7), (5, 3)] })
    // 4: Some(Vertex { key: 4, connects: [] })
    // 5: Some(Vertex { key: 5, connects: [(1, 8)] })
}
