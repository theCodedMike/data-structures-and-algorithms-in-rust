use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
use crate::_4_basic_data_structures::{NNLink, NNNode};

/// 栈
///
/// 单链表(NonNull版)
///
/// 头插法
pub struct SingleListHeadNonNullStack<T> {
    size: usize,
    head: NNLink<T>,
}

impl<T> ExerciseStack for SingleListHeadNonNullStack<T> {
    type Item = T;

    fn new() -> Self {
        SingleListHeadNonNullStack {
            size: 0,
            head: None,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new_head = NNNode::new(elem);

        unsafe {
            if let Some(old_head) = self.head {
                (*new_head.as_ptr()).next = Some(old_head);
            }
        }
        self.head = Some(new_head);

        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        /*
        match self.head {
            None => {
                // 空链表
                None
            }
            Some(head) => unsafe {
                let old_head = Box::from_raw(head.as_ptr());

                match old_head.next {
                    None => {
                        // self.head.next为None，说明链表中只有1个元素
                        self.head = None;
                    }
                    Some(new_head) => {
                        self.head = Some(new_head);
                    }
                }
                self.size -= 1;

                Some(old_head.elem)
            }
        }
        */

        // 等价于上面的代码
        self.head.take().map(|head| unsafe {
            let old_head = Box::from_raw(head.as_ptr());

            match old_head.next {
                None => {
                    // self.head.next为None，说明链表中只有1个元素
                    self.head = None;
                }
                Some(new_head) => {
                    self.head = Some(new_head);
                }
            }
            self.size -= 1;

            old_head.elem
        })
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for SingleListHeadNonNullStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        self.head.map(|head| unsafe { &(*head.as_ptr()).elem })
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.head.map(|head| unsafe { &mut (*head.as_ptr()).elem })
    }
}
