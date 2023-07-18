use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_2::blockchain::BlockChain as BC;

/// cargo test -- --show-output test_blockchain_2
#[test]
fn test_blockchain_2() {
    println!("----------------Mine Info----------------");
    let mut bc = BC::new();

    let tx = "0xabcd -> 0xabce: 5 btc".to_string();
    bc.add_block(tx);

    let tx = "0xabcd -> 0xabcf: 2.5 btc".to_string();
    bc.add_block(tx);

    println!("----------------Block Info----------------");
    bc.block_info();

    // ----------------Mine Info----------------
    // Start mining ...
    // Produce a new block!
    //
    // Start mining ...
    // Produce a new block!
    //
    // Start mining ...
    // Produce a new block!
    //
    // ----------------Block Info----------------
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1689717971,
    //         txs_hash: "811d05bb599c1b47d6e7a56a262c36990dddd258c8d414c80f923f0e56934d2f",
    //         pre_hash: "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7",
    //     },
    //     tranxs: "创世区块",
    //     hash: "cd4aea853a56130ce38a4878deef3a410f10262f6ac4e231a41b0c4aaec53c2d",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1689717977,
    //         txs_hash: "84eeeb7be34240b4a5c45534fc0951ec8f375d93cd3a77d2f154fbdfdde080c1",
    //         pre_hash: "cd4aea853a56130ce38a4878deef3a410f10262f6ac4e231a41b0c4aaec53c2d",
    //     },
    //     tranxs: "0xabcd -> 0xabce: 5 btc",
    //     hash: "aaae35e235f9aeb8ba57ca37a6bfaf7f3656ff914561d406d346ea8df20e8bec",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1689717983,
    //         txs_hash: "ca51ee57941a2af26ae2fa02d05f6276f6c2c050535314d6393501b797b13438",
    //         pre_hash: "aaae35e235f9aeb8ba57ca37a6bfaf7f3656ff914561d406d346ea8df20e8bec",
    //     },
    //     tranxs: "0xabcd -> 0xabcf: 2.5 btc",
    //     hash: "b94961921b82512ab33fb8a36c6901261176a8c3fe2730c132d7b4b6b921b0e1",
    // }
}
