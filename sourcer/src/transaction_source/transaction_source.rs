use std::{
    fs::{self, ReadDir},
    time::{Duration, UNIX_EPOCH},
};

use crate::models::Transaction;
use anyhow::Result;

// Trait declaring what a transaction source needs to implement
// Think about it as a Java interface
pub trait TransactionSource {
    fn source(&self) -> Result<Vec<Transaction>>;
}

// This is going to be our CsvSource, it just requires a directory
// where to source transactions from
pub struct CsvSource {
    // Directory where to source transactionsf from
    directory: String,
}

// Methods available for CsvSource
impl CsvSource {
    // Constructor
    pub fn new(dir: String) -> CsvSource {
        CsvSource { directory: dir }
    }
}

// Implementing transaction source trait in CsvSource
impl TransactionSource for CsvSource {
    fn source(&self) -> Result<Vec<Transaction>> {
        let mut transactions = vec![];
        let directory = fs::read_dir(&self.directory)?;
        let csvs = get_csvs(directory);
        println!("Found {:?} in {:?}", &csvs, &self.directory);

        for csv in csvs {
            println!("Reading {}", &csv);

            let mut reader = csv::Reader::from_path(&csv)?;
            let timestamp = get_version_timestamp(&csv);

            for entry in reader.deserialize() {
                match entry {
                    Ok(tx) => {
                        let mut transaction: Transaction = tx;
                        transaction.version_timestamp = timestamp
                            .as_ref()
                            .ok()
                            .map_or(None, |duration| Some(duration.as_secs().to_string()));
                        transactions.push(transaction);
                    }
                    Err(err) => eprintln!("Could not deserialize transaction: {:?}", err),
                }
            }
        }

        Ok(transactions)
    }
}

fn get_csvs(directory: ReadDir) -> Vec<String> {
    let filenames: Vec<String> = directory
        .map(|entry| entry.unwrap().path()) // get file name
        .filter(|path| path.to_str().unwrap().ends_with(".csv")) // filter files that end with .csv
        .filter_map(|csv_paths| csv_paths.to_str().map(|csv_str| csv_str.to_string())) // convert to string
        .collect(); // collect the iterator into a collection, in this case a vector

    filenames
}

fn get_version_timestamp(file_name: &String) -> Result<Duration> {
    let metadata = fs::metadata(&file_name)?;
    let creation_time = metadata.created()?;

    let timestamp = creation_time.duration_since(UNIX_EPOCH)?;
    Ok(timestamp)
}
