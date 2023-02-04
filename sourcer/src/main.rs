use blazing_expense::transaction_source::transaction_source::{CsvSource};
use blazing_expense::transaction_source::transaction_source::TransactionSource;
use settings::Settings;

mod models;
mod transaction_source;
mod settings;

fn main() {
    match Settings::new() {
        Ok(settings) => {
            let tx_source = CsvSource::new(settings.transaction_source.dir);
            let transactions = tx_source.source();
            ()
        }
        Err(err) => {
            eprintln!("Could not load settings: {:?}", err)

        }
    }

}
