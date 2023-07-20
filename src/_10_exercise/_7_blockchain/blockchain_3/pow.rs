use super::super::utils::serializer;
use super::block::Block;
use bigint::U256;
use std::thread;
use std::time::Duration;

/// nonce最大值
///
/// BIN: 0111 1111 1111 1111 1111 1111 1111 1111  
/// DEC: 2,147,483,647  
const MAX_NONCE: u32 = 0x7FFFFFFF;

/// 工作量证明任务
pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    /// 计算当前任务难度值
    ///
    /// 0xFFFFFF: 1111 1111 1111 1111 1111 1111, 即16,777,215  
    /// 0x7FFFFF: 0111 1111 1111 1111 1111 1111, 即 8,388,607  
    pub fn new(bits: u32) -> Self {
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            if unshifted_expt <= 3 {
                ((bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
            } else {
                (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };
        if mant > 0x7FFFFF {
            ProofOfWork {
                target: Default::default(),
            }
        } else {
            ProofOfWork {
                target: U256::from(mant as u64) << (expt as usize),
            }
        }
    }

    /// 开启工作量证明任务，即挖矿
    pub fn run(&self, block: &mut Block) {
        println!("Start mining ...");
        thread::sleep(Duration::from_secs(3));

        let mut nonce = 0_u32;
        while nonce <= MAX_NONCE {
            // 计算值
            let hd_ser = Self::prepare_data(block, nonce);
            let mut hash_u = [0_u8; 32];
            serializer::hash_u8(&hd_ser, &mut hash_u);

            // 判断值是否满足要求，满足则计算并设置区块哈希值
            let hash_int = U256::from(hash_u);
            if hash_int <= self.target {
                block.hash = serializer::hash_str(&hd_ser);
                println!("Produce a new block!\n");
                return;
            }

            nonce += 1;
        }
    }

    /// 准备区块头数据
    fn prepare_data(block: &mut Block, nonce: u32) -> Vec<u8> {
        block.header.nonce = nonce;
        serializer::serialize(&block.header)
    }
}
