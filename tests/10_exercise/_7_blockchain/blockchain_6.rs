use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_6::blockchain::BlockChain as BC;
use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::elements::account::Account;
use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::elements::transaction::Transaction;

/// cargo test -- --show-output test_blockchain_6
#[test]
fn test_blockchain_6() {
    let mut user1 = Account::new("0xabcd".to_string(), "Kim".to_string());
    let mut user2 = Account::new("0xabce".to_string(), "Tom".to_string());
    let mut user3 = Account::new("0xabcf".to_string(), "Jim".to_string());

    println!("-------------------------Mine Info----------------------------");
    let mut bc = BC::new();

    let mut txs: Vec<Transaction> = Vec::new();
    let res = user1.transfer_to(&mut user2, 9, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    let res = user1.transfer_to(&mut user2, 5, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    bc.add_block(txs);

    let mut txs: Vec<Transaction> = Vec::new();
    let res = user2.transfer_to(&mut user3, 6, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    let res = user2.transfer_to(&mut user3, 3, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    bc.add_block(txs);

    println!("-------------------------Account Info----------------------------");
    let users = vec![&user1, &user2, &user3];
    for u in users {
        u.account_info();
    }

    println!("-------------------------Block Info------------------------------");
    bc.block_info();

    // -------------------------Mine Info----------------------------
    // Start mining ...
    // Produce a new block!
    //
    // Start mining ...
    // Produce a new block!
    //
    // Start mining ...
    // Produce a new block!
    //
    // -------------------------Account Info----------------------------
    // Account {
    //     nonce: 2,
    //     balance: 84,
    //     name: "Kim",
    //     address: "0xabcd",
    //     hash: "43a313f7687f1bb31ba1511c0fcad924a0fb72241ec8699e38f4b2f590f2e972",
    // }
    // Account {
    //     nonce: 4,
    //     balance: 103,
    //     name: "Tom",
    //     address: "0xabce",
    //     hash: "82f22a084728d1a408d0e6d2153968bb62eb7701f7d646485cd739d1f69cd94f",
    // }
    // Account {
    //     nonce: 2,
    //     balance: 109,
    //     name: "Jim",
    //     address: "0xabcf",
    //     hash: "2ba65fbbb28012d25a4287ca3099e73b9a0d832e648ab1fbc94cd54b57ea231c",
    // }
    // -------------------------Block Info------------------------------
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305286,
    //         txs_hash: "fb8cde2d50d0d48bce64c96c507194823a5b6dcb46c90034fda8f0f6b0dc6656",
    //         pre_hash: "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 0,
    //             amount: 0,
    //             fee: 0,
    //             from: "0x0000",
    //             to: "0x0000",
    //             sign: "创世区块",
    //             hash: "fb8cde2d50d0d48bce64c96c507194823a5b6dcb46c90034fda8f0f6b0dc6656",
    //         },
    //     ],
    //     hash: "62bf69e30ffa7a354c02445a8ff1929bb9f2292baa6a3bbe1c662057eb0b703b",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305289,
    //         txs_hash: "46ac09fe4e083a6514982e2802dabb8a51c205cc76bc3dfd9aef997923c86b20",
    //         pre_hash: "62bf69e30ffa7a354c02445a8ff1929bb9f2292baa6a3bbe1c662057eb0b703b",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 1,
    //             amount: 9,
    //             fee: 1,
    //             from: "0xabcd",
    //             to: "0xabce",
    //             sign: "0xabcd -> 0xabce: 9 btc",
    //             hash: "53d5cc1ac2b19555233eb2a9e8fdb62fb4a19387127f5f3b45b195edaa568305",
    //         },
    //         Transaction {
    //             nonce: 2,
    //             amount: 5,
    //             fee: 1,
    //             from: "0xabcd",
    //             to: "0xabce",
    //             sign: "0xabcd -> 0xabce: 5 btc",
    //             hash: "84fa895cb4f0d21d555e66366af23648987fd81da539d99cb4f6810558863c2d",
    //         },
    //     ],
    //     hash: "74a5fcbeb199e1e328e071cdc58c0781dc6a2e7f382f0065d2b978df9a8286f7",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305292,
    //         txs_hash: "f04c9d5789b4a9cf2079bad4d4b325ace6e99d3a624b794d75b51903d7f22879",
    //         pre_hash: "74a5fcbeb199e1e328e071cdc58c0781dc6a2e7f382f0065d2b978df9a8286f7",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 3,
    //             amount: 6,
    //             fee: 1,
    //             from: "0xabce",
    //             to: "0xabcf",
    //             sign: "0xabce -> 0xabcf: 6 btc",
    //             hash: "edb391ee92448903f7b1f26b8e23e1cc4b7a77d78e9f5e70fec7ebedb7670052",
    //         },
    //         Transaction {
    //             nonce: 4,
    //             amount: 3,
    //             fee: 1,
    //             from: "0xabce",
    //             to: "0xabcf",
    //             sign: "0xabce -> 0xabcf: 3 btc",
    //             hash: "cde1e971fab799a9b3ca6adc150c4c3c3b472ab8f2185f3d1985179b4d30f3b2",
    //         },
    //     ],
    //     hash: "8423083a8dd7e5f76f8c298fe5260689225b4eb1f927ff1bbd801d3f4e5ed89b",
    // }
}
