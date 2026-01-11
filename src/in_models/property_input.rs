use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PropertyInput {
    pub external_id: String,

    pub title: String,
    pub description: Option<String>,

    pub sqm: Option<i32>,

    pub city: String,
    pub municipality: Option<String>,

    pub market_type: PropertyMarketTypeInput,

    pub listed_at: NaiveDate,

    // Price info
    pub price_amount: i64,
    pub currency: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyMarketTypeInput {
    New,
    Resale,
}

impl From<PropertyMarketTypeInput> for String {
    fn from(market_type: PropertyMarketTypeInput) -> Self {
        match market_type {
            PropertyMarketTypeInput::New => "New".to_string(),
            PropertyMarketTypeInput::Resale => "Resale".to_string(),
        }
    }
}
