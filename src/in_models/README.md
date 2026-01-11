# 📥 Input Models Crate

The `in_models` crate defines API input models (DTOs) for the Property Price Tracker backend.  
These models are used to validate and deserialize incoming HTTP requests.

---

## Example: PropertyInput

```rust
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PropertyInput {
    pub external_id: String,
    pub title: String,
    pub description: Option<String>,
    pub sqm: Option<i64>,
    pub city: String,
    pub municipality: Option<String>,
    pub market_type: String, // "New" or "Resale"
    pub listed_at: NaiveDate,
    pub price_amount: i64,
    pub currency: String,
}