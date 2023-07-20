use super::super::utils::serializer;
use super::pow::ProofOfWork;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub nonce: u32,
    pub bits: u32,
    pub time: i64,
    pub txs_hash: String,
    pub pre_hash: String,
}

/// 区块
///
/// 用字符串来模拟交易
#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader, // 区块头
    pub tranxs: String,      // 区块体
    pub hash: String,        // 区块哈希
}

impl Block {
    /// 创建一个区块
    ///
    /// txs: 区块体   
    /// pre_hash: 前一个区块的哈希
    /// bits: 比特位
    pub fn new(txs: String, pre_hash: String, bits: u32) -> Self {
        // 准备时间、计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_ser = serializer::serialize(&txs);
        let txs_hash = serializer::hash_str(&txs_ser);
        let mut block = Block {
            header: BlockHeader {
                nonce: 0,
                bits,
                time,
                txs_hash,
                pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        };

        // 初始化挖矿任务并开始挖矿
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }
}
