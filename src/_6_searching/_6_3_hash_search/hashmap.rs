// slot保存位置，data保存数据
#[derive(Debug, Clone, PartialEq)]
pub struct HashMap<T> {
    size: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Default> HashMap<T> {
    pub fn new(size: usize) -> Self {
        let slot = vec![0; size];
        let data = vec![Default::default(); size];
        HashMap { size, slot, data }
    }

    fn hash(&self, key: usize) -> usize {
        key % self.size
    }

    fn rehash(&self, pos: usize) -> usize {
        self.hash(pos + 1)
    }

    pub fn insert(&mut self, key: usize, value: T) {
        if key == 0 {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        if self.slot[pos] == 0 {
            // 槽内无数据，可直接插入
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            // 槽内有数据，找下一个可行的位置
            let mut next = self.rehash(pos);
            while self.slot[next] != 0 && key != self.slot[next] {
                next = self.rehash(next);
                if next == pos {
                    // 槽满了则退出
                    println!("Error: slot is full, quit insertion");
                    return;
                }
            }

            // 找到了槽
            if self.slot[next] == 0 {
                // 找到了空槽，插入数据
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                // 键已存在，用新值替换掉旧值
                self.data[next] = value;
            }
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if key == 0 {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        if self.slot[pos] == 0 {
            // 槽中无数据，直接返回None
            None
        } else if self.slot[pos] == key {
            // 找到目标key了
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            // slot内存储的key与入参key不匹配
            let mut data = None;
            let mut stop = false;
            let mut found = false;
            let mut curr = pos;

            while self.slot[curr] != 0 && !found && !stop {
                if self.slot[curr] == key {
                    // 找到了key，删除
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    // 再哈希回到最初位置，说明找了一圈还没找到
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }

            data
        }
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        if key == 0 {
            panic!("Error: key must > 0");
        }

        // 计算数据位置
        let pos = self.hash(key);
        let mut data = None;
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        // 循环查找数据
        while self.slot[curr] != 0 && !found && !stop {
            if self.slot[curr] == key {
                found = true;
                data = self.data.get(curr);
            } else {
                // 再哈希回到最初位置，说明找了一圈还没找到
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }

        data
    }

    pub fn contains(&self, key: usize) -> bool {
        if key == 0 {
            panic!("Error: key must > 0");
        }

        self.slot.contains(&key)
    }

    pub fn len(&self) -> usize {
        let mut length = 0;

        for d in self.slot.iter() {
            if *d != 0 {
                length += 1;
            }
        }

        length
    }
}
