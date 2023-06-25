use std::fmt::{Display, Formatter};
/// 计算父节点下标
macro_rules! parent {
    ($child:ident) => {
        $child >> 1
    };
}
/// 计算左子节点下标
macro_rules! left_child {
    ($parent:ident) => {
        $parent << 1
    };
}
/// 计算右子节点下标
macro_rules! right_child {
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}
///
/// 二叉小顶堆定义
///
/// 注意: 第1个元素为0，但不算
///
#[derive(Debug, Clone)]
pub struct BinaryHeap {
    size: usize,    // 元素个数
    data: Vec<i32>, // 数据容器
}

impl BinaryHeap {
    pub fn new() -> Self {
        BinaryHeap {
            size: 0,
            data: vec![0],
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 获取堆中最小的数据
    pub fn min(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[1])
        }
    }

    /// 向堆末尾添加数据，并调整堆
    pub fn push(&mut self, val: i32) {
        self.data.push(val);
        self.size += 1;
        self.move_up(self.size);
    }

    /// 小数据上浮
    fn move_up(&mut self, mut c: usize) {
        loop {
            // 计算当前节点的父节点
            let p = parent!(c);
            if p <= 0 {
                break;
            }
            // 当前节点数据小于父节点则交换
            if self.data[c] < self.data[p] {
                self.data.swap(c, p);
            }
            // 父节点成为当前节点
            c = p;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.size {
            0 => {
                // 无数据，返回None
                None
            }
            1 => {
                // 有1个数据，直接返回
                self.size -= 1;
                self.data.pop()
            }
            _ => {
                // 有多个数据，先交换并弹出数据，再调整堆
                self.data.swap(1, self.size);
                let val = self.data.pop();
                self.size -= 1;
                self.move_down(1);
                val
            }
        }
    }

    /// 大数据下沉
    fn move_down(&mut self, mut c: usize) {
        loop {
            // 计算当前节点的左子节点位置
            let lc = left_child!(c);
            if lc > self.size {
                break;
            }
            // 计算当前节点的最小子节点位置
            let mc = self.min_child(c);
            // 当前节点数据大于最小子节点数据，则交换
            if self.data[c] > self.data[mc] {
                self.data.swap(c, mc);
            }
            // 最小子节点成为当前节点
            c = mc;
        }
    }

    /// 计算最小子节点位置
    fn min_child(&self, i: usize) -> usize {
        // 同时计算左右子节点位置
        let (lc, rc) = (left_child!(i), right_child!(i));
        // 1. 如果右子节点位置大于size，表示只有左子节点，则左子节点就是最小子节点
        // 2. 否则，同时存在左右子节点，需具体判断左右子节点数据大小，然后返回最小的子节点位置
        if rc > self.size {
            lc
        } else if self.data[lc] < self.data[rc] {
            lc
        } else {
            rc
        }
    }

    /// 构建新堆
    ///
    /// ori: [6, 7, 8, 9, 10]
    /// arr: [5, 4, 3, 1, 2]
    ///
    /// now: [1, 2, 4, 5, 3]
    pub fn build_new(&mut self, arr: &[i32]) {
        // 删除原始数据
        for _ in 0..self.size {
            self.data.pop();
        }

        // 添加新数据
        for &val in arr {
            self.data.push(val);
        }
        self.size = arr.len();

        // 调整为小顶堆
        let size = self.size;
        let mut p = parent!(size);
        while p > 0 {
            self.move_down(p);
            p -= 1;
        }
    }

    /// 逐个加入堆
    ///
    /// ori: [6, 7, 8, 9, 10]
    /// arr: [5, 4, 3, 1, 2]
    ///
    /// now: [1, 2, 5, 4, 3, 8, 6, 9, 7, 10]
    pub fn build_add(&mut self, arr: &[i32]) {
        for &val in arr {
            self.push(val);
        }
    }

    pub fn equal(&self, arr: &[i32]) -> bool {
        let len = arr.len();

        self.size == len && &self.data[1..] == arr
    }
}

impl Display for BinaryHeap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = &self.data[1..];
        write!(f, "size: {}, data: {:?}", self.size, data)
    }
}

impl PartialEq<[i32]> for BinaryHeap {
    fn eq(&self, other: &[i32]) -> bool {
        let len = other.len();

        self.size == len && &self.data[1..] == other
    }
}
