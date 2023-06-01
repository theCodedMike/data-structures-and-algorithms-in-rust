use crate::_4_basic_data_structures::linked_list::{Link, Node};

/// 链表定义
pub struct List<T> {
    size: usize,   // 链表节点个数
    head: Link<T>, // 头节点
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// 头插法: 新节点总是加到头部
    pub fn push(&mut self, elem: T) {
        // take()方法会置空self.head，该方法的返回值是self.head的当前值
        let new = Node::new(elem, self.head.take());
        self.head = Some(Box::new(new));
        self.size += 1;
    }

    /// take会取出数据并置空
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    /// peek不改变链表，只是引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /// into_iter: 链表改变，成为迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// iter: 链表不变，只得到不可变迭代器
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// iter_mut: 链表不变，得到可变迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
