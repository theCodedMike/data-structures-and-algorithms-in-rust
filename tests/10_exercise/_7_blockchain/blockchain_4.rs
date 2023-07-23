use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::blockchain_4::blockchain::BlockChain as BC;
use data_structures_and_algorithms_in_rust::_10_exercise::_7_blockchain::elements::transaction::Transaction;

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
}
