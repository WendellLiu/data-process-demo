pub mod datetime_formatter;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use std::collections::{HashMap, HashSet};

type ShopperID = String;
type ProductID = u64;
type Datetime = DateTime<Utc>;
pub type HistoryMap = HashMap<ShopperID, HashSet<ProductID>>;

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

pub fn to_history_map(records: Vec<PurchaseRecord>) -> HistoryMap {
    let mut result = HistoryMap::new();

    for record in records.iter() {
        match result.get(&record.shopper_id) {
            Some(products) => {
                let mut p = products.clone();
                p.insert(record.product_id);
                result.insert(record.shopper_id.clone(), p);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(record.product_id);
                result.insert(record.shopper_id.clone(), set);
            }
        }
    }

    return result;
}
