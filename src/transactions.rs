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

    #[serde(rename = "type")]
    pub txn_type: String,

    #[serde(rename = "card_charge", skip_serializing_if = "Option::is_none")]
    pub card_charge: Option<CardChargeObject>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub struct CardChargeObject {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "href")]
    pub href: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub struct CardCharge {
    pub user: User,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub struct User {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "full_name")]
    pub full_name: String,
}

pub type Transactions = Vec<Transaction>;
