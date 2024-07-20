use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub struct Transaction {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "href")]
    pub href: String,

    #[serde(rename = "amount_cents")]
    pub amount_cents: i64,

    #[serde(rename = "memo")]
    pub memo: String,

    #[serde(rename = "date")]
    pub date: String,
}

pub type Transactions = Vec<Transaction>;
