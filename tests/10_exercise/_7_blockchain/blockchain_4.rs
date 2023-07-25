use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_4::blockchain::BlockChain as BC;
use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::elements::transaction::Transaction;

/// cargo test -- --show-output test_blockchain_4
#[test]
fn test_blockchain_4() {
    println!("-------------------------Mine Info----------------------------");
    let mut bc = BC::new();

    let from = "0xabcd".to_string();
    let to = "0xabce".to_string();
    let sign = format!("{} -> {}: 9 btc", from, to);
    let tx = Transaction::new(from, to, 9, 1, 0, sign);
    bc.add_block(vec![tx]);

    let from = "0xabce".to_string();
    let to = "0xabcf".to_string();
    let sign = format!("{} -> {}: 6 btc", from, to);
    let tx = Transaction::new(from, to, 6, 1, 0, sign);
    bc.add_block(vec![tx]);

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
    // -------------------------Block Info------------------------------
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690304490,
    //         txs_hash: "13169c4d15fa620b0e71605890e92bbef7898c7cbe83996de80cdc23f7ea53cf",
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
    //     hash: "c9d125895fc27d92fa739f32b65a8e70431f1bc5db286fa85ef4eb85524e32f3",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690304493,
    //         txs_hash: "6aa2c511dd78c2519a33e80b56349b5cc55bc4cf1c251a626e7eae0ccac1ad30",
    //         pre_hash: "c9d125895fc27d92fa739f32b65a8e70431f1bc5db286fa85ef4eb85524e32f3",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 0,
    //             amount: 9,
    //             fee: 1,
    //             from: "0xabcd",
    //             to: "0xabce",
    //             sign: "0xabcd -> 0xabce: 9 btc",
    //             hash: "4752832ec2a33178c906fd03f159eb10d33c53f18c7d4485e36a682d0bf9d30a",
    //         },
    //     ],
    //     hash: "af6a899699cf8d75af76ca4f5dd452696f1507cdd400f9ce9a6c9c8bc1d7b297",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690304496,
    //         txs_hash: "baacaa450784eaeec0dbc23f7880a00675947d9ff5a85397625919f947a74c60",
    //         pre_hash: "af6a899699cf8d75af76ca4f5dd452696f1507cdd400f9ce9a6c9c8bc1d7b297",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 0,
    //             amount: 6,
    //             fee: 1,
    //             from: "0xabce",
    //             to: "0xabcf",
    //             sign: "0xabce -> 0xabcf: 6 btc",
    //             hash: "818311694d8bb7dd62ebe94e0c4b0b5ed4f0bada3e66fb5c2e6eb1c5f0344d18",
    //         },
    //     ],
    //     hash: "bb6662f2449e2628f9cf6ecdc73519995edbfbf0cdceac36cbab9ce942fea169",
    // }
}
