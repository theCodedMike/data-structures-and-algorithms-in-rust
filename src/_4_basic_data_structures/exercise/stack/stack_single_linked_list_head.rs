use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::{Link, Node};
/// 栈
///
/// 单链表
///
/// 头插法
pub struct SingleListHeadStack<T> {
    size: usize,
    head: Link<T>,
}
impl<T> ExerciseStack for SingleListHeadStack<T> {
    type Item = T;

    fn new() -> Self {
        SingleListHeadStack {
            size: 0,
            head: None,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new = Node::new(elem, self.head.take());
        self.head = Some(Box::new(new));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.head.take().map(|head| {
            self.head = head.next;
            self.size -= 1;
            head.elem
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for SingleListHeadStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        self.head.as_ref().map(|head| &head.elem)
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.head.as_mut().map(|head| &mut head.elem)
    }
}

impl<T> Drop for SingleListHeadStack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(mut node) = head {
            head = node.next.take();
        }
    }
}
