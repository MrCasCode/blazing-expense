use std::time::SystemTime;

use anyhow::Result;
use redis::{Client, Commands, Connection};
use serde_json::to_string;

use crate::models::Transaction;

pub trait TransactionPublisher {
    fn publish_transaction(&mut self, transaction: Transaction) -> Result<()>;
}

pub struct RedisPublisher {
    redis: Client,
    conn: Connection
}

impl RedisPublisher {
    pub fn new(address: &str) -> Result<RedisPublisher> {
        let client = Client::open(address)?;
        let conn = client.get_connection()?;
        Ok(RedisPublisher { redis: client, conn: conn })
    }
}

impl TransactionPublisher for RedisPublisher {
    fn publish_transaction(&mut self, transaction: Transaction) -> Result<()> {
        let tx_data = to_string(&transaction).unwrap();

        let _: () = self.conn
        .xadd(
            "raw_tx",
            "*",
            &vec![
                ("data", tx_data)
                ],
            )
            .unwrap();
        Ok(())
    }
}
