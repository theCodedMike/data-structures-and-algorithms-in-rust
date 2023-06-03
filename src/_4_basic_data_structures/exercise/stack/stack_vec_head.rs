use crate::_4_basic_data_structures::exercise::stack::ExerciseStack;
use crate::_4_basic_data_structures::exercise::Peek;
/// 栈
///
/// 动态数组
///
/// 头插法
pub struct VecHeadStack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> ExerciseStack for VecHeadStack<T> {
    type Item = T;

    fn new() -> Self {
        VecHeadStack {
            size: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, elem: Self::Item) {
        self.data.insert(0, elem);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        Some(self.data.remove(0))
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl<T> Peek for VecHeadStack<T> {
    fn peek(&self) -> Option<&Self::Item> {
        if self.is_empty() {
            return None;
        }
        self.data.first()
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        if self.is_empty() {
            return None;
        }
        self.data.first_mut()
    }
}
