mod linked_list;
mod linked_list_exercise;
mod list_stack;

pub use linked_list::*;
pub use linked_list_exercise::*;
pub use list_stack::*;

/// 链表节点
#[derive(Debug, Clone)]
struct Node<T> {
    elem: T,       // 数据
    next: Link<T>, // 下一个节点链接
}
// 节点连接用Box指针(大小确定)，因为确定大小才能分配内存
type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(elem: T, next: Link<T>) -> Self {
        Self { elem, next }
    }
}
