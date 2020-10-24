use entity::helpers::{get_recommendation_index, get_recommendation_parameters, to_material};
use entity::schema;
use process::read_raw_data;

mod entity;
mod process;

fn main() {
    let (history_map, shopper_ids, product_ids) = match read_raw_data("./data.txt") {
        Ok(r) => to_material(r),
        Err(e) => panic!(e),
    };

    println!("history map: {:?}", history_map);
    println!("shopper_ids: {:?}", shopper_ids);
    println!("product_ids: {:?}", product_ids);

    // 1.

    // 2.
    let mut recommends: Vec<(schema::ProductID, schema::JaccardIndex)> = Vec::new();
    let shopper_id = String::from("andrew");
    let top_n = 3;

    for product_id in product_ids.iter() {
        let s_id = shopper_id.clone();
        let s_product_ids = history_map.get(&s_id).unwrap();

        match s_product_ids.get(product_id) {
            Some(_) => {}
            None => {
                let (target, rest) = get_recommendation_parameters(s_id, product_id, &history_map);
                recommends.push((*product_id, get_recommendation_index(&target, rest)));
            }
        }
    }

    recommends.sort_by(|a, b| b.1.partial_cmp(&(a.1)).unwrap());

    println!("answer: {:?}", recommends[0..top_n].to_vec());
}
