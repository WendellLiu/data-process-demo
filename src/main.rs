extern crate itertools;

use entity::helpers::{
    get_jaccard_index, get_recommendation_index, get_recommendation_parameters, to_material,
};
use entity::schema;
use process::read_raw_data;

use itertools::Itertools;

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
    let shopper_ids_vec: Vec<schema::ShopperID> = shopper_ids.into_iter().collect();
    let shopper_ids_index_combinations = (0..shopper_ids_vec.len()).combinations(2);

    let mut highest_jaccard_index = 0 as schema::JaccardIndex;
    let mut highest_combination: Vec<&schema::ShopperID> = vec![];
    for combination in shopper_ids_index_combinations.into_iter() {
        let shopper_id_a = &shopper_ids_vec[combination[0]];
        let shopper_id_b = &shopper_ids_vec[combination[1]];

        let product_ids_a = history_map.get(shopper_id_a).unwrap();
        let product_ids_b = history_map.get(shopper_id_b).unwrap();

        let jaccard_index = get_jaccard_index(product_ids_a, product_ids_b);
        match highest_jaccard_index < jaccard_index {
            true => {
                highest_jaccard_index = jaccard_index;
                highest_combination = vec![shopper_id_a, shopper_id_b];
                println!(
                    "got a new combination: comination: {:?}, jaccard_index: {}",
                    highest_combination, highest_jaccard_index
                );
            }
            false => {}
        }
    }
    println!(
        "answer 1: comination: {:?}, jaccard_index: {}",
        highest_combination, highest_jaccard_index
    );

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

    println!("answer 2: {:?}", recommends[0..top_n].to_vec());
}
