use crate::entity::PurchaseRecord;
use csv::ReaderBuilder;

pub fn read_raw_data(path: &'static str) -> Result<Vec<PurchaseRecord>, csv::Error> {
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_path(path)?;

    let mut result = Vec::new();

    for record in reader.deserialize() {
        let record: PurchaseRecord = record?;
        result.push(record)
    }

    Ok(result)
}
