use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

/// 布隆过滤器
pub struct BloomFilter<T: ?Sized> {
    bits: Vec<bool>,             // 比特桶
    hash_fn_count: usize,        // 哈希函数个数
    hashers: [DefaultHasher; 2], // 2个哈希函数
    _phantom: PhantomData<T>,    // T占位，为了欺骗编译器
}

impl<T: ?Sized + Hash> BloomFilter<T> {
    pub fn new(cap: usize, ert: f64) -> Self {
        let square_of_ln2 = std::f64::consts::LN_2.powf(2_f64);
        // 计算比特桶的大小和哈希函数的个数
        let bits_count = -1_f64 * (cap as f64) * ert.ln() / square_of_ln2;
        let hash_fn_count = -1_f64 * ert.log2();

        // 随机哈希函数
        let hashers = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];

        BloomFilter {
            bits: vec![false; bits_count.ceil() as usize],
            hash_fn_count: hash_fn_count.ceil() as usize,
            hashers,
            _phantom: PhantomData,
        }
    }

    /// 按照hash_fn_count计算值并置比特桶相应位为true
    pub fn insert(&mut self, elem: &T) {
        let hashes = self.make_hash(elem);
        for fn_i in 0..self.hash_fn_count {
            let idx = self.get_index(hashes, fn_i as u64);
            self.bits[idx] = true;
        }
    }

    /// 查询
    pub fn contains(&self, elem: &T) -> bool {
        let hashes = self.make_hash(elem);
        (0..self.hash_fn_count).all(|fn_i| {
            let idx = self.get_index(hashes, fn_i as u64);
            self.bits[idx]
        })
    }

    /// 计算哈希
    fn make_hash(&self, elem: &T) -> (u64, u64) {
        let hasher1 = &mut self.hashers[0].clone();
        let hasher2 = &mut self.hashers[1].clone();

        elem.hash(hasher1);
        elem.hash(hasher2);

        (hasher1.finish(), hasher2.finish())
    }

    /// 获取比特桶某位的下标
    ///
    /// h1 + fn_i x h2
    fn get_index(&self, (h1, h2): (u64, u64), fn_i: u64) -> usize {
        let ih2 = fn_i.wrapping_mul(h2); //      ih2 = fn_i x h2
        let h1pih2 = h1.wrapping_add(ih2); // h1pih2 = h1 + ih2
        (h1pih2 % (self.bits.len() as u64)) as usize
    }
}
