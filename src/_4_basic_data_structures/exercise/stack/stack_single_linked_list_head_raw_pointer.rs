use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::RawPtrNode;
use std::ptr::null_mut;

/// 栈
///
/// 单链表(裸指针版)
///
/// 头插法
pub struct SingleListHeadRawPtrStack<T> {
    size: usize,
    head: *mut RawPtrNode<T>,
}

impl<T> ExerciseStack for SingleListHeadRawPtrStack<T> {
    type Item = T;

    fn new() -> Self {
        SingleListHeadRawPtrStack {
            size: 0,
            head: null_mut(),
        }
    }

    fn push(&mut self, elem: Self::Item) {
        unsafe {
            let new_head = Box::into_raw(Box::new(RawPtrNode::new(elem)));

            if !self.head.is_null() {
                (*new_head).next = self.head;
            }
            self.head = new_head;

            self.size += 1;
        }
    }

    fn pop(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.head.is_null() {
                return None;
            }

            let old_head = Box::from_raw(self.head);

            let new_head = (*self.head).next;
            if new_head.is_null() {
                // self.head.next为Null，说明此时链表中只有1个节点
                self.head = null_mut();
            } else {
                // self.head.next不为Null，说明此时链表中至少有1个节点
                self.head = new_head;
            }
            self.size -= 1;

            Some(old_head.elem)
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}
impl<T> Peek for SingleListHeadRawPtrStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        unsafe { self.head.as_ref().map(|head| &head.elem) }
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        unsafe { self.head.as_mut().map(|head| &mut head.elem) }
    }
}
