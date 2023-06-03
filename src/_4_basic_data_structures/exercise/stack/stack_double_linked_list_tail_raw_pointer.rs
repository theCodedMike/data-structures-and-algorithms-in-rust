use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::BiDiRawPtrNode;
use std::ptr::null_mut;

/// 栈
///
/// 双向链表(裸指针版本)
///
/// 尾插法
pub struct DoubleListTailRawPtrStack<T> {
    size: usize,
    head: *mut BiDiRawPtrNode<T>,
    tail: *mut BiDiRawPtrNode<T>,
}

impl<T> ExerciseStack for DoubleListTailRawPtrStack<T> {
    type Item = T;

    fn new() -> Self {
        DoubleListTailRawPtrStack {
            size: 0,
            head: null_mut(),
            tail: null_mut(),
        }
    }

    fn push(&mut self, elem: Self::Item) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(BiDiRawPtrNode::new(elem)));

            if self.tail.is_null() {
                // 空链表
                self.head = new_tail;
                self.tail = new_tail;
            } else {
                (*new_tail).prev = self.tail;
                (*self.tail).next = new_tail;
                self.tail = new_tail;
            }

            self.size += 1;
        }
    }

    fn pop(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.tail.is_null() {
                return None;
            }

            let old_tail = Box::from_raw(self.tail);
            let new_tail = (*self.tail).prev;
            if new_tail.is_null() {
                // self.tail.prev为Null，说明此时链表中只有1个元素
                self.head = null_mut();
            } else {
                (*new_tail).next = null_mut();
            }
            self.tail = new_tail;

            self.size -= 1;
            Some(old_tail.elem)
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for DoubleListTailRawPtrStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        unsafe { self.tail.as_ref().map(|tail| &tail.elem) }
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        unsafe { self.tail.as_mut().map(|tail| &mut tail.elem) }
    }
}
