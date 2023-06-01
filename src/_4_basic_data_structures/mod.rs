pub mod deque;
pub mod linked_list;
pub mod queue;
pub mod stack;
pub mod vec;

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
