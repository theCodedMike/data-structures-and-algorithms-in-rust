use super::super::utils::serializer;
use chrono::Utc;
use serde::Serialize;
use std::thread;
use std::time::Duration;

/// 区块头
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub time: i64,        // 区块打包时间
    pub pre_hash: String, // 前一个区块的哈希
    pub txs_hash: String, // 当前区块交易哈希
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
    pub fn new(txs: String, pre_hash: String) -> Self {
        // 用延迟3秒以模拟挖矿
        println!("Start mining ...");
        thread::sleep(Duration::from_secs(3));

        // 准备时间、计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_ser = serializer::serialize(&txs);
        let txs_hash = serializer::hash_str(&txs_ser);
        let mut block = Block {
            header: BlockHeader {
                time,
                txs_hash,
                pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        };

        block.set_hash();
        println!("Produce a new block!\n");

        block
    }

    /// 计算并设置区块哈希值
    fn set_hash(&mut self) {
        let header = serializer::serialize(&self.header);
        self.hash = serializer::hash_str(&header);
    }
}
