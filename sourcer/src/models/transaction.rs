// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::transaction;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: transaction = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "Amount")]
    amount: f64,

    #[serde(rename = "Attachment")]
    attachment: Option<String>,

    #[serde(rename = "Card Holder Full Name")]
    card_holder_full_name: Option<String>,

    #[serde(rename = "Card Last Four Digits")]
    card_last_four_digits: Option<String>,

    #[serde(rename = "Currency")]
    currency: String,

    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Exchange From")]
    exchange_from: Option<String>,

    #[serde(rename = "Exchange Rate")]
    exchange_rate: Option<f64>,

    #[serde(rename = "Exchange To")]
    exchange_to: Option<String>,

    #[serde(rename = "Merchant")]
    merchant: Option<String>,

    #[serde(rename = "Note")]
    note: Option<String>,

    #[serde(rename = "Payee Account Number")]
    payee_account_number: Option<String>,

    #[serde(rename = "Payee Name")]
    payee_name: Option<String>,

    #[serde(rename = "Payer Name")]
    payer_name: Option<String>,

    #[serde(rename = "Payment Reference")]
    payment_reference: Option<String>,

    #[serde(rename = "Running Balance")]
    running_balance: f64,

    #[serde(rename = "Total fees")]
    total_fees: f64,

    /// This is the id
    #[serde(rename = "TransferWise ID")]
    transfer_wise_id: String,
}
