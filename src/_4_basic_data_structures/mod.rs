use std::cell::RefCell;
use std::rc::Rc;

pub mod deque;
pub mod exercise;
pub mod linked_list;
pub mod queue;
pub mod stack;
pub mod vec;

/// 单链表节点
#[derive(Debug, Clone)]
struct Node<T> {
    elem: T,       // 数据
    next: Link<T>, // 下一个节点链接
}
// 节点连接用Box指针(大小确定)，因为确定大小才能分配内存
type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(elem: T, next: Link<T>) -> Self {
        Node { elem, next }
    }
}

/// 双链表节点
#[derive(Debug)]
struct BiDiNode<T> {
    elem: T,           // 数据
    prev: BiDiLink<T>, // 前一个节点链接
    next: BiDiLink<T>, // 下一个节点链接
}

type BiDiLink<T> = Option<Rc<RefCell<BiDiNode<T>>>>;

impl<T> BiDiNode<T> {
    fn new(elem: T, prev: BiDiLink<T>, next: BiDiLink<T>) -> Rc<RefCell<BiDiNode<T>>> {
        Rc::new(RefCell::new(BiDiNode { elem, prev, next }))
    }
}
