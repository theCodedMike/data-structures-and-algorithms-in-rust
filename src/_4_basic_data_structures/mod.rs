use std::cell::RefCell;
use std::ptr::{null_mut, NonNull};
use std::rc::Rc;

pub mod _4_1_stack;
pub mod _4_2_queue;
pub mod _4_3_deque;
pub mod _4_4_linked_list;
pub mod _4_5_vec;
pub mod exercise;
///
/// 单链表节点
///
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

///
/// 单链表节点 - 裸指针版
///
#[derive(Debug)]
struct RawPtrNode<T> {
    elem: T,
    next: *mut RawPtrNode<T>,
}

impl<T> RawPtrNode<T> {
    fn new(elem: T) -> Self {
        RawPtrNode {
            elem,
            next: null_mut(),
        }
    }
}

///
/// 单链表节点 - NonNull版
///
struct NNNode<T> {
    elem: T,
    next: NNLink<T>,
}
type NNLink<T> = Option<NonNull<NNNode<T>>>;

impl<T> NNNode<T> {
    fn new(elem: T) -> NonNull<Self> {
        unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(NNNode { elem, next: None }))) }
    }
}

///
/// 双链表节点
///
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

///
/// 双链表节点 - 裸指针版
///
#[derive(Debug)]
struct BiDiRawPtrNode<T> {
    elem: T,
    prev: *mut BiDiRawPtrNode<T>,
    next: *mut BiDiRawPtrNode<T>,
}

impl<T> BiDiRawPtrNode<T> {
    fn new(elem: T) -> Self {
        BiDiRawPtrNode {
            elem,
            prev: null_mut(),
            next: null_mut(),
        }
    }
}

///
/// 双链表节点 - NonNull版
///
#[derive(Debug)]
struct BiDiNNNode<T> {
    elem: T,
    prev: BiDiNNLink<T>,
    next: BiDiNNLink<T>,
}
type BiDiNNLink<T> = Option<NonNull<BiDiNNNode<T>>>;

impl<T> BiDiNNNode<T> {
    fn new(elem: T) -> NonNull<Self> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(BiDiNNNode {
                elem,
                prev: None,
                next: None,
            })))
        }
    }
}
