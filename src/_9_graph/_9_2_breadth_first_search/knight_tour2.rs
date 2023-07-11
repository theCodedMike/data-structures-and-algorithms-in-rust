#![allow(unused)]
use crate::_9_graph::_9_1_what_is_graph::graph_adjlist::Graph;

fn kinght_graph(bdsize: u32) -> Graph<u32> {
    let mut g = Graph::new();

    for row in 0..bdsize {
        for col in 0..bdsize {
            let node_id = pos_2_nod_id(row, col, bdsize);
            let pos = gen_legal_move(row, col, bdsize);
            for e in pos {
                let n_id = pos_2_nod_id(e.0, e.1, bdsize);
                g.add_edge(&node_id, &n_id, 0);
            }
        }
    }

    g
}

fn pos_2_nod_id(row: u32, col: u32, bdsize: u32) -> u32 {
    row * bdsize + col
}

fn gen_legal_move(x: u32, y: u32, bdsize: u32) -> Vec<(u32, u32)> {
    let mut moves = vec![];

    // 马移动是坐标变化，共8个方向
    let move_offsets = [
        (-1, -2),
        (-1, 2),
        (-2, -1),
        (-2, 1),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];
    for offset in move_offsets {
        let new_x = (x as i32 + offset.0) as u32;
        let new_y = (y as i32 + offset.1) as u32;
        if legal_coord(new_x, bdsize) && legal_coord(new_y, bdsize) {
            moves.push((new_x, new_y));
        }
    }

    moves
}

fn legal_coord(idx: u32, bdsize: u32) -> bool {
    if idx < bdsize {
        true
    } else {
        false
    }
}

/// depth: 走过的路径长度  
/// path: 保存访问过的点
/// u: 起始点
///
/// 这里的代码有问题
///
fn kinght_tour(depth: u32, path: &mut Vec<u32>, u: &mut Vec<u32>, limit: u32) -> bool {
    /*u.set_color("gray");
    //path.push(u);
    let mut done = false;
    if depth < limit {
        let nbrs = u.get_connects();
        let mut i = 0;
        while i < nbrs.len() && !done {
            if nbrs[i].get_color() == "white" {
                done = kinght_tour(depth + 1, path, nbrs[i], limit);
            }
            i += 1;
        }
        if !done {
            let _rm = path.pop();
            u.set_color("white");
        }
    } else {
        done = true;
    }
    done*/
    true
}
