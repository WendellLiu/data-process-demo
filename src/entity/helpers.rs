use crate::entity::schema;

use std::collections::HashSet;

pub fn to_history_map(records: Vec<schema::PurchaseRecord>) -> schema::HistoryMap {
    let mut result = schema::HistoryMap::new();

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

fn get_intersection(a: &schema::ProductIDs, b: &schema::ProductIDs) -> entity::ProductIDs {
    let mut set = HashSet::new();

    let intersection = a.intersection(b);
    for ele in intersection {
        set.insert(ele.clone());
    }

    return set;
}

fn get_union(a: &schema::ProductIDs, b: &schema::ProductIDs) -> schema::ProductIDs {
    let mut set = HashSet::new();

    let union = a.union(b);
    for ele in union {
        set.insert(ele.clone());
    }

    return set;
}

fn get_jaccard_index(a: &schema::ProductIDs, b: &schema::ProductIDs) -> schema::JaccardIndex {
    let intersection_len = get_intersection(a, b).len() as f64;
    let union_len = get_union(a, b).len() as f64;

    return intersection_len / union_len;
}

pub fn get_recommendation_parameters(
    shopper_id: schema::ShopperID,
    product_id: schema::ProductID,
    history_map: schema::HistoryMap,
) -> (schema::ProductIDs, Vec<schema::ProductIDs>) {
    let a: schema::ProductIDs = [].iter().cloned().collect();
    let b: schema::ProductIDs = [].iter().cloned().collect();
    let c: schema::ProductIDs = [].iter().cloned().collect();

    return (a, vec![b, c]);
}

pub fn get_recommendation_index(
    target: &schema::ProductIDs,
    rest: Vec<&schema::ProductIDs>,
) -> schema::JaccardIndex {
    let length = rest.len() as schema::JaccardIndex;

    let mut sum = 0 as schema::JaccardIndex;
    for rt in rest {
        sum += get_jaccard_index(target, rt);
    }
    return sum / length;
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_intersection_works() {
        use super::*;
        let a: schema::ProductIDs = [1, 2, 3].iter().cloned().collect();
        let b: schema::ProductIDs = [4, 2, 3, 4].iter().cloned().collect();

        let intersection = get_intersection(&a, &b);
        assert_eq!(intersection, [2, 3].iter().cloned().collect());
    }

    #[test]
    fn get_union_works() {
        use super::*;
        let a: schema::ProductIDs = [1, 2, 3].iter().cloned().collect();
        let b: schema::ProductIDs = [4, 2, 3, 4].iter().cloned().collect();

        let union = get_union(&a, &b);
        assert_eq!(union, [1, 2, 3, 4].iter().cloned().collect());
    }

    #[test]
    fn get_jaccard_index_works() {
        use super::*;
        let a: schema::ProductIDs = [3, 5, 7, 9].iter().cloned().collect();
        let b: schema::ProductIDs = [3, 6, 9].iter().cloned().collect();

        let jaccard_index = get_jaccard_index(&a, &b);
        assert_eq!(jaccard_index, 0.4);
    }

    #[test]
    fn get_recommendation_parameters_works() {
        use super::*;
        let a: schema::ProductIDs = [1, 2, 3, 4].iter().cloned().collect();
        let b: schema::ProductIDs = [2, 3, 4, 5].iter().cloned().collect();
        let c: schema::ProductIDs = [1, 3, 5].iter().cloned().collect();

        let d: schema::ProductIDs = [2, 4, 6].iter().cloned().collect();
        let (target, rest) = get_recommendation_parameters();
        assert_eq!(target, [1, 3, 5].iter().cloned().collect());
        assert_eq!(
            rest,
            vec![
                [1, 3, 5].iter().cloned().collect(),
                [1, 3, 5].iter().cloned().collect(),
            ]
        );
    }

    #[test]
    fn get_recommendation_index_works() {
        use super::*;
        let a: schema::ProductIDs = [1, 2, 3, 4].iter().cloned().collect();
        let b: schema::ProductIDs = [2, 3, 4, 5].iter().cloned().collect();
        let c: schema::ProductIDs = [1, 3, 5].iter().cloned().collect();

        let recommend_index = get_recommendation_index(&a, vec![&b, &c]);
        assert_eq!(recommend_index, 0.5);
    }
}
