use super::super::elements::bcdb::BlockChainDb;
use super::super::elements::transaction::Transaction;
use super::super::utils::bkey::BKey;
use super::super::utils::serializer;
use super::block::Block;
use crate::_10_exercise::_7_blockchain::blockchain_7::pow::ProofOfWork;
use bigint::U256;
use leveldb::database::Database;
use std::collections::HashMap;
use std::sync::Mutex;

/// 难度值和创世区块哈希值
///
/// BIN: 0010 0001 0000 0000 1111 1111 1111 1111  
/// DEC: 553,713,663  
const CURR_BITS: u32 = 0x2100FFFF;

const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

const SAVE_DIR: &str = "bc_db";

pub struct BlockChain {
    pub gnes_hash: String,
    pub curr_hash: String,
    pub curr_bits: u32,
    blocks_db: Box<Database<BKey>>,
    blocks_index: Mutex<HashMap<String, Block>>,
}

impl BlockChain {
    pub fn new() -> Self {
        let mut db = BlockChainDb::new(SAVE_DIR);
        let genesis = Self::genesis_block();
        BlockChain::write_block(&mut db, &genesis);
        BlockChain::write_tail(&mut db, &genesis);
        println!("New produced block saved!\n");

        let gene_block = genesis.clone();
        let mut block_index = Mutex::new(HashMap::new());
        Self::update_hashmap(&mut block_index, gene_block);

        let gnes_hash = genesis.hash.clone();
        let curr_hash = gnes_hash.clone();
        BlockChain {
            gnes_hash,
            curr_hash,
            curr_bits: CURR_BITS,
            blocks_db: Box::new(db),
            blocks_index: block_index,
        }
    }

    fn genesis_block() -> Block {
        let from = "0x0000".to_string();
        let to = "0x0000".to_string();
        let sign = "创世区块".to_string();
        let tx = Transaction::new(from, to, 0, 0, 0, sign);
        let mut block = Block::new(vec![tx], PRE_HASH.to_string(), CURR_BITS);

        let header_ser = ProofOfWork::prepare_data(&mut block, 0);
        block.hash = serializer::hash_str(&header_ser);

        block
    }

    fn update_hashmap(hmap: &mut Mutex<HashMap<String, Block>>, block: Block) {
        match hmap.lock() {
            Ok(mut map) => {
                map.insert(block.hash.clone(), block);
            }
            Err(e) => {
                println!("Failed to get lock: {}", e);
            }
        }
    }

    /// 添加区块
    pub fn add_block(&mut self, block: Block) {
        // 数据入库
        Self::write_block(&mut self.blocks_db, &block);
        Self::write_tail(&mut self.blocks_db, &block);

        println!("New produced block saved!\n");
        self.curr_hash = block.hash.clone();
        self.curr_bits = block.header.bits;
        Self::update_hashmap(&mut self.blocks_index, block);
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
        let mut hash = self.curr_hash.clone();
        let hmap = self.blocks_index.lock().unwrap();
        let mut blocks = Vec::new();

        loop {
            match hmap.get(&hash) {
                None => panic!("Error getting block"),
                Some(val) => {
                    blocks.push(val.clone());
                    hash = val.header.pre_hash.clone();
                }
            }

            if hash == self.gnes_hash {
                if let Some(val) = hmap.get(&hash) {
                    blocks.push(val.clone());
                }
                break;
            }
        }

        blocks.reverse();
        for block in blocks {
            println!("{:#?}", block);
        }
    }
}
