use crate::entity::PurchaseRecord;
use csv::ReaderBuilder;

pub fn read_raw_data(path: &'static str) -> Result<(), csv::Error> {
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_path(path)?;

    for record in reader.deserialize() {
        println!("{:?}", record);
        let record: PurchaseRecord = record?;
        println!(
            "shopper: {}, product: {}, datetime: {}",
            record.shopper_id, record.product_id, record.datetime
        );
    }

    Ok(())
}
