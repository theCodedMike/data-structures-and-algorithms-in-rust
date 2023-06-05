use crate::_4_basic_data_structures::{Link, Node};
use std::fmt::Debug;

/// 单链表实现的Vec
///
#[derive(Debug)]
pub struct LVec<T> {
    size: usize,
    head: Link<T>,
}

impl<T: Copy + Debug + PartialEq> LVec<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    pub fn clean(&mut self) {
        self.size = 0;
        self.head = None;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, elem: T) {
        let new = Node::new(elem, None);
        if self.is_empty() {
            self.head = Some(Box::new(new));
        } else {
            let mut curr = self.head.as_mut().unwrap();
            // 找到最后一个节点
            for _i in 0..self.size - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            // 在最后一个节点之后插入新数据
            curr.next = Some(Box::new(new));
        }
        self.size += 1;
    }

    // 将另一个链表拼接到当前的链表
    pub fn append(&mut self, other: &mut Self) {
        while let Some(mut node) = other.head.take() {
            self.push(node.elem);
            other.head = node.next.take();
        }
        other.clean();
    }

    pub fn insert(&mut self, mut index: usize, elem: T) {
        // 如果index>=元素个数，插在末尾
        if index >= self.size {
            index = self.size;
        }
        // 分3种情况插入新节点
        if self.is_empty() {
            // 如果链表为空，则插入的是第1个节点
            let new = Node::new(elem, None);
            self.head = Some(Box::new(new));
        } else {
            // 如果链表不为空
            if index == 0 {
                // 如果要插入首位
                let new = Node::new(elem, self.head.take());
                self.head = Some(Box::new(new));
            } else {
                // 如果要插在链表中间
                let mut curr = self.head.as_mut().unwrap();
                for _i in 0..index - 1 {
                    // 找到插入位置
                    curr = curr.next.as_mut().unwrap();
                }
                let new = Node::new(elem, curr.next.take());
                curr.next = Some(Box::new(new));
            }
        }
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        // 如果index不在[0, self.size-1]范围内，返回None
        if index >= self.size {
            return None;
        }
        // 分2种情况
        let mut node;
        if index == 0 {
            // 删除首节点
            node = self.head.take().unwrap();
            self.head = node.next.take();
        } else {
            // 非首节点需要找到待删除的节点，并处理前后链接
            let mut curr = self.head.as_mut().unwrap();
            for _i in 0..index - 1 {
                curr = curr.next.as_mut().unwrap();
            }
            node = curr.next.take().unwrap();
            curr.next = node.next.take();
        }

        self.size -= 1;

        Some(node.elem)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.remove(self.size - 1)
    }

    /// 获取某个索引位置的元素的不可变引用
    /// 如果索引无效，则返回None
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }

        let mut curr = self.head.as_ref().unwrap();
        for _i in 0..index {
            curr = curr.next.as_ref().unwrap();
        }

        Some(&curr.elem)
    }

    /// 查找是否存在某个元素。如果该元素存在多个，则返回第1次出现的位置   
    /// 如果存在，则返回索引[0, self.size - 1]   
    /// 如果不存在，返回-1
    pub fn find(&self, elem: T) -> isize {
        let mut idx = -1;
        let mut curr = self.head.as_ref();

        while let Some(node) = curr {
            idx += 1;
            if elem == node.elem {
                return idx;
            }
            curr = node.next.as_ref();
        }

        return -1;
    }

    /// 判断某个元素是否存在
    pub fn exist(&self, elem: T) -> bool {
        self.find(elem) != -1
    }

    /// 打印LVec，也可以实现Display特性
    pub fn print_lvec(&self) {
        let mut curr = self.head.as_ref();
        while let Some(node) = curr {
            println!("lvec val: {:#?}", node.elem);
            curr = node.next.as_ref();
        }
    }
}
