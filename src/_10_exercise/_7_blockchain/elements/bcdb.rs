use redis::{Commands, Connection, ToRedisArgs};

const EXPIRE_TIME_SECONDS: usize = 60 * 1; // 1 minute

const REDIS_URI: &str = "redis://localhost:6379";

pub struct BlockChainDb;

impl BlockChainDb {
    /// 新建并返回数据库
    pub fn new(path: &str) -> Connection {
        let client = redis::Client::open(path).expect("Failed to open redis");

        client.get_connection().expect("Failed to get connection")
    }

    pub fn default() -> Connection {
        Self::new(REDIS_URI)
    }

    /// 将数据写入数据库
    ///
    /// impl ToRedisArgs for u8  
    /// impl ToRedisArgs for bool  
    /// impl ToRedisArgs for String  
    /// impl<'a> ToRedisArgs for &'a str  
    /// impl<T: ToRedisArgs> ToRedisArgs for Vec<T>  
    /// impl<'a, T: ToRedisArgs> ToRedisArgs for &'a [T]  
    /// impl<T: ToRedisArgs> ToRedisArgs for Option<T>  
    /// impl<T: ToRedisArgs> ToRedisArgs for &T  
    pub fn write_db<K: ToRedisArgs>(db: &mut Connection, key: K, val: &[u8]) {
        db.set_ex(key, val, EXPIRE_TIME_SECONDS)
            .expect("Failed to write k-v to Redis")
    }
}
