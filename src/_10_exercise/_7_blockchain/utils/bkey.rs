use bigint::U256;
use db_key::Key;
use redis::{RedisWrite, ToRedisArgs};

/// 定义大Key
#[derive(Debug, PartialEq, Eq)]
pub struct BKey {
    pub val: U256,
}

impl ToRedisArgs for BKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(self.as_ref())
    }
}

impl<'a> From<&'a [u8]> for BKey {
    fn from(key: &'a [u8]) -> Self {
        assert_eq!(key.len(), 32);
        let mut res = [0_u8; 32];
        for (idx, val) in key.iter().enumerate() {
            res[idx] = *val;
        }
        unsafe { std::mem::transmute::<[u8; 32], Self>(res) }
    }
}

impl AsRef<[u8]> for BKey {
    fn as_ref(&self) -> &[u8] {
        let val = unsafe { std::mem::transmute::<_, &[u8; 32]>(self) };
        val
    }
}

impl Key<'_> for BKey {}
