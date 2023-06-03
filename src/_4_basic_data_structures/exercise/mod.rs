//! 这个模块用来练习
//!
//! 线性数据结构的底层实现可以分为2种：    
//!         stack  queue  deque   
//! 动态数组    *      *      *
//!    链表    *      *      *
//!
//! 再加上头插法和尾插法，所以排列组合下来每个都有4种要实现，一共12种

mod deque;
mod queue;
mod stack;

pub use deque::*;
pub use queue::*;
pub use stack::*;
