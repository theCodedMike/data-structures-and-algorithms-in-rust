use super::super::elements::transaction::Transaction;
use super::super::utils::serializer;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    pub nonce: u64,      // 交易记录值
    pub balance: u64,    // 余额
    pub name: String,    // 姓名
    pub address: String, // 地址
    pub hash: String,    // 哈希
}

impl Account {
    pub fn new(address: String, name: String) -> Self {
        let mut account = Account {
            nonce: 0,
            balance: 100,
            name,
            address,
            hash: "".to_string(),
        };

        account.set_hash();

        account
    }

    fn set_hash(&mut self) {
        let data = serializer::serialize(&self);
        self.hash = serializer::hash_str(&data);
    }

    // 交易，此处只是转移比特币
    pub fn transfer_to(
        &mut self,
        to: &mut Self,
        amount: u64,
        fee: u64,
    ) -> Result<Transaction, String> {
        if amount + fee > self.balance {
            return Err(String::from("Error: not enough amount!"));
        }

        self.balance -= amount;
        self.balance -= fee;
        self.nonce += 1;
        self.set_hash();

        to.balance += amount;
        to.nonce += 1;
        to.set_hash();

        let sign = format!("{} -> {}: {} btc", self.address, to.address, amount);
        let tx = Transaction::new(
            self.address.clone(),
            to.address.clone(),
            amount,
            fee,
            self.nonce,
            sign,
        );

        Ok(tx)
    }

    pub fn account_info(&self) {
        println!("{:#?}", self);
    }
}
