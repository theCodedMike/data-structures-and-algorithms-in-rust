use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::{BiDiNNLink, BiDiNNNode};
use std::marker::PhantomData;

/// 栈
///
/// 双向链表(NonNull版本)
///
/// 头插法
pub struct DoubleListHeadNonNullStack<T> {
    size: usize,
    head: BiDiNNLink<T>,
    tail: BiDiNNLink<T>,
    _flg: PhantomData<T>,
}

impl<T> ExerciseStack for DoubleListHeadNonNullStack<T> {
    type Item = T;

    fn new() -> Self {
        DoubleListHeadNonNullStack {
            size: 0,
            head: None,
            tail: None,
            _flg: PhantomData,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new_head = BiDiNNNode::new(elem);

        match self.head {
            None => {
                // 空链表
                self.tail = Some(new_head);
            }
            Some(old_head) => unsafe {
                (*old_head.as_ptr()).prev = Some(new_head);
                (*new_head.as_ptr()).next = Some(old_head);
            },
        }
        self.head = Some(new_head);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.head.take().map(|old_head| unsafe {
            let old = Box::from_raw(old_head.as_ptr());

            match old.next {
                None => {
                    // 说明链表中总共就1个节点
                    self.tail = None;
                }
                Some(new_head) => {
                    (*new_head.as_ptr()).prev = None;
                    (*old_head.as_ptr()).next = None;
                    self.head = Some(new_head);
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

impl<T> Peek for DoubleListHeadNonNullStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        self.head.map(|head| unsafe { &(*head.as_ptr()).elem })
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.head.map(|head| unsafe { &mut (*head.as_ptr()).elem })
    }
}
