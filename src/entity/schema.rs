use crate::entity::datetime_formatter;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use std::collections::{HashMap, HashSet};

pub type ShopperID = String;
pub type ProductID = u64;
pub type Datetime = DateTime<Utc>;
pub type ProductIDs = HashSet<ProductID>;
pub type ShopperIDs = HashSet<ShopperID>;
pub type HistoryMap = HashMap<ShopperID, ProductIDs>;

pub type JaccardIndex = f64;

#[derive(Debug, Deserialize)]
pub struct PurchaseRecord {
    #[serde(rename = "Shopper ID")]
    pub shopper_id: ShopperID,
    #[serde(rename = "Product ID")]
    pub product_id: ProductID,
    #[serde(
        with = "datetime_formatter::linc_date_format",
        rename = "Purchase datetime"
    )]
    pub datetime: Datetime,
}
