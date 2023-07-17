use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

/// 默认节点个数
const DEFAULT_REPLICAS: usize = 10;

/// 环上节点，代表机器
#[derive(Debug, Clone)]
pub struct Node {
    pub host: &'static str,
    pub ip: &'static str,
    pub port: u16,
}

impl Node {
    pub fn new(host: &'static str, ip: &'static str, port: u16) -> Self {
        Node { host, ip, port }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

/// 环
#[derive(Debug)]
pub struct Ring<T> {
    // replicas 是为防止结点聚集而导致数据也集中存储到少量结点上
    replicas: usize,        // 分区数
    ring: BTreeMap<u64, T>, // 保存数据的环
}

impl<T> Ring<T>
where
    T: Debug + Display + Clone,
{
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_REPLICAS)
    }

    pub fn with_capacity(replicas: usize) -> Self {
        Ring {
            replicas,
            ring: BTreeMap::new(),
        }
    }

    /// 批量插入节点
    pub fn add_multi(&mut self, nodes: &[T]) {
        if !nodes.is_empty() {
            for node in nodes {
                self.add(node);
            }
        }
    }

    pub fn add(&mut self, node: &T) {
        for i in 0..self.replicas {
            let key = hash(&format!("{}{}", node, i));
            self.ring.insert(key, node.clone());
        }
    }

    /// 批量删除节点
    pub fn remove_multi(&mut self, nodes: &[T]) {
        for node in nodes {
            self.remove(node);
        }
    }

    pub fn remove(&mut self, node: &T) {
        if !self.ring.is_empty() {
            for i in 0..self.replicas {
                let key = hash(&format!("{}{}", node, i));
                self.ring.remove(&key);
            }
        }
    }

    /// 查询节点
    pub fn get(&self, key: u64) -> Option<&T> {
        if self.ring.is_empty() {
            return None;
        }
        let mut keys = self.ring.keys();
        keys.find(|&k| k >= &key)
            .and_then(|k| self.ring.get(k))
            .or(keys.nth(0).and_then(|x| self.ring.get(x)))
    }

    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }
}

/// 哈希计算函数
pub fn hash<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}
