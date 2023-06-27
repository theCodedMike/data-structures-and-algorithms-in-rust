use crate::_4_basic_data_structures::_4_2_queue::Queue;
use std::cmp::{max, Ordering};
use std::fmt::{Debug, Display};

/// 子节点链接
type Link<T> = Option<Box<BinaryTree<T>>>;
/// 二叉树定义
///
/// key 保存数据
///
/// left和right保存左右子节点链接
#[derive(Debug, Clone)]
pub struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BinaryTree<T>
where
    T: Clone + Debug + Ord + Display,
{
    pub fn new(key: T) -> Self {
        BinaryTree {
            key,
            left: None,
            right: None,
        }
    }

    /// 新子节点作为根节点的左子节点
    pub fn insert_left_tree(&mut self, key: T) {
        let mut node = BinaryTree::new(key);

        if self.left.is_none() {
            self.left = Some(Box::new(node));
        } else {
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    /// 新子节点作为根节点的右子节点
    pub fn insert_right_tree(&mut self, key: T) {
        let mut node = BinaryTree::new(key);

        if self.right.is_none() {
            self.right = Some(Box::new(node));
        } else {
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    /// 获取左子节点，注意使用了clone
    pub fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    /// 获取右子节点
    pub fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    /// 获取根节点的值
    pub fn get_key(&self) -> T {
        self.key.clone()
    }

    pub fn set_key(&mut self, key: T) {
        self.key = key;
    }

    /// 前序遍历: 内部实现 - 递归版
    pub fn preorder(&self) {
        println!("key: {:?}", self.key);
        if let Some(left) = &self.left {
            left.preorder();
        }
        if let Some(right) = &self.right {
            right.preorder();
        }
    }

    /// 中序遍历: 内部实现 - 递归版
    pub fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        println!("key: {:?}", self.key);
        if let Some(right) = &self.right {
            right.inorder();
        }
    }

    /// 后序遍历: 内部实现 - 递归版
    pub fn postorder(&self) {
        if let Some(left) = &self.left {
            left.postorder();
        }
        if let Some(right) = &self.right {
            right.postorder();
        }
        println!("key: {:?}", self.key);
    }

    /// 层次遍历
    pub fn level_order(&self) {
        let size = self.size();
        let mut q = Queue::new(size);
        // 根节点入队
        let _ = q.enqueue(Box::new(self.clone()));
        while !q.is_empty() {
            let front = q.dequeue().unwrap();
            println!("key: {}", front.get_key());

            // 左右子节点入队
            if let Some(left) = front.get_left() {
                let _ = q.enqueue(left);
            }
            if let Some(right) = front.get_right() {
                let _ = q.enqueue(right);
            }
        }
    }

    /// 计算节点总数
    pub fn size(&self) -> usize {
        self.calc_size(0)
    }
    fn calc_size(&self, mut size: usize) -> usize {
        size += 1;
        if self.left.is_some() {
            size = self.left.as_ref().unwrap().calc_size(size);
        }
        if self.right.is_some() {
            size = self.right.as_ref().unwrap().calc_size(size);
        }
        size
    }

    /// 计算叶子节点个数
    pub fn leaf_size(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(left), None) => left.leaf_size(),
            (None, Some(right)) => right.leaf_size(),
            (Some(left), Some(right)) => left.leaf_size() + right.leaf_size(),
        }
    }

    /// 计算非叶子节点个数
    pub fn none_leaf_size(&self) -> usize {
        self.size() - self.leaf_size()
    }

    /// 计算树的深度
    pub fn depth(&self) -> usize {
        let mut left_depth = 1;
        if let Some(left) = &self.left {
            left_depth += left.depth();
        }

        let mut right_depth = 1;
        if let Some(right) = &self.right {
            right_depth += right.depth();
        }

        max(left_depth, right_depth)
    }

    /// 查找最小的key
    pub fn min(&self) -> Option<&T> {
        match &self.left {
            None => Some(&self.key),
            Some(node) => node.min(),
        }
    }

    /// 查找最大的key
    pub fn max(&self) -> Option<&T> {
        match &self.right {
            None => Some(&self.key),
            Some(node) => node.max(),
        }
    }

    /// 是否包含key
    pub fn contains(&self, key: &T) -> bool {
        match &self.key.cmp(key) {
            Ordering::Less => match self.right.as_ref() {
                None => false,
                Some(right) => right.contains(key),
            },
            Ordering::Equal => true,
            Ordering::Greater => match self.left.as_ref() {
                None => false,
                Some(left) => left.contains(key),
            },
        }
    }

    /// 中序遍历并输出括号
    pub fn get_exp(&self) -> String {
        let mut exp = "(".to_string();

        match (&self.left, &self.right) {
            (None, None) => exp += &self.get_key().to_string(),
            (Some(left), None) => {
                exp += &left.get_exp();
                exp += &self.get_key().to_string();
            }
            (None, Some(right)) => {
                exp += &self.get_key().to_string();
                exp += &right.get_exp();
            }
            (Some(left), Some(right)) => {
                exp += &left.get_exp();
                exp += &self.get_key().to_string();
                exp += &right.get_exp();
            }
        }

        exp += ")";
        exp
    }
}

/// 前序遍历: 外部实现 - 递归版
pub fn preorder<T: Clone + Debug + Ord + Display>(bt: Link<T>) {
    if let Some(node) = &bt {
        println!("key: {:?}", node.key);
        preorder(node.get_left());
        preorder(node.get_right());
    }
}

/// 中序遍历: 外部实现 - 递归版
pub fn inorder<T: Clone + Debug + Ord + Display>(bt: Link<T>) {
    if let Some(node) = &bt {
        inorder(node.get_left());
        println!("key: {:?}", node.key);
        inorder(node.get_right());
    }
}

/// 后序遍历: 外部实现 - 递归版
pub fn postorder<T: Clone + Debug + Ord + Display>(bt: Link<T>) {
    if let Some(node) = &bt {
        postorder(node.get_left());
        postorder(node.get_right());
        println!("key: {:?}", node.key);
    }
}

/// 中序遍历并输出括号
pub fn get_exp<T: Clone + Debug + Ord + Display>(bt: Link<T>) -> String {
    let mut exp = String::new();
    if let Some(node) = &bt {
        exp = "(".to_string() + &get_exp(node.get_left());
        exp += &node.key.to_string();
        exp += &(get_exp(node.get_right()) + ")");
    }
    exp
}
