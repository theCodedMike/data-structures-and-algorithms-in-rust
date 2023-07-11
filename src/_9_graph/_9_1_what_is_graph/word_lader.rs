#![allow(dead_code)]
#![allow(unused)]

use crate::_4_basic_data_structures::_4_2_queue::Queue;
use crate::_9_graph::_9_1_what_is_graph::graph_adjlist::{Graph, Vertex};
use std::collections::HashMap;

pub fn build_word_graph(words: &[String]) -> Graph<String> {
    let mut d = HashMap::new();
    for word in words {
        for i in 0..word.len() {
            let bucket = word[..i].to_string() + "_" + &word[i + 1..];
            let wd = word.to_string();
            if d.contains_key(&bucket) {
                d.get_mut(&bucket).map(|v: &mut Vec<String>| v.push(wd));
            } else {
                d.insert(bucket, vec![wd]);
            }
        }
    }
    let mut g = Graph::new();
    for bucket in d.keys() {
        for wd_outer in &d[bucket] {
            for wd_inner in &d[bucket] {
                if wd_outer != wd_inner {
                    g.add_edge(wd_outer, wd_inner, 1);
                }
            }
        }
    }

    g
}

fn word_lader(
    mut g: Graph<String>,
    mut start: Vertex<String>,
    end: Vertex<String>,
    len: usize,
) -> u32 {
    // start.set_distance(0);
    // start.set_pred(None);

    // let mut vertex_queue = Queue::new(len);
    // let _ = vertex_queue.enqueue(start);
    //
    // while !vertex_queue.is_empty() {
    //     let mut currv = vertex_queue.dequeue().unwrap();
    //     for nbr in currv.get_connects() {
    //         let nbv = g.vertices.get_mut(nbr).unwrap();
    //         if 0 == nbv.color {
    //             nbv.set_color(1);
    //             nbv.set_distance(currv.dist + 1);
    //             nbv.set_pred(Some(currv.key.clone()));
    //             let v = g.vertices.get(nbr).unwrap().clone();
    //             let _r = vertex_queue.enqueue(v);
    //         }
    //     }
    //     currv.set_color(2);
    // }
    //
    // g.vertices.get(&end.key).unwrap().dist

    0
}
