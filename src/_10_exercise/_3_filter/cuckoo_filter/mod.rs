mod bucket;
mod util;

use self::bucket::{Bucket, Fingerprint, BUCKET_SIZE, FINGERPRINT_SIZE};
use self::util::{get_alt_index, get_fai, FaI};
use rand::Rng;
use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// If insertion fails, we will retry this many times.
const MAX_RELOCATION: u32 = 500;

/// The default number of buckets.
const DEFAULT_CAPACITY: usize = (1 << 20) - 1;

/// 错误处理
#[derive(Debug)]
pub enum CuckooError {
    NotEnoughSpace,
}

/// 打印输出
impl Display for CuckooError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("NotEnoughSpace")
    }
}

impl std::error::Error for CuckooError {
    fn description(&self) -> &str {
        "Not enough space to store this item, rebucketing failed."
    }
}

pub struct CuckooFilter<H> {
    buckets: Box<[Bucket]>,
    len: usize,
    _hasher: PhantomData<H>,
}

impl Default for CuckooFilter<DefaultHasher> {
    fn default() -> Self {
        Self::new()
    }
}

impl CuckooFilter<DefaultHasher> {
    /// Construct a CuckooFilter with default capacity and hasher.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl<H> CuckooFilter<H>
where
    H: Hasher + Default,
{
    /// Constructs a Cuckoo Filter with a given max capacity
    pub fn with_capacity(cap: usize) -> Self {
        let capacity = cmp::max(1, cap.next_power_of_two() / BUCKET_SIZE);

        Self {
            buckets: std::iter::repeat(Bucket::new())
                .take(capacity)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            len: 0,
            _hasher: PhantomData,
        }
    }

    /// Checks if data is in the filter.
    pub fn contains<T: ?Sized + Hash>(&self, data: &T) -> bool {
        let FaI { fp, i1, i2 } = get_fai::<T, H>(data);
        let len = self.buckets.len();
        self.buckets[i1 % len]
            .get_fingerprint_index(fp)
            .or_else(|| self.buckets[i2 % len].get_fingerprint_index(fp))
            .is_some()
    }

    /// Adds `data` to the filter. Returns `Ok` if the insertion was successful,
    /// but could fail with a `NotEnoughSpace` error, especially when the filter
    /// is nearing its capacity.
    /// Note that while you can put any hashable type in the same filter, beware
    /// for side effects like that the same number can have diferent hashes
    /// depending on the type.
    /// So for the filter, 4711i64 isn't the same as 4711u64.
    ///
    /// **Note:** When this returns `NotEnoughSpace`, the element given was
    /// actually added to the filter, but some random *other* element was
    /// removed. This might improve in the future.
    pub fn add<T: ?Sized + Hash>(&mut self, data: &T) -> Result<(), CuckooError> {
        let fai = get_fai::<T, H>(data);
        if self.put(fai.fp, fai.i1) || self.put(fai.fp, fai.i2) {
            return Ok(());
        }

        let len = self.buckets.len();
        let mut rng = rand::thread_rng();
        let mut i = fai.random_index(&mut rng);
        let mut fp = fai.fp;
        for _ in 0..MAX_RELOCATION {
            let other_fp;
            {
                let loc = &mut self.buckets[i % len].buffer[rng.gen_range(0..BUCKET_SIZE)];
                other_fp = *loc;
                *loc = fp;
                i = get_alt_index::<H>(other_fp, i);
            }
            if self.put(other_fp, i) {
                return Ok(());
            }
            fp = other_fp;
        }

        // fp is dropped here, which means that the last item that was
        // rebucketed gets removed from the filter.
        // TODO: One could introduce a single-item cache for this element,
        // check this cache in all methods additionally to the actual filter,
        // and return NotEnoughSpace if that cache is already in use.
        // This would complicate the code, but stop random elements from
        // getting removed and result in nicer behaviour for the user.
        Err(CuckooError::NotEnoughSpace)
    }

    /// Adds `data` to the filter if it does not exist in the filter yet.
    /// Returns `Ok(true)` if `data` was not yet present in the filter and added successfully.
    pub fn test_and_add<T: ?Sized + Hash>(&mut self, data: &T) -> Result<bool, CuckooError> {
        if self.contains(data) {
            Ok(false)
        } else {
            self.add(data).map(|_| true)
        }
    }

    /// Number of items in the filter.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Exports fingerprints in all buckets, along with the filter's length for storage.
    /// The filter can be recovered by passing the `ExportedCuckooFilter` struct to the `from` method of `CuckooFilter`.
    pub fn export(&self) -> ExportedCuckooFilter {
        self.into()
    }

    /// Number of bytes the filter occupies in memory
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of_val(self) + self.buckets.len() * std::mem::size_of::<Bucket>()
    }

    /// Check if filter is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Deletes data from the filter. Returns true if data existed in the filter before.
    pub fn delete<T: ?Sized + Hash>(&mut self, data: &T) -> bool {
        let FaI { fp, i1, i2 } = get_fai::<T, H>(data);
        self.remove(fp, i1) || self.remove(fp, i2)
    }

    /// Extracts fingerprint values from all buckets, used for exporting the filters data.
    fn values(&self) -> Vec<u8> {
        self.buckets
            .iter()
            .flat_map(|b| b.get_fingerprint_data().into_iter())
            .collect()
    }

    fn remove(&mut self, fp: Fingerprint, i: usize) -> bool {
        let len = self.buckets.len();
        if self.buckets[i % len].delete(fp) {
            self.len -= 1;
            true
        } else {
            false
        }
    }

    fn put(&mut self, fp: Fingerprint, i: usize) -> bool {
        let len = self.buckets.len();
        if self.buckets[i % len].insert(fp) {
            self.len += 1;
            true
        } else {
            false
        }
    }
}

/// A minimal representation of the CuckooFilter which can be transferred or stored,
/// then recovered at a later stage.
#[derive(Debug)]
pub struct ExportedCuckooFilter {
    pub values: Vec<u8>,
    pub length: usize,
}

impl<H> From<ExportedCuckooFilter> for CuckooFilter<H> {
    fn from(exported: ExportedCuckooFilter) -> Self {
        // Assumes that the `BUCKET_SIZE` and `FINGERPRINT_SIZE` constants do not change.
        Self {
            buckets: exported
                .values
                .chunks(BUCKET_SIZE * FINGERPRINT_SIZE)
                .map(Bucket::from)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            len: exported.length,
            _hasher: PhantomData,
        }
    }
}

impl<H> From<&CuckooFilter<H>> for ExportedCuckooFilter
where
    H: Hasher + Default,
{
    fn from(cuckoo: &CuckooFilter<H>) -> Self {
        Self {
            values: cuckoo.values(),
            length: cuckoo.len(),
        }
    }
}
