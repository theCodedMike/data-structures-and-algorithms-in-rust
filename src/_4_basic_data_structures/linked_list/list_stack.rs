use crate::_4_basic_data_structures::linked_list::{Link, Node};

/// 链表栈
#[derive(Debug, Clone)]
pub struct Stack<T> {
    size: usize,
    top: Link<T>, // 栈顶控制整个栈
}
impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Self { size: 0, top: None }
    }

    pub fn push(&mut self, val: T) {
        // take取出top中节点，留下空位，所以可以回填节点
        let new = Node::new(val, self.top.take());
        self.top = Some(Box::new(new));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|top| {
            self.top = top.next;
            self.size -= 1;
            top.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|top| &top.elem)
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
