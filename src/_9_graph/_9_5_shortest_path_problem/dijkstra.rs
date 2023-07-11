use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vertex<'a> {
    pub name: &'a str,
}

impl<'a> Vertex<'a> {
    pub fn new(name: &'a str) -> Self {
        Vertex { name }
    }
}

/// 访问过的点
#[derive(Debug)]
struct Visit<V> {
    vertex: V,
    distance: usize, //距离
}

impl<V> PartialEq<Self> for Visit<V> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<V> Eq for Visit<V> {}

impl<V> PartialOrd<Self> for Visit<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// 为Visited添加全序比较功能
impl<V> Ord for Visit<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub fn dijkstra<'a>(
    start: Vertex<'a>,
    adj_list: &HashMap<Vertex<'a>, Vec<(Vertex<'a>, usize)>>,
) -> HashMap<Vertex<'a>, usize> {
    let mut distances = HashMap::new(); // 距离
    let mut visited = HashSet::new(); // 已访问过的点
    let mut to_visit = BinaryHeap::new(); // 待访问的点

    // 设置起始点和起始距离
    distances.insert(start, 0);
    to_visit.push(Visit {
        vertex: start,
        distance: 0,
    });

    while let Some(Visit { vertex, distance }) = to_visit.pop() {
        // 已访问过该点，继续下一个点
        if !visited.insert(vertex) {
            continue;
        }
        // 获取邻点
        if let Some(neighbors) = adj_list.get(&vertex) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(&neighbor)
                    .map_or(true, |curr| new_distance < *curr);
                // 若距离更近，则插入新距离和邻点
                if is_shorter {
                    distances.insert(*neighbor, new_distance);
                    to_visit.push(Visit {
                        vertex: *neighbor,
                        distance: new_distance,
                    });
                }
            }
        }
    }

    distances
}
