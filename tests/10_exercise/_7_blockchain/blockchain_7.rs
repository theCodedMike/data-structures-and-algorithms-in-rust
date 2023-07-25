use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_7::mine::Mine;
use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::elements::account::Account;
use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::elements::transaction::Transaction;

/// cargo test -- --show-output test_blockchain_7
#[test]
fn test_blockchain_7() {
    let mut user1 = Account::new("0xabcd".to_string(), "Kim".to_string());
    let mut user2 = Account::new("0xabce".to_string(), "Tom".to_string());
    let mut user3 = Account::new("0xabcf".to_string(), "Jim".to_string());

    println!("-------------------------Mine Info----------------------------");
    let mut mine = Mine::new();

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
    mine.mining(&mut txs);

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
    mine.mining(&mut txs);

    println!("-------------------------Miner Info------------------------------");
    mine.miner.miner_info();

    println!("-------------------------Account Info----------------------------");
    let users = vec![&user1, &user2, &user3];
    for u in users {
        u.account_info();
    }

    println!("-------------------------Block Info------------------------------");
    mine.blockchain.block_info();

    // -------------------------Mine Info----------------------------
    // Start mining ...
    // Produce a new block!
    //
    // Start mining ...
    // Produce a new block!
    //
    // -------------------------Miner Info------------------------------
    // Miner {
    //     name: "anonymous",
    //     balance: 0,
    //     address: "0x1b2d",
    // }
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
    //         time: 1690305990,
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
    //     hash: "e6a194105511a50fdd56b0df2677e9cd07a9e67215441516c045d24f44127404",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305990,
    //         txs_hash: "20c4a3b94c760d6a9f0bd9b2cd0dcb683cd6ad57d8c8fd3287df2df2eb5c1d3c",
    //         pre_hash: "e6a194105511a50fdd56b0df2677e9cd07a9e67215441516c045d24f44127404",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 0,
    //             amount: 0,
    //             fee: 0,
    //             from: "0x0000",
    //             to: "0x1b2d",
    //             sign: "0x0000 -> 0x1b2d: 50 btc",
    //             hash: "9a1cd23a5b0ecb6326621ae93c218e8db4499283386ce6b6008d496d469364c2",
    //         },
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
    //     hash: "1ee94ead08afa723c42a62a4010f50ad2c80e5575db3493a79538723058cd482",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305993,
    //         txs_hash: "dcb88666af447af1833b20e871f0a8f024b369b88111a2b315758c5e841539f7",
    //         pre_hash: "1ee94ead08afa723c42a62a4010f50ad2c80e5575db3493a79538723058cd482",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 0,
    //             amount: 0,
    //             fee: 0,
    //             from: "0x0000",
    //             to: "0x1b2d",
    //             sign: "0x0000 -> 0x1b2d: 50 btc",
    //             hash: "9a1cd23a5b0ecb6326621ae93c218e8db4499283386ce6b6008d496d469364c2",
    //         },
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
    //     hash: "acff416ca6c049dadfec7df2f951a4099aeb96b99a44d9e60f583966cf6f6a10",
    // }
}
