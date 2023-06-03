use crate::_4_basic_data_structures::exercise::{ExerciseStack, Peek};
/// 栈
///
/// 动态数组
///
/// 头插法
pub struct VecTailStack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> ExerciseStack for VecTailStack<T> {
    type Item = T;

    fn new() -> Self {
        VecTailStack {
            size: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, elem: Self::Item) {
        self.data.push(elem);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for VecTailStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        if self.is_empty() {
            return None;
        }
        self.data.last()
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        if self.is_empty() {
            return None;
        }
        self.data.last_mut()
    }
}
