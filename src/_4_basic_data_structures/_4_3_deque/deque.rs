/// 选择 Vec 的左端作为队尾，右端作为队首。
///
/// 为防止队列无限增长，添加了一个 cap 参数用于控制双端队列长度。
///
#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,   // 容量
    data: Vec<T>, // 数据容器
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Deque {
            cap,
            data: Vec::with_capacity(cap),
        }
    }

    /// 在队首添加一个新元素
    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val);
        Ok(())
    }

    /// 在队尾添加一个新元素
    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.size() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    /// 移除队首元素
    pub fn remove_front(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    /// 移除队尾元素
    pub fn remove_rear(&mut self) -> Option<T> {
        if self.size() > 0 {
            Some(self.data.remove(0))
        } else {
            None
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
