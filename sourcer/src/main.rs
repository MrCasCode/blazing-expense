use blazing_sourcer::transaction_output::transaction_output::{
    RedisPublisher, TransactionPublisher,
};
use blazing_sourcer::transaction_source::transaction_source::CsvSource;
use blazing_sourcer::transaction_source::transaction_source::TransactionSource;
use settings::Settings;

mod models;
mod settings;
mod transaction_source;

use anyhow::Result;

fn main() -> Result<()> {
    let settings = Settings::new()?;
    let mut publisher = RedisPublisher::new(&settings.redis.address)?;

    let tx_source = CsvSource::new(settings.transaction_source.dir);

    if let Ok(transactions) = tx_source.source() {
        for transaction in transactions {
            println!("Sending transaction {}", transaction.transfer_wise_id);
            publisher.publish_transaction(transaction)?;
        }
    }

    Ok(())
}
