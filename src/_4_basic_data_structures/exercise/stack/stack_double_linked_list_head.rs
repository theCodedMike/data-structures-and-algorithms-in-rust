use crate::_4_basic_data_structures::exercise::{ExerciseStack, RefPeek};
use crate::_4_basic_data_structures::{BiDiLink, BiDiNode};
use std::cell::{Ref, RefMut};
use std::rc::Rc;

/// 栈
///
/// 双向链表
///
/// 头插法
pub struct DoubleListHeadStack<T> {
    size: usize,
    head: BiDiLink<T>,
    tail: BiDiLink<T>,
}

impl<T> ExerciseStack for DoubleListHeadStack<T> {
    type Item = T;

    fn new() -> Self {
        DoubleListHeadStack {
            size: 0,
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, elem: Self::Item) {
        let new_head = BiDiNode::new(elem, None, None);

        match self.head.take() {
            None => {
                // self.head为None，说明此时是空链表
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
            Some(old_head) => {
                // 此时是非空链表，只操作self.head
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
        }

        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.head.take().and_then(|head| {
            match head.borrow_mut().next.take() {
                None => {
                    // head.next为None，说明此时仅剩1个元素了
                    self.tail = None;
                }
                Some(new_head) => {
                    // head -> next 即为 new_head
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
            }

            self.size -= 1;

            match Rc::try_unwrap(head) {
                Ok(head) => Some(head.into_inner().elem),
                // 这里处理的不够好，先这么做吧
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

impl<T> RefPeek for DoubleListHeadStack<T> {
    fn peek(&self) -> Option<Ref<Self::Item>> {
        self.head.as_ref().map(|head| {
            // &head.borrow().elem
            // head.borrow()会生成一个临时值，所以会报以下错误
            // cannot return value referencing temporary value
            Ref::map(head.borrow(), |node| &node.elem)
        })
    }

    fn peek_mut(&mut self) -> Option<RefMut<Self::Item>> {
        self.head.as_mut().map(|head| {
            // &mut head.borrow_mut().elem
            // head.borrow_mut()会生成一个临时值，所以会报以下错误
            // cannot return value referencing temporary value
            RefMut::map(head.borrow_mut(), |node| &mut node.elem)
        })
    }
}

impl<T> Drop for DoubleListHeadStack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}
