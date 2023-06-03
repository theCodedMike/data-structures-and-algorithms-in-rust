use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::{Link, Node};
/// 栈
///
/// 单链表
///
/// 尾插法
pub struct SingleListTailStack<T> {
    size: usize,
    head: Link<T>,
}

impl<T> ExerciseStack for SingleListTailStack<T> {
    type Item = T;

    fn new() -> Self {
        SingleListTailStack {
            size: 0,
            head: None,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new = Some(Box::new(Node::new(elem, None)));
        if self.is_empty() {
            self.head = new;
        } else {
            let mut node = self.head.as_mut().unwrap();
            for _ in 0..self.size - 1 {
                node = node.next.as_mut().unwrap();
            }
            node.next = new;
        }
        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        let res;
        if self.size == 1 {
            let head = self.head.take().unwrap();
            res = head.elem
        } else {
            let mut node = self.head.as_mut().unwrap();
            for _ in 0..self.size - 2 {
                node = node.next.as_mut().unwrap();
            }
            let target = node.next.take().unwrap();
            res = target.elem;
        }
        self.size -= 1;

        Some(res)
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for SingleListTailStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        return if self.is_empty() {
            None
        } else {
            let mut node = self.head.as_ref().unwrap();
            for _ in 0..self.size - 1 {
                node = node.next.as_ref().unwrap();
            }
            Some(&node.elem)
        };
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        return if self.is_empty() {
            None
        } else {
            let mut node = self.head.as_mut().unwrap();
            for _ in 0..self.size - 1 {
                node = node.next.as_mut().unwrap();
            }
            Some(&mut node.elem)
        };
    }
}

impl<T> Drop for SingleListTailStack<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(mut node) = head {
            head = node.next.take();
        }
    }
}
