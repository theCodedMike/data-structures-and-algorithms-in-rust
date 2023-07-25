use super::super::elements::bcdb::BlockChainDb;
use super::super::elements::transaction::Transaction;
use super::super::utils::bkey::BKey;
use super::super::utils::serializer;
use super::block::Block;
use bigint::U256;
use redis::Connection;

/// 难度值和创世区块哈希值
///
/// BIN: 0010 0001 0000 0000 1111 1111 1111 1111  
/// DEC: 553,713,663  
const CURR_BITS: u32 = 0x2100FFFF;

const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

pub struct BlockChain {
    pub blocks: Vec<Block>,
    curr_bits: u32,
    blocks_db: Box<Connection>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::default();
        let genesis = Self::genesis_block();
        BlockChain::write_block(&mut db, &genesis);
        BlockChain::write_tail(&mut db, &genesis);

        BlockChain {
            blocks: vec![genesis],
            curr_bits: CURR_BITS,
            blocks_db: Box::new(db),
        }
    }

    fn genesis_block() -> Block {
        let from = "0x0000".to_string();
        let to = "0x0000".to_string();
        let sign = "创世区块".to_string();
        let tx = Transaction::new(from, to, 0, 0, 0, sign);
        Block::new(vec![tx], PRE_HASH.to_string(), CURR_BITS)
    }

    /// 添加区块
    pub fn add_block(&mut self, txs: Vec<Transaction>) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash = pre_block.hash.clone();
        let new_block = Block::new(txs, pre_hash, self.curr_bits);

        // 数据入库
        Self::write_block(&mut self.blocks_db, &new_block);
        Self::write_tail(&mut self.blocks_db, &new_block);

        self.blocks.push(new_block);
    }

    /// 将区块写入数据库
    fn write_block(db: &mut Connection, block: &Block) {
        // 基于区块链的header生成key
        let header_ser = serializer::serialize(&block.header);
        let mut hash_u = [0_u8; 32];
        serializer::hash_u8(&header_ser, &mut hash_u);

        let key = BKey {
            val: U256::from(hash_u),
        };
        let val = serializer::serialize(&block);
        BlockChainDb::write_db(db, key, &val);
    }

    /// 将区块哈希值作为尾巴写入
    fn write_tail(db: &mut Connection, block: &Block) {
        let key = BKey {
            val: U256::from("tail".as_bytes()),
        };
        let val = serializer::serialize(&block.hash);
        BlockChainDb::write_db(db, key, &val);
    }

    pub fn block_info(&self) {
        for b in self.blocks.iter() {
            println!("{:#?}", b);
        }
    }
}
