use super::super::elements::transaction::Transaction;
use super::blockchain::BlockChain;
use super::miner::Miner;

const MINER_ADDRESS: &str = "0x1b2d";

/// 挖矿任务
pub struct Mine {
    pub miner: Miner,
    pub blockchain: BlockChain,
}

impl Mine {
    pub fn new() -> Self {
        Mine {
            miner: Miner::new(MINER_ADDRESS.to_string()),
            blockchain: BlockChain::new(),
        }
    }

    pub fn mining(&mut self, txs: &mut Vec<Transaction>) {
        // 准备pre_hash和难度值
        let pre_hash = self.blockchain.curr_hash.clone();
        let bits = self.blockchain.curr_bits;
        // 核心代码: 开始挖矿
        let block = self.miner.mine_block(txs, pre_hash, bits);
        // 区块保存
        self.blockchain.add_block(block);
    }
}
