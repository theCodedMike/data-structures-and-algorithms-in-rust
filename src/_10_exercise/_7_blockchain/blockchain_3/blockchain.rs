use super::super::utils::bkey::BKey;
use super::super::utils::serializer;
use super::bcdb::BlockChainDb;
use super::block::Block;
use bigint::U256;
use leveldb::database::Database;

/// 难度值和创世区块哈希值
///
/// BIN: 0010 0001 0000 0000 1111 1111 1111 1111  
/// DEC: 553,713,663  
const CURR_BITS: u32 = 0x2100FFFF;

const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

const SAVE_DIR: &str = "bc_db";

pub struct BlockChain {
    pub blocks: Vec<Block>,
    curr_bits: u32,
    blocks_db: Box<Database<BKey>>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::new(SAVE_DIR);
        let genesis = Self::genesis_block();
        BlockChain::write_block(&mut db, &genesis);
        BlockChain::write_tail(&mut db, &genesis);
        println!("New produced block saved!\n");

        BlockChain {
            blocks: vec![genesis],
            curr_bits: CURR_BITS,
            blocks_db: Box::new(db),
        }
    }

    fn genesis_block() -> Block {
        Block::new("创世区块".to_string(), PRE_HASH.to_string(), CURR_BITS)
    }

    /// 添加区块
    pub fn add_block(&mut self, txs: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash = pre_block.hash.clone();
        let new_block = Block::new(txs, pre_hash, self.curr_bits);

        // 数据入库
        Self::write_block(&mut self.blocks_db, &new_block);
        Self::write_tail(&mut self.blocks_db, &new_block);

        println!("New produced block saved!\n");
        self.blocks.push(new_block);
    }

    /// 将区块写入数据库
    fn write_block(db: &mut Database<BKey>, block: &Block) {
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
    fn write_tail(db: &mut Database<BKey>, block: &Block) {
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
