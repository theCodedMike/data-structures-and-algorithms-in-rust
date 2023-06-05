/// 选择 Vec 的左端作为队尾，右端作为队首(头插法)
///
/// 这样移除数据的复杂度是 O(1)，加入数据的复杂度为 O(n)
///
/// 为了防止队列无限增长，cap 参数用于控制队列长度
#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    /// 入队
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        // 判断是否有剩余空间，有则加入队列，无则返回Err
        if self.size() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    /// 出队
    pub fn dequeue(&mut self) -> Option<T> {
        if self.size() == 0 {
            None
        } else {
            self.data.pop()
        }
    }

    /// 判空
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// 获取实际存储的元素个数
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
