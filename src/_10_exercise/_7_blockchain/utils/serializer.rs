use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::Serialize;

/// 序列化数据
pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    bincode::serialize(value).unwrap()
}

/// 计算value哈希值
pub fn hash_str(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

/// 计算value哈希值并传递给out参数
pub fn hash_u8(value: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result(out);
}
