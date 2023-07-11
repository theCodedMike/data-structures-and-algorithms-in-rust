#![allow(dead_code)]
/// 顶点定义
///
/// 基于邻接矩阵
#[derive(Debug)]
pub struct Vertex<'a> {
    id: usize,
    name: &'a str,
}

impl Vertex<'_> {
    pub fn new(id: usize, name: &'static str) -> Self {
        Vertex { id, name }
    }
}

/// 边定义
///
/// 基于邻接矩阵
#[derive(Debug, Clone)]
pub struct Edge {
    edge: bool, // 表示是否有边，并不需要构造一个边实体
}

impl Edge {
    pub fn new() -> Self {
        Edge { edge: false }
    }

    pub fn set_edge() -> Self {
        Edge { edge: true }
    }
}

/// 图定义
///
/// 基于邻接矩阵
#[derive(Debug)]
pub struct Graph {
    nodes: usize,
    graph: Vec<Vec<Edge>>, // 每个点的边放一个vec
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Graph {
            nodes,
            graph: vec![vec![Edge::new(); nodes]; nodes],
        }
    }

    pub fn len(&self) -> usize {
        self.nodes
    }

    pub fn is_empty(&self) -> bool {
        self.nodes == 0
    }

    /// 添加边，设置边属性为true
    pub fn add_edge(&mut self, f: &Vertex, s: &Vertex) {
        if f.id < self.nodes && s.id < self.nodes {
            self.graph[f.id][s.id] = Edge::set_edge();
        } else {
            panic!("error: vertex num is {}", self.nodes);
        }
    }
}
