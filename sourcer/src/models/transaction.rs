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
    pub amount: f64,

    #[serde(rename = "Attachment")]
    pub attachment: Option<String>,

    #[serde(rename = "Card Holder Full Name")]
    pub card_holder_full_name: Option<String>,

    #[serde(rename = "Card Last Four Digits")]
    pub card_last_four_digits: Option<String>,

    #[serde(rename = "Currency")]
    pub currency: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Description")]
    pub description: String,

    #[serde(rename = "Exchange From")]
    pub exchange_from: Option<String>,

    #[serde(rename = "Exchange Rate")]
    pub exchange_rate: Option<f64>,

    #[serde(rename = "Exchange To")]
    pub exchange_to: Option<String>,

    #[serde(rename = "Merchant")]
    pub merchant: Option<String>,

    #[serde(rename = "Note")]
    pub note: Option<String>,

    #[serde(rename = "Payee Account Number")]
    pub payee_account_number: Option<String>,

    #[serde(rename = "Payee Name")]
    pub payee_name: Option<String>,

    #[serde(rename = "Payer Name")]
    pub payer_name: Option<String>,

    #[serde(rename = "Payment Reference")]
    pub payment_reference: Option<String>,

    #[serde(rename = "Running Balance")]
    pub running_balance: f64,

    #[serde(rename = "Total fees")]
    pub total_fees: f64,

    /// This is the id
    #[serde(rename = "TransferWise ID")]
    pub transfer_wise_id: String,

    #[serde(rename = "Version Timestamp")]
    pub version_timestamp: Option<String>,
}
