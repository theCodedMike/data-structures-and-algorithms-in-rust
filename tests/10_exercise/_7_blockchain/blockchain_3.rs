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
}
