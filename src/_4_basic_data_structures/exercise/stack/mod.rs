//! vec表明底层是动态数组，list表明底层是链表
//!
//! single linked list 表明是单链表，double linked list表明是双链表
//!
//! head表明是头插法，tail表明是尾插法
//!
mod bench;
mod stack_double_linked_list_head;
mod stack_double_linked_list_tail;
mod stack_single_linked_list_head;
mod stack_single_linked_list_tail;
mod stack_vec_head;
mod stack_vec_tail;

pub use bench::*;
pub use stack_double_linked_list_head::*;
pub use stack_double_linked_list_tail::*;
pub use stack_single_linked_list_head::*;
pub use stack_single_linked_list_tail::*;
pub use stack_vec_head::*;
pub use stack_vec_tail::*;
use std::cell::{Ref, RefMut};

/// 定义栈的公共行为
pub trait ExerciseStack {
    type Item;

    /// 创建一个新的空栈
    fn new() -> Self;

    /// 入栈
    fn push(&mut self, elem: Self::Item);

    /// 出栈
    fn pop(&mut self) -> Option<Self::Item>;

    /// 判空
    fn is_empty(&self) -> bool;

    /// 查看栈中元素个数
    fn size(&self) -> usize;
}

/// peek接口，返回值一般为Some(&T)
pub trait Peek: ExerciseStack {
    /// 查看栈顶元素
    fn peek(&self) -> Option<&Self::Item>;

    /// 获取栈顶元素的可变引用
    fn peek_mut(&mut self) -> Option<&mut Self::Item>;
}

/// peek接口，返回值一般为Some(Ref(T))
pub trait RefPeek: ExerciseStack {
    /// 查看栈顶元素
    fn peek(&self) -> Option<Ref<Self::Item>>;

    /// 获取栈顶元素的可变引用
    fn peek_mut(&mut self) -> Option<RefMut<Self::Item>>;
}
