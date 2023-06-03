use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::RawPtrNode;
use std::ptr::null_mut;

/// 栈
///
/// 单链表(裸指针版)
///
/// 尾插法
pub struct SingleListTailRawPtrStack<T> {
    size: usize,
    head: *mut RawPtrNode<T>,
}

impl<T> ExerciseStack for SingleListTailRawPtrStack<T> {
    type Item = T;

    fn new() -> Self {
        SingleListTailRawPtrStack {
            size: 0,
            head: null_mut(),
        }
    }

    fn push(&mut self, elem: Self::Item) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(RawPtrNode::new(elem)));

            if self.head.is_null() {
                // 空链表
                self.head = new_tail;
            } else {
                // 将指针移动到最后一个节点
                let mut cur = self.head;
                for _ in 0..self.size - 1 {
                    cur = (*cur).next;
                }
                (*cur).next = new_tail;
            }

            self.size += 1;
        }
    }

    fn pop(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.head.is_null() {
                return None;
            }

            let tail;
            if self.size == 1 {
                // 链表中只有1个元素
                tail = Box::from_raw(self.head);
                self.head = null_mut();
            } else {
                // 链表中至少有1个元素，将指针移动到倒数第2个元素上
                let mut cur = self.head;
                for _ in 0..self.size - 2 {
                    cur = (*cur).next;
                }
                tail = Box::from_raw((*cur).next);
                (*cur).next = null_mut();
            };

            self.size -= 1;

            Some(tail.elem)
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for SingleListTailRawPtrStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        unsafe {
            if self.head.is_null() {
                return None;
            }

            // 将指针移动到最后一个元素上
            let mut cur = self.head;
            for _ in 0..self.size - 1 {
                cur = (*cur).next;
            }

            Some(&(*cur).elem)
        }
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        unsafe {
            if self.head.is_null() {
                return None;
            }

            // 将指针移动到最后一个元素上
            let mut cur = self.head;
            for _ in 0..self.size - 1 {
                cur = (*cur).next;
            }

            Some(&mut (*cur).elem)
        }
    }
}
