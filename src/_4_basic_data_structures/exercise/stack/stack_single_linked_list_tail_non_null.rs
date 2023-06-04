use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::{NNLink, NNNode};

/// 栈
///
/// 单链表(NonNull版)
///
/// 尾插法
pub struct SingleListTailNonNullStack<T> {
    size: usize,
    head: NNLink<T>,
}

impl<T> ExerciseStack for SingleListTailNonNullStack<T> {
    type Item = T;

    fn new() -> Self {
        SingleListTailNonNullStack {
            size: 0,
            head: None,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new_tail = NNNode::new(elem);

        match self.head {
            None => {
                // 空链表
                self.head = Some(new_tail);
            }
            Some(head) => unsafe {
                // 将指针移动到最后一个节点
                let mut cur = head.as_ptr();
                for _ in 0..self.size - 1 {
                    // 能进入循环说明链表中至少有2个节点
                    cur = (*cur).next.unwrap().as_ptr();
                }
                (*cur).next = Some(new_tail);
            },
        }

        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.head.map(|head| unsafe {
            // 将指针移动到倒数第2个元素上
            let mut cur = head.as_ptr();
            if self.size > 1 {
                // 这里加if分支是因为如果self.size为1，self.size - 2这里会panic，虽然并不会进入循环
                // panicked at 'attempt to subtract with overflow'
                for _ in 0..self.size - 2 {
                    // 能进入循环说明链表中至少有3个元素
                    cur = (*cur).next.unwrap().as_ptr();
                }
            }

            self.size -= 1;
            match (*cur).next {
                None => {
                    // 链表中只有1个元素
                    self.head = None;
                    Box::from_raw(cur).elem
                }
                Some(tail) => {
                    (*cur).next = None;
                    Box::from_raw(tail.as_ptr()).elem
                }
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

impl<T> Peek for SingleListTailNonNullStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        self.head.map(|head| unsafe {
            // 将指针移动到最后一个元素
            let mut cur = head.as_ptr();
            for _ in 0..self.size - 1 {
                // 能进入循环说明链表中至少有2个元素
                cur = (*cur).next.unwrap().as_ptr();
            }
            &(*cur).elem
        })
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.head.map(|head| unsafe {
            // 将指针移动到最后一个元素
            let mut cur = head.as_ptr();
            for _ in 0..self.size - 1 {
                // 能进入循环说明链表中至少有2个元素
                cur = (*cur).next.unwrap().as_ptr();
            }
            &mut (*cur).elem
        })
    }
}
