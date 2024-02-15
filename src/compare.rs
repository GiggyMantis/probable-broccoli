use std::cmp;
use edit_distance::edit_distance;
use crate::*;

// This Module (compare) compares two languages and returns a "distance" score.
// The higher a distance score is, the more *distantly* two languages are related.

fn compare_individual(lect_a: Box<Languoid>, lect_b: Box<Languoid>) -> u16 {
    if lect_a == lect_b {
        return 0;
    }

    let mut normalized_levenshtein_distance = 0;
    for n in 0..100 {
        normalized_levenshtein_distance += edit_distance(&lect_a.leipzig_jakarta_list[n], &lect_b.leipzig_jakarta_list[n]) / cmp::max(lect_a.leipzig_jakarta_list[n].len(), lect_b.leipzig_jakarta_list[n].len());
    }
    
    1 //testing
}

fn compare(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    if val(&lect_a) != None && val(&lect_b) != None {
        return compare_individual(val(&lect_a).unwrap(), val(&lect_b).unwrap());
    }
    if val(&lect_a) != None {
        return compare_languoid_and_fam(lect_a, lect_b);
    }
    if val(&lect_b) != None {
        return compare_languoid_and_fam(lect_b, lect_a);
    }

    return compare_fam_and_fam(lect_a, lect_b);
}

fn compare_fam_and_fam(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    let left_distance = compare(left(&lect_a).unwrap(), lect_b.clone());
    let right_distance = compare(right(&lect_a).unwrap(), lect_b.clone());
    
    (left_distance + right_distance) / 2
}

fn compare_languoid_and_fam(lect_a: TreeNodeRef, lect_b: TreeNodeRef) -> u16 {
    let left_distance = compare(lect_a.clone(), left(&lect_b).unwrap());
    let right_distance = compare(lect_a.clone(), right(&lect_b).unwrap());

    (left_distance + right_distance) / 2
}

fn age_of_common_ancestor(distance: u16, age_a: i32, age_b: i32, language_change_rate: f64) -> i32 {
    if distance == 0 {
        return cmp::max(age_a, age_b);
    }
    1 //testing
}