use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_5::blockchain::BlockChain as BC;
use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::elements::account::Account;

/// cargo test -- --show-output test_blockchain_5
#[test]
fn test_blockchain_5() {
    let mut user1 = Account::new("0xabcd".to_string(), "Kim".to_string());
    let mut user2 = Account::new("0xabce".to_string(), "Tom".to_string());
    let mut user3 = Account::new("0xabcf".to_string(), "Jim".to_string());

    println!("-------------------------Mine Info----------------------------");
    let mut bc = BC::new();

    let res = user1.transfer_to(&mut user2, 9, 1);
    match res {
        Ok(tx) => bc.add_block(vec![tx]),
        Err(e) => panic!("{}", e),
    }

    let res = user2.transfer_to(&mut user3, 6, 1);
    match res {
        Ok(tx) => bc.add_block(vec![tx]),
        Err(e) => panic!("{}", e),
    }

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
    //     nonce: 1,
    //     balance: 90,
    //     name: "Kim",
    //     address: "0xabcd",
    //     hash: "f562e92f93e25b53cc1c5aec5b5c97c9d7ec9a0ca701fcd8f11c38a0a8e90c8e",
    // }
    // Account {
    //     nonce: 2,
    //     balance: 102,
    //     name: "Tom",
    //     address: "0xabce",
    //     hash: "e6bd387693c5ae66ef00c45406f2421211fd6e8698412a5658028e36d6856fb0",
    // }
    // Account {
    //     nonce: 1,
    //     balance: 106,
    //     name: "Jim",
    //     address: "0xabcf",
    //     hash: "3994f1847fa8c624a9d82fb51162efedb3e56a82cd8315c9b876928568b01e36",
    // }
    // -------------------------Block Info------------------------------
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305008,
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
    //     hash: "ae3f95ac2915574a0307c74c373c80ec6fd2dc0cbc0660410224916c501eeafa",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305011,
    //         txs_hash: "baaaa84118587b802dbd7a59c893c8a56d0b53c8ff35cb48fc530da8a1c4f323",
    //         pre_hash: "ae3f95ac2915574a0307c74c373c80ec6fd2dc0cbc0660410224916c501eeafa",
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
    //     ],
    //     hash: "d6b198dada30f27fd31cec00996ee727879b5db68c5d9f9244f6fedf6118804c",
    // }
    // Block {
    //     header: BlockHeader {
    //         nonce: 0,
    //         bits: 553713663,
    //         time: 1690305014,
    //         txs_hash: "2ae03a6195f50e9d656df88e3bcce82371d61ab2dc7bd83de390b91a59a8c9b4",
    //         pre_hash: "d6b198dada30f27fd31cec00996ee727879b5db68c5d9f9244f6fedf6118804c",
    //     },
    //     tranxs: [
    //         Transaction {
    //             nonce: 2,
    //             amount: 6,
    //             fee: 1,
    //             from: "0xabce",
    //             to: "0xabcf",
    //             sign: "0xabce -> 0xabcf: 6 btc",
    //             hash: "b33b5d37c44e53e2a7ddf12b51adfdc77f2f40807e5f63a47d1f936504b30cd4",
    //         },
    //     ],
    //     hash: "ab961e426b937b3b5e840a1ce9cb000b86fa19ca630ad26f0ea3b376b04401e3",
    // }
}
