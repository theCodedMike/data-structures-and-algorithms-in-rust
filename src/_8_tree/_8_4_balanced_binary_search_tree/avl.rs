use crate::_4_basic_data_structures::_4_2_queue::Queue;
use crate::_8_tree::_8_4_balanced_binary_search_tree::avl::AvlTree::{Null, Tree};
use std::cmp::{max, Ordering};
use std::fmt::Debug;

///
/// Avl树定义
///
#[derive(Debug)]
pub enum AvlTree<T> {
    Null, // 空树
    Tree(Box<AvlNode<T>>),
}
///
/// Avl树节点定义
///
#[derive(Debug)]
pub struct AvlNode<T> {
    val: T,
    left: AvlTree<T>,  // left subtree
    right: AvlTree<T>, // right subtree
    bfactor: i8,       // balance factor
}
impl<T> AvlNode<T> {
    fn new(val: T) -> Self {
        AvlNode {
            val,
            left: Null,
            right: Null,
            bfactor: 0,
        }
    }
}

impl<T> AvlTree<T>
where
    T: Ord + Debug,
{
    pub fn new() -> AvlTree<T> {
        Null
    }

    pub fn insert(&mut self, val: T) -> (bool, bool) {
        let ret = match *self {
            Null => {
                // 没有节点，则直接插入
                let node = AvlNode::new(val);
                *self = Tree(Box::new(node));
                (true, true)
            }
            Tree(ref mut node) => {
                // 比较节点值，再判断从哪边插入
                // inserted 表示是否插入
                // deepened 表示是否加深
                match val.cmp(&node.val) {
                    Ordering::Less => {
                        // 比节点数据小，插入左子树
                        let (inserted, deepened) = node.left.insert(val);
                        if deepened {
                            let ret = match node.bfactor {
                                -1 | 1 => (inserted, false),
                                0 => (inserted, true),
                                _ => unreachable!(),
                            };
                            node.bfactor -= 1;

                            ret
                        } else {
                            (inserted, deepened)
                        }
                    }
                    Ordering::Equal => {
                        // 相等，无需插入
                        (false, false)
                    }
                    Ordering::Greater => {
                        // 比节点数据大，插入右子树
                        let (inserted, deepened) = node.right.insert(val);
                        if deepened {
                            let ret = match node.bfactor {
                                -1 | 1 => (inserted, false),
                                0 => (inserted, true),
                                _ => unreachable!(),
                            };
                            node.bfactor += 1;

                            ret
                        } else {
                            (inserted, deepened)
                        }
                    }
                }
            }
        };
        self.rebalance();

        ret
    }

    /// 调整各节点的平衡因子
    fn rebalance(&mut self) {
        match *self {
            Null => {
                // 没数据，不用调整
            }
            Tree(_) => match self.node().bfactor {
                // 右子树重
                -2 => {
                    let lbf = self.node().left.node().bfactor;
                    if lbf == -1 || lbf == 0 {
                        let (a, b) = if lbf == -1 { (0, 0) } else { (-1, 1) };
                        self.rotate_right(); // 不平衡，旋转
                        self.node().right.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if lbf == 1 {
                        let (a, b) = match self.node().left.node().right.node().bfactor {
                            -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0, -1),
                            _ => unreachable!(),
                        };

                        // 先左旋再右旋
                        self.node().left.rotate_left();
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().left.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!()
                    }
                }
                // 左子树重
                2 => {
                    let rbf = self.node().right.node().bfactor;
                    if rbf == 1 || rbf == 0 {
                        let (a, b) = if rbf == 1 { (0, 0) } else { (1, -1) };
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if rbf == -1 {
                        let (a, b) = match self.node().right.node().left.node().bfactor {
                            1 => (-1, 0),
                            0 => (0, 0),
                            -1 => (0, 1),
                            _ => unreachable!(),
                        };
                        // 先右旋再左旋
                        self.node().right.rotate_right();
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().right.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!()
                    }
                }
                _ => {}
            },
        }
    }

    /// 获取节点
    fn node(&mut self) -> &mut AvlNode<T> {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut n) => n,
        }
    }

    /// 获取左子树
    fn left_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.left,
        }
    }

    /// 获取右子树
    fn right_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.right,
        }
    }

    /// 左旋
    fn rotate_left(&mut self) {
        let mut v = std::mem::replace(self, Null);
        let mut right = std::mem::replace(v.right_subtree(), Null);
        let right_left = std::mem::replace(right.left_subtree(), Null);
        *v.right_subtree() = right_left;
        *right.left_subtree() = v;
        *self = right;
    }

    /// 右旋
    fn rotate_right(&mut self) {
        let mut v = std::mem::replace(self, Null);
        let mut left = std::mem::replace(v.left_subtree(), Null);
        let left_right = std::mem::replace(left.right_subtree(), Null);
        *v.left_subtree() = left_right;
        *left.right_subtree() = v;
        *self = left;
    }

    /// 获取节点个数
    ///
    /// 树节点数是左右子树节点数加根节点数，递归计算
    pub fn len(&self) -> usize {
        match self {
            Null => 0,
            Tree(node) => 1 + node.left.len() + node.right.len(),
        }
    }

    /// 获取叶子节点个数
    pub fn leaf_len(&self) -> usize {
        match self {
            Null => 0,
            Tree(node) => match (&node.left, &node.right) {
                (Null, Null) => 1,
                (left, Null) => left.leaf_len(),
                (Null, right) => right.leaf_len(),
                (left, right) => left.leaf_len() + right.leaf_len(),
            },
        }
    }

    /// 获取非叶子节点个数
    pub fn non_leaf_len(&self) -> usize {
        self.len() - self.leaf_len()
    }

    /// 获取树深度
    ///
    /// 树深度是左右子树深度最大值 + 1，递归计算
    pub fn depth(&self) -> usize {
        match *self {
            Null => 0,
            Tree(ref node) => max(node.left.depth(), node.right.depth()) + 1,
        }
    }

    /// 判空
    pub fn is_empty(&self) -> bool {
        match *self {
            Null => true,
            Tree(_) => false,
        }
    }

    /// 查找val是否在树内
    pub fn search(&self, val: &T) -> bool {
        match *self {
            Null => false,
            Tree(ref node) => match val.cmp(&node.val) {
                Ordering::Less => match &node.left {
                    Null => false,
                    Tree(_) => node.left.search(val),
                },
                Ordering::Equal => true,
                Ordering::Greater => match &node.right {
                    Null => false,
                    Tree(_) => node.right.search(val),
                },
            },
        }
    }

    /// 先序遍历 - 递归版
    pub fn preorder(&self) {
        match self {
            Null => {}
            Tree(node) => {
                println!("val: {:?}", node.val);
                node.left.preorder();
                node.right.preorder();
            }
        }
    }

    /// 中序遍历 - 递归版
    pub fn inorder(&self) {
        match self {
            Null => {}
            Tree(node) => {
                node.left.inorder();
                println!("val: {:?}", node.val);
                node.right.inorder();
            }
        }
    }

    /// 后序遍历 - 递归版
    pub fn postorder(&self) {
        match self {
            Null => {}
            Tree(node) => {
                node.left.postorder();
                node.right.postorder();
                println!("val: {:?}", node.val);
            }
        }
    }

    /// 层次遍历
    pub fn level_order(&self) {
        match self {
            Null => {}
            Tree(head) => {
                let size = self.len();
                let mut q = Queue::new(size);
                let _ = q.enqueue(head);

                while !q.is_empty() {
                    let node = q.dequeue().unwrap();
                    println!("val: {:?}", node.val);

                    if let Tree(ref left) = node.left {
                        let _ = q.enqueue(left);
                    }
                    if let Tree(ref right) = node.right {
                        let _ = q.enqueue(right);
                    }
                }
            }
        }
    }

    /// 查看树中最小值
    pub fn min(&self) -> Option<&T> {
        match self {
            Null => None,
            Tree(node) => match node.left {
                Null => Some(&node.val),
                Tree(_) => node.left.min(),
            },
        }
    }

    /// 查看树中最大值
    pub fn max(&self) -> Option<&T> {
        match self {
            Null => None,
            Tree(node) => match node.right {
                Null => Some(&node.val),
                Tree(_) => node.right.max(),
            },
        }
    }
}
