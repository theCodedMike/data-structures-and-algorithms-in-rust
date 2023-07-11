#![allow(dead_code)]
use std::collections::HashMap;
use std::hash::Hash;

/// 顶点定义
///
/// 基于邻接表
#[derive(Debug)]
pub struct Vertex<T> {
    key: T,
    connects: Vec<(T, i32)>, // 邻点集合
}

impl<T> Vertex<T>
where
    T: PartialEq,
{
    pub fn new(key: T) -> Self {
        Vertex {
            key,
            connects: vec![],
        }
    }

    /// 判断key与当前节点是否相邻
    pub fn adjacent_key(&self, key: &T) -> bool {
        self.connects.iter().any(|(nbr, _wt)| nbr == key)
    }

    /// 新指向某个顶点
    pub fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.connects.push((nbr, wt));
    }

    /// 获取相邻的点集合
    pub fn get_connects(&self) -> Vec<&T> {
        self.connects.iter().map(|(nbr, _wt)| nbr).collect()
    }

    /// 返回某个邻点的边权重
    pub fn get_nbr_weight(&self, key: &T) -> Option<i32> {
        self.connects
            .iter()
            .filter(|(nbr, _wt)| nbr == key)
            .map(|(_nbr, wt)| *wt)
            .nth(0)
    }
}

/// 图定义
///
/// 基于邻接表
#[derive(Debug)]
pub struct Graph<T> {
    vert_nums: usize,                           // 点数
    edge_nums: usize,                           // 边数
    pub(crate) vertices: HashMap<T, Vertex<T>>, // 点集合
}

impl<T> Graph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Graph {
            vert_nums: 0,
            edge_nums: 0,
            vertices: HashMap::new(),
        }
    }

    /// 判空
    pub fn is_empty(&self) -> bool {
        self.vert_nums == 0
    }

    /// 获取顶点总数
    pub fn vertex_nums(&self) -> usize {
        self.vert_nums
    }

    /// 获取边的总数
    pub fn edge_nums(&self) -> usize {
        self.edge_nums
    }

    /// 是否包含某个顶点
    pub fn contains(&self, key: &T) -> bool {
        self.vertices.contains_key(key)
    }

    /// 添加顶点
    pub fn add_vertex(&mut self, key: &T) {
        let vertex = Vertex::new(key.clone());
        self.vert_nums += 1;
        self.vertices.insert(key.clone(), vertex);
    }

    /// 获取顶点
    pub fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        self.vertices.get(key)
    }

    /// 获取所有顶点的key
    pub fn vertex_keys(&self) -> Vec<&T> {
        self.vertices.keys().collect()
    }

    /// 删除某个顶点(同时要删除边)
    pub fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        if let Some(old_vert) = self.vertices.remove(key) {
            self.vert_nums -= 1;
            // 删除从当前顶点出发的边
            self.edge_nums -= old_vert.connects.len();
            // 删除到当前顶点的边
            self.vertices.values_mut().for_each(|v| {
                if v.adjacent_key(key) {
                    v.connects.retain(|(nbr, _wt)| nbr != key);
                    self.edge_nums -= 1;
                }
            });
            return Some(old_vert);
        }

        None
    }

    /// 添加边
    pub fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        // 若顶点不存在则要先添加顶点
        if !self.contains(from) {
            self.add_vertex(from);
        }
        if !self.contains(to) {
            self.add_vertex(to);
        }

        // 添加边
        self.edge_nums += 1;
        let _ = self
            .vertices
            .get_mut(from)
            .map(|v| v.add_neighbor(to.clone(), wt));
    }

    /// 判断两个顶点是否相邻
    pub fn adjacent(&self, from: &T, to: &T) -> bool {
        // 判断顶点是否存在
        if !self.contains(from) {
            return false;
        }
        if !self.contains(to) {
            return false;
        }

        self.vertices
            .get(from)
            .map_or(false, |v| v.adjacent_key(to))
    }
}
