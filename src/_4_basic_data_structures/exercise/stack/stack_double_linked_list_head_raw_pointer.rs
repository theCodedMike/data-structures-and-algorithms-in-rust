use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::BiDiRawPtrNode;
use std::ptr::null_mut;

/// 栈
///
/// 双向链表(裸指针版本)
///
/// 头插法
pub struct DoubleListHeadRawPtrStack<T> {
    size: usize,
    head: *mut BiDiRawPtrNode<T>,
    tail: *mut BiDiRawPtrNode<T>,
}

impl<T> ExerciseStack for DoubleListHeadRawPtrStack<T> {
    type Item = T;

    fn new() -> Self {
        DoubleListHeadRawPtrStack {
            size: 0,
            head: null_mut(),
            tail: null_mut(),
        }
    }

    fn push(&mut self, elem: Self::Item) {
        unsafe {
            let new_head = Box::into_raw(Box::new(BiDiRawPtrNode::new(elem)));

            if self.head.is_null() {
                // 空链表
                self.head = new_head;
                self.tail = new_head;
            } else {
                (*self.head).prev = new_head;
                (*new_head).next = self.head;
                self.head = new_head;
            }

            self.size += 1;
        }
    }

    fn pop(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.head.is_null() {
                // 空链表
                return None;
            }

            let head = Box::from_raw(self.head);
            let new_head = (*self.head).next;
            if new_head.is_null() {
                // self.head.next为Null，说明此时链表中只有1个元素
                self.tail = null_mut();
            } else {
                (*new_head).prev = null_mut();
            }
            self.head = new_head;
            self.size -= 1;

            Some(head.elem)
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for DoubleListHeadRawPtrStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        unsafe { self.head.as_ref().map(|head| &head.elem) }
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        unsafe { self.head.as_mut().map(|head| &mut head.elem) }
    }
}
