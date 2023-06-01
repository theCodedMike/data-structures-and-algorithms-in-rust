type Link<T> = Option<Box<Node<T>>>;

/// 链表节点
struct Node<T> {
    elem: T,
    next: Link<T>,
}

/// 链表定义
pub struct List<T> {
    size: usize,   // 链表节点个数
    head: Link<T>, // 头节点
}
