use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_3::blockchain::BlockChain as BC;

/// cargo test -- --show-output test_blockchain_3
#[test]
fn test_blockchain_3() {
    println!("-------------------------Mine Info----------------------------");
    let mut bc = BC::new();

    let tx = "0xabcd -> 0xabce: 5 btc".to_string();
    bc.add_block(tx);

    let tx = "0xabcd -> 0xabcf: 2.5 btc".to_string();
    bc.add_block(tx);

    println!("-------------------------Block Info------------------------------");
    bc.block_info();

    // -------------------------Mine Info----------------------------
    // Start mining ...
    // Produce a new block!
    //
    // New produced block saved!
    //
    // Start mining ...
    // Produce a new block!
    //
    // New produced block saved!
    //
    // Start mining ...
    // Produce a new block!
    //
    // New produced block saved!
    //
    // -------------------------Block Info------------------------------
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690303720,
    //         txs_hash: "811d05bb599c1b47d6e7a56a262c36990dddd258c8d414c80f923f0e56934d2f",
    //         pre_hash: "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7",
    //     },
    //     tranxs: "创世区块",
    //     hash: "bee3be6cca219583924cf3353566e97070d9aae273889efe340cb8890a0b9e7a",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690303723,
    //         txs_hash: "84eeeb7be34240b4a5c45534fc0951ec8f375d93cd3a77d2f154fbdfdde080c1",
    //         pre_hash: "bee3be6cca219583924cf3353566e97070d9aae273889efe340cb8890a0b9e7a",
    //     },
    //     tranxs: "0xabcd -> 0xabce: 5 btc",
    //     hash: "0e600c06d264abc442f50a949d084e7228ca726182eac0955d6043cdf963c859",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690303726,
    //         txs_hash: "ca51ee57941a2af26ae2fa02d05f6276f6c2c050535314d6393501b797b13438",
    //         pre_hash: "0e600c06d264abc442f50a949d084e7228ca726182eac0955d6043cdf963c859",
    //     },
    //     tranxs: "0xabcd -> 0xabcf: 2.5 btc",
    //     hash: "23c70567aaba8db68f45303e9a9bb5f16413e5008b02576a944c92d51af18d4d",
    // }
}
