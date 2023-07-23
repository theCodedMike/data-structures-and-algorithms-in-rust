use super::super::elements::transaction::Transaction;
use super::super::utils::serializer;
use super::pow::ProofOfWork;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub nonce: u32, // 交易记录值
    pub bits: u32,
    pub time: i64,        // 区块打包时间
    pub txs_hash: String, // 当前区块交易哈希
    pub pre_hash: String, // 前一个区块的哈希
}

/// 区块
///
/// 基于具体交易
#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,      // 区块头
    pub tranxs: Vec<Transaction>, // 区块体
    pub hash: String,             // 区块哈希
}

impl Block {
    /// 创建一个区块
    ///
    /// txs: 区块体   
    /// pre_hash: 前一个区块的哈希
    /// bits: 比特位
    pub fn new(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Self {
        // 准备时间、计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_hash = Block::merkle_hash_str(&txs);

        Block {
            header: BlockHeader {
                nonce: 0,
                bits,
                time,
                txs_hash,
                pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        }
    }

    /// 计算梅根哈希值
    fn merkle_hash_str(txs: &Vec<Transaction>) -> String {
        if txs.is_empty() {
            return "00000000".to_string();
        }

        // 哈希值收集
        let mut merkle_tree = Vec::new();
        for tx in txs {
            merkle_tree.push(tx.hash.clone());
        }

        let mut j = 0;
        let mut size = merkle_tree.len();
        while size > 1 {
            let mut i = 0;
            let temp = size as u64;

            // 合并计算哈希值
            while i < temp {
                let k = std::cmp::min(i + 1, temp - 1);
                let idx1 = (j + i) as usize;
                let idx2 = (j + k) as usize;
                let hash1 = merkle_tree[idx1].clone();
                let hash2 = merkle_tree[idx2].clone();
                let merge = format!("{}-{}", hash1, hash2);
                let merge_ser = serializer::serialize(&merge);
                let merge_hash = serializer::hash_str(&merge_ser);
                // 合并计算的新哈希值放到 merkle_tree 的尾部
                merkle_tree.push(merge_hash);
                i += 2;
            }

            // 跳过已经处理过的哈希
            j += temp;
            // 哈希数减少一半
            size = (size + 1) >> 1;
        }

        if merkle_tree.is_empty() {
            "00000000".to_string()
        } else {
            merkle_tree.pop().unwrap()
        }
    }
}
