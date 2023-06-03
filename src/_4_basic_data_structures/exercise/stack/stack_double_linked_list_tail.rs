use crate::_4_basic_data_structures::exercise::{ExerciseStack, RefPeek};
use crate::_4_basic_data_structures::{BiDiLink, BiDiNode};
use std::cell::{Ref, RefMut};
use std::rc::Rc;

/// 栈
///
/// 双向链表
///
/// 尾插法
pub struct DoubleListTailStack<T> {
    size: usize,
    head: BiDiLink<T>,
    tail: BiDiLink<T>,
}

impl<T> ExerciseStack for DoubleListTailStack<T> {
    type Item = T;

    fn new() -> Self {
        DoubleListTailStack {
            size: 0,
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new_tail = BiDiNode::new(elem, None, None);

        match self.tail.take() {
            None => {
                // self.tail为None，说明此时链表为空
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
        }

        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;

        self.tail.take().and_then(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                None => {
                    // self.tail.prev为None，说明此时链表中仅剩下1个节点
                    self.head = None;
                }
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
            }

            match Rc::try_unwrap(old_tail) {
                Ok(old_tail) => Some(old_tail.into_inner().elem),
                // 这里暂时先这么处理
                Err(_) => None,
            }
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> RefPeek for DoubleListTailStack<T> {
    fn peek(&self) -> Option<Ref<Self::Item>> {
        self.tail
            .as_ref()
            .map(|tail| Ref::map(tail.borrow(), |node| &node.elem))
    }

    fn peek_mut(&mut self) -> Option<RefMut<Self::Item>> {
        self.tail
            .as_mut()
            .map(|tail| RefMut::map(tail.borrow_mut(), |node| &mut node.elem))
    }
}

impl<T> Drop for DoubleListTailStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}
