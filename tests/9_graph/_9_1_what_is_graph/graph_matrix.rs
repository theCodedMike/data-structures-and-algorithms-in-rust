use data_structures_and_algorithms_in_rust::_9_graph::_9_1_what_is_graph::graph_matrix::{
    Graph, Vertex,
};

/// cargo test -- --show-output graph_matrix_test
#[test]
fn graph_matrix_test() {
    let mut g = Graph::new(4);
    let n1 = Vertex::new(0, "n1");
    let n2 = Vertex::new(1, "n2");
    let n3 = Vertex::new(2, "n3");
    let n4 = Vertex::new(3, "n4");

    g.add_edge(&n1, &n2);
    g.add_edge(&n1, &n3);
    g.add_edge(&n2, &n3);
    g.add_edge(&n2, &n4);
    g.add_edge(&n3, &n4);
    g.add_edge(&n3, &n1);

    //      n1   n2   n3   n4
    //
    // n1        x    x
    //
    // n2             x    x
    //
    // n3   x              x
    //
    // n4

    assert_eq!(false, g.is_empty());
    assert_eq!(4, g.len());

    println!("{:?}", g);
    // [
    //   [Edge { edge: false }, Edge { edge: true  }, Edge { edge: true  }, Edge { edge: false }],
    //   [Edge { edge: false }, Edge { edge: false }, Edge { edge: true  }, Edge { edge: true  }],
    //   [Edge { edge: true  }, Edge { edge: false }, Edge { edge: false }, Edge { edge: true  }],
    //   [Edge { edge: false }, Edge { edge: false }, Edge { edge: false }, Edge { edge: false }]
    // ]
}
