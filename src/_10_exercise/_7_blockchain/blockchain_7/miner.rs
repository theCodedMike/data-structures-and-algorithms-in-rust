#![allow(dead_code)]

use super::super::elements::transaction::Transaction;
use super::block::Block;
use super::pow::ProofOfWork;

const MINER_NAME: &str = "anonymous";

/// 矿工
#[derive(Debug)]
pub struct Miner {
    name: String,    // 名字
    balance: u64,    // 账户余额
    address: String, // 地址，用于接收比特币
}

impl Miner {
    pub fn new(address: String) -> Self {
        Miner {
            name: MINER_NAME.to_string(),
            balance: 0,
            address,
        }
    }

    pub fn mine_block(&self, txs: &mut Vec<Transaction>, pre_hash: String, bits: u32) -> Block {
        let from = "0x0000".to_string();
        let to = self.address.clone();
        let sign = format!("{} -> {}: 50 btc", from, to);
        let coinbase = Transaction::new(from, to, 0, 0, 0, sign);

        // 加入coinbase交易和普通交易
        let mut txs_2 = Vec::new();
        txs_2.push(coinbase);
        txs_2.append(txs);

        Self::mine_job(txs_2, pre_hash, bits)
    }

    /// 挖矿任务 - 工作量证明
    fn mine_job(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Block {
        let mut block = Block::new(txs, pre_hash, bits);
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn miner_info(&self) {
        println!("{:#?}", self);
    }
}
