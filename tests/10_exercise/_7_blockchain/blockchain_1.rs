use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_1::blockchain::BlockChain as BC;

/// cargo test -- --show-output test_blockchain_1
#[test]
fn test_blockchain_1() {
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
    //         time: 1689718843,
    //         pre_hash: "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7",
    //         txs_hash: "811d05bb599c1b47d6e7a56a262c36990dddd258c8d414c80f923f0e56934d2f",
    //     },
    //     tranxs: "创世区块",
    //     hash: "e7ff8a9d31542347e8baf1922e1c2a057bc707878bd0ed742ad2ef76615b11a3",
    // }
    // Block {
    //     header: BlockHeader {
    //         time: 1689718846,
    //         pre_hash: "e7ff8a9d31542347e8baf1922e1c2a057bc707878bd0ed742ad2ef76615b11a3",
    //         txs_hash: "84eeeb7be34240b4a5c45534fc0951ec8f375d93cd3a77d2f154fbdfdde080c1",
    //     },
    //     tranxs: "0xabcd -> 0xabce: 5 btc",
    //     hash: "458a2ffd02b0f113734c823db6e4768821debdd872008bf614952b92034e3bfd",
    // }
    // Block {
    //     header: BlockHeader {
    //         time: 1689718849,
    //         pre_hash: "458a2ffd02b0f113734c823db6e4768821debdd872008bf614952b92034e3bfd",
    //         txs_hash: "ca51ee57941a2af26ae2fa02d05f6276f6c2c050535314d6393501b797b13438",
    //     },
    //     tranxs: "0xabcd -> 0xabcf: 2.5 btc",
    //     hash: "81bce6c7e491973b2e988b06f3ccb01693525d773bba78eaaf4e2bcef0487180",
    // }
}
