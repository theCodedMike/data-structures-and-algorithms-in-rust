#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

const CACHE_SIZE: usize = 100;

/// 元素项
#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    val: Option<V>,
    next: Option<usize>,
    prev: Option<usize>,
}

/// LRU缓存
#[derive(Debug)]
pub struct LRUCache<K, V> {
    cap: usize,
    head: Option<usize>,
    tail: Option<usize>,
    map: HashMap<K, usize>,
    entries: Vec<Entry<K, V>>,
}

impl<K: Clone + Hash + Eq, V> LRUCache<K, V> {
    pub fn new() -> Self {
        Self::with_capacity(CACHE_SIZE)
    }

    pub fn with_capacity(cap: usize) -> Self {
        LRUCache {
            cap,
            head: None,
            tail: None,
            map: HashMap::with_capacity(cap),
            entries: Vec::with_capacity(cap),
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.contains(&key) {
            // 存在key则更新
            self.access(&key);
            let entry = &mut self.entries[self.head.unwrap()];
            let old_val = entry.val.take();
            entry.val = Some(val);

            old_val
        } else {
            // 不存在则插入
            self.ensure_room();

            // 更新原始头指针
            let index = self.entries.len();
            self.head.map(|e| {
                self.entries[e].next = Some(index);
            });

            // 新的头结点
            self.entries.push(Entry {
                key: key.clone(),
                val: Some(val),
                prev: self.head,
                next: None,
            });
            self.head = Some(index);
            self.tail = self.tail.or(self.head);
            self.map.insert(key, index);

            None
        }
    }

    /*fn get(&mut self, key: &K) -> Option<&V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &self.entries;
        self.map
            .get(key)
            .and_then(move |i| entries[*i].val.as_ref())
    }*/

    /*fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &mut self.entries;
        self.map
            .get(key)
            .and_then(move |i| entries[*i].val.as_mut())
    }*/

    fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    // 确保容量足够，满了则移除末尾的元素
    fn ensure_room(&mut self) {
        if self.is_full() {
            self.remove_tail();
        }
    }

    /// 获取某个key的值
    ///
    /// 移除原来位置的值并在头部加入
    fn access(&mut self, key: &K) {
        let i = *self.map.get(key).unwrap();
        self.remove_from_list(i);
        self.head.take().map(|old| {
            let old_head = &mut self.entries[old];
            old_head.next = Some(i);
            let new_head = &mut self.entries[i];
            new_head.prev = Some(old);
            new_head.next = None;
            self.head = Some(i);
        });
    }

    fn remove_tail(&mut self) {
        if let Some(index) = self.tail {
            self.remove_from_list(index);
            let key = &self.entries[index].key;
            self.map.remove(key);
        }

        if self.tail.is_none() {
            self.head = None;
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(&key).map(|idx| {
            self.remove_from_list(idx);
            self.entries[idx].val.take().unwrap()
        })
    }

    fn remove_from_list(&mut self, i: usize) {
        let (prev, next) = {
            let entry = self.entries.get_mut(i).unwrap();
            (entry.prev, entry.next)
        };

        match (prev, next) {
            // 数据项在中间
            (Some(p), Some(n)) => {
                let prev_entry = &mut self.entries[p];
                prev_entry.next = next;
                let next_entry = &mut self.entries[n];
                next_entry.prev = prev;
            }
            // 数据项在头部
            (Some(p), None) => {
                let head = &mut self.entries[p];
                head.next = None;
                self.tail = prev;
            }
            // 数据项在尾部
            (None, Some(n)) => {
                let head = &mut self.entries[n];
                self.tail = head.next;
                head.next = None;
                let next = &mut self.entries[1];
                next.prev = None;
            }
            _ => {}
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.map.len() == self.cap
    }
}
