use data_structures_and_algorithms_in_rust::_9_graph::_9_2_breadth_first_search::bfs;

/// cargo test -- --show-output bfs_test
#[test]
fn bfs_test() {
    let data = [
        [1, 2],
        [2, 1],
        [1, 3],
        [3, 1],
        [2, 4],
        [4, 2],
        [2, 5],
        [5, 2],
        [3, 6],
        [6, 3],
        [3, 7],
        [7, 3],
        [4, 5],
        [5, 4],
        [6, 7],
        [7, 6],
        [5, 8],
        [8, 5],
        [6, 8],
        [8, 6],
    ];

    let gp = bfs::create_graph(data);
    // [1] -> [2] [3]
    // [2] -> [1] [4] [5]
    // [3] -> [1] [6] [7]
    // [4] -> [2] [5]
    // [5] -> [2] [4] [8]
    // [6] -> [3] [7] [8]
    // [7] -> [3] [6]
    // [8] -> [5] [6]
    bfs::bfs(gp);
    // 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 ->
}
