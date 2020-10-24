use crate::entity;

use std::collections::HashSet;

fn get_intersection(
    a: &HashSet<entity::ProductID>,
    b: &HashSet<entity::ProductID>,
) -> HashSet<entity::ProductID> {
    let mut set = HashSet::new();

    let intersection = a.intersection(b);
    for ele in intersection {
        set.insert(ele.clone());
    }

    return set;
}

fn get_union(
    a: &HashSet<entity::ProductID>,
    b: &HashSet<entity::ProductID>,
) -> HashSet<entity::ProductID> {
    let mut set = HashSet::new();

    let union = a.union(b);
    for ele in union {
        set.insert(ele.clone());
    }

    return set;
}

pub fn get_jaccard_index(
    a: &HashSet<entity::ProductID>,
    b: &HashSet<entity::ProductID>,
) -> entity::JaccardIndex {
    let intersection_len = get_intersection(a, b).len() as f64;
    let union_len = get_union(a, b).len() as f64;

    return intersection_len / union_len;
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_intersection_works() {
        use super::*;
        let a: HashSet<entity::ProductID> = [1, 2, 3].iter().cloned().collect();
        let b: HashSet<entity::ProductID> = [4, 2, 3, 4].iter().cloned().collect();

        let intersection = get_intersection(&a, &b);
        assert_eq!(intersection, [2, 3].iter().cloned().collect());
    }

    #[test]
    fn get_union_works() {
        use super::*;
        let a: HashSet<entity::ProductID> = [1, 2, 3].iter().cloned().collect();
        let b: HashSet<entity::ProductID> = [4, 2, 3, 4].iter().cloned().collect();

        let union = get_union(&a, &b);
        assert_eq!(union, [1, 2, 3, 4].iter().cloned().collect());
    }

    #[test]
    fn get_jaccard_index_works() {
        use super::*;
        let a: HashSet<entity::ProductID> = [3, 5, 7, 9].iter().cloned().collect();
        let b: HashSet<entity::ProductID> = [3, 6, 9].iter().cloned().collect();

        let jaccard_index = get_jaccard_index(&a, &b);
        assert_eq!(jaccard_index, 0.4);
    }
}
