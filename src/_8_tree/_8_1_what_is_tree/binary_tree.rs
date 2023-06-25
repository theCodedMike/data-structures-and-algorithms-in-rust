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

impl<T: Clone + Debug> BinaryTree<T> {
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
        if self.left.is_some() {
            self.left.as_ref().unwrap().preorder();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().preorder();
        }
    }

    /// 中序遍历: 内部实现 - 递归版
    pub fn inorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().inorder();
        }
        println!("key: {:?}", self.key);
        if self.right.is_some() {
            self.right.as_ref().unwrap().inorder();
        }
    }

    /// 后序遍历: 内部实现 - 递归版
    pub fn postorder(&self) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().postorder();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().postorder();
        }
        println!("key: {:?}", self.key);
    }
}

/// 前序遍历: 外部实现 - 递归版
pub fn preorder<T: Clone + Debug>(bt: Link<T>) {
    if bt.is_some() {
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}

/// 中序遍历: 外部实现 - 递归版
pub fn inorder<T: Clone + Debug>(bt: Link<T>) {
    if bt.is_some() {
        inorder(bt.as_ref().unwrap().get_left());
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}

/// 后序遍历: 外部实现 - 递归版
pub fn postorder<T: Clone + Debug>(bt: Link<T>) {
    if bt.is_some() {
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key is {:?}", bt.as_ref().unwrap().get_key());
    }
}

/// 中序遍历并输出括号
pub fn get_exp<T: Clone + Debug + Display>(bt: Link<T>) -> String {
    let mut exp = String::new();
    if bt.is_some() {
        exp = "(".to_string() + &get_exp(bt.as_ref().unwrap().get_left());
        exp += &bt.as_ref().unwrap().get_key().to_string();
        exp += &(get_exp(bt.as_ref().unwrap().get_right()) + ")");
    }
    exp
}
