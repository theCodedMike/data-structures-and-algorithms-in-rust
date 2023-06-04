use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::{BiDiNNLink, BiDiNNNode};
use std::marker::PhantomData;

/// 栈
///
/// 双向链表(NonNull版本)
///
/// 尾插法
pub struct DoubleListTailNonNullStack<T> {
    size: usize,
    head: BiDiNNLink<T>,
    tail: BiDiNNLink<T>,
    _flg: PhantomData<T>,
}

impl<T> ExerciseStack for DoubleListTailNonNullStack<T> {
    type Item = T;

    fn new() -> Self {
        DoubleListTailNonNullStack {
            size: 0,
            head: None,
            tail: None,
            _flg: PhantomData,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new_tail = BiDiNNNode::new(elem);

        match self.tail {
            None => {
                // 空链表
                self.head = Some(new_tail);
            }
            Some(old_tail) => unsafe {
                (*new_tail.as_ptr()).prev = Some(old_tail);
                (*old_tail.as_ptr()).next = Some(new_tail);
            },
        }
        self.tail = Some(new_tail);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.tail.take().map(|old_tail| unsafe {
            let old = Box::from_raw(old_tail.as_ptr());

            match old.prev {
                None => {
                    // 链表中总共就1个节点
                    self.head = None;
                }
                Some(new_tail) => {
                    (*new_tail.as_ptr()).next = None;
                    (*old_tail.as_ptr()).prev = None;
                    self.tail = Some(new_tail);
                }
            }
            self.size -= 1;

            old.elem
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for DoubleListTailNonNullStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        self.tail.map(|tail| unsafe { &(*tail.as_ptr()).elem })
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.tail.map(|tail| unsafe { &mut (*tail.as_ptr()).elem })
    }
}
