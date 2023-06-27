use crate::_4_basic_data_structures::_4_2_queue::Queue;
use std::cmp::{max, Ordering};
use std::fmt::Debug;

/// 二叉查找树子节点链接
type Link<T, U> = Option<Box<BST<T, U>>>;

///
/// 二叉查找树定义
///
pub struct BST<T, U> {
    key: Option<T>,
    val: Option<U>,
    left: Link<T, U>,
    right: Link<T, U>,
}

impl<T, U> BST<T, U>
where
    T: Clone + Ord + Debug,
    U: Clone + Debug,
{
    pub fn new() -> Self {
        BST {
            key: None,
            val: None,
            left: None,
            right: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.key.is_none()
    }

    /// 查看元素个数
    ///
    /// O(n)
    pub fn len(&self) -> usize {
        self.calc_len(0)
    }

    /// 递归计算节点个数
    fn calc_len(&self, mut i: usize) -> usize {
        if self.key.is_none() {
            return i;
        }

        // 当前节点加入总节点树i
        i += 1;
        // 计算左子节点数
        if self.left.is_some() {
            i = self.left.as_ref().unwrap().calc_len(i);
        }
        // 计算右子节点数
        if self.right.is_some() {
            i = self.right.as_ref().unwrap().calc_len(i);
        }

        i
    }

    /// 前序遍历 - 递归版
    ///
    /// O(n)
    pub fn preorder(&self) {
        println!("key:{:?}, val:{:?}", &self.key, &self.val);
        if let Some(left) = &self.left {
            left.preorder();
        }
        if let Some(right) = &self.right {
            right.preorder();
        }
    }

    /// 中序遍历 - 递归版
    ///
    /// O(n)
    pub fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        println!("key:{:?}, val:{:?}", &self.key, &self.val);
        if let Some(right) = &self.right {
            right.inorder();
        }
    }

    /// 后序遍历 - 递归版
    ///
    /// O(n)
    pub fn postorder(&self) {
        if let Some(left) = &self.left {
            left.postorder();
        }
        if let Some(right) = &self.right {
            right.postorder();
        }
        println!("key:{:?}, val:{:?}", &self.key, &self.val);
    }

    /// 插入新节点 - 递归版
    ///
    /// O(log2(n))
    pub fn insert(&mut self, key: T, val: U) {
        match &self.key {
            None => {
                // 无数据则直接插入
                self.key = Some(key);
                self.val = Some(val);
            }
            Some(k) => {
                // 存在key，更新val
                if key == *k {
                    self.val = Some(val);
                    return;
                }
                // 未找到key，需插入新节点
                // 先找到需要插入的子树
                let child = if key < *k {
                    &mut self.left
                } else {
                    &mut self.right
                };
                // 根据节点递归下去，直到插入
                match child {
                    None => {
                        let mut node = BST::new();
                        node.insert(key, val);
                        *child = Some(Box::new(node));
                    }
                    Some(node) => {
                        node.insert(key, val);
                    }
                }
            }
        }
    }

    /// 查找某个key是否存在 - 递归版
    ///
    /// O(log2(n))
    pub fn search(&self, key: &T) -> bool {
        match &self.key {
            None => false,
            Some(k) => {
                // 比较key值，并判断是否继续递归
                match key.cmp(k) {
                    Ordering::Less => {
                        // 在左子树查找
                        match &self.left {
                            None => false,
                            Some(node) => node.search(key),
                        }
                    }
                    Ordering::Equal => true,
                    Ordering::Greater => {
                        // 在右子树查找
                        match &self.right {
                            None => false,
                            Some(node) => node.search(key),
                        }
                    }
                }
            }
        }
    }

    /// 查看树中最小的key
    pub fn min(&self) -> (Option<&T>, Option<&U>) {
        // 最小值一定在最左侧
        match &self.left {
            None => match &self.key {
                None => (None, None),
                Some(key) => (Some(key), self.val.as_ref()),
            },
            Some(node) => node.min(),
        }
    }

    /// 查看树中最大的key
    pub fn max(&self) -> (Option<&T>, Option<&U>) {
        // 最大值一定在最右侧
        match &self.right {
            None => match &self.key {
                None => (None, None),
                Some(key) => (Some(key), self.val.as_ref()),
            },
            Some(node) => node.max(),
        }
    }

    /// 根据key获取val
    ///
    /// O(log2(n))
    pub fn get(&self, key: &T) -> Option<&U> {
        match &self.key {
            None => None,
            Some(k) => match key.cmp(k) {
                Ordering::Less => {
                    // 在左子树查找
                    match &self.left {
                        None => None,
                        Some(node) => node.get(key),
                    }
                }
                Ordering::Equal => self.val.as_ref(),
                Ordering::Greater => {
                    // 在右子树查找
                    match &self.right {
                        None => None,
                        Some(node) => node.get(key),
                    }
                }
            },
        }
    }

    /// 计算叶子节点个数
    pub fn leaf_len(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(left), None) => left.leaf_len(),
            (None, Some(right)) => right.leaf_len(),
            (Some(left), Some(right)) => left.leaf_len() + right.leaf_len(),
        }
    }

    pub fn none_leaf_len(&self) -> usize {
        self.len() - self.leaf_len()
    }

    /// 计算树的深度
    pub fn depth(&self) -> usize {
        let mut left_depth = 0;
        if let Some(left) = &self.left {
            left_depth += left.depth();
        }
        let mut right_depth = 0;
        if let Some(right) = &self.right {
            right_depth += right.depth();
        }

        max(left_depth, right_depth) + 1
    }

    pub fn contains(&self, key: &T) -> bool {
        match &self.key {
            None => false,
            Some(k) => match key.cmp(k) {
                Ordering::Less => {
                    // 在左子树查找
                    match &self.left {
                        None => false,
                        Some(node) => node.contains(key),
                    }
                }
                Ordering::Equal => true,
                Ordering::Greater => {
                    // 在右子树查找
                    match &self.right {
                        None => false,
                        Some(node) => node.contains(key),
                    }
                }
            },
        }
    }

    /// 层次遍历
    pub fn level_order(&self) {
        if self.is_empty() {
            return;
        }
        let mut q = Queue::new(self.len());
        let _ = q.enqueue(self);

        while !q.is_empty() {
            if let Some(node) = q.dequeue() {
                println!("key:{:?}, val:{:?}", node.key, node.val);

                if let Some(ref left) = node.left {
                    let _ = q.enqueue(left);
                }
                if let Some(ref right) = node.right {
                    let _ = q.enqueue(right);
                }
            };
        }
    }
}
