use data_structures_and_algorithms_in_rust::_9_graph::_9_3_depth_first_search::dfs;

/// cargo test -- --show-output dfs_test
#[test]
fn dfs_test() {
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

    let gp = dfs::create_graph(data);
    // [1] -> [2] [3]
    // [2] -> [1] [4] [5]
    // [3] -> [1] [6] [7]
    // [4] -> [2] [5]
    // [5] -> [2] [4] [8]
    // [6] -> [3] [7] [8]
    // [7] -> [3] [6]
    // [8] -> [5] [6]
    dfs::dfs(gp);
    // 1 -> 2 -> 4 -> 5 -> 8 -> 6 -> 3 -> 7 ->
}
