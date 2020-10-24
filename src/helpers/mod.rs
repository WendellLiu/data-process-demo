use crate::entity;

use std::collections::HashSet;

pub fn get_intersection(
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

pub fn get_union(
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

        let intersection = get_union(&a, &b);
        assert_eq!(intersection, [1, 2, 3, 4].iter().cloned().collect());
    }
}
