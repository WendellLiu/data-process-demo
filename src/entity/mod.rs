pub mod datetime_formatter;

use chrono::serde::ts_seconds;
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PurchaseRecord {
    #[serde(rename = "Shopper ID")]
    pub shopper_id: String,
    #[serde(rename = "Product ID")]
    pub product_id: u64,
    #[serde(
        with = "datetime_formatter::linc_date_format",
        rename = "Purchase datetime"
    )]
    pub datetime: DateTime<Utc>,
}
