use std::cmp;
use crate::*;

let mut languages: Vec<TreeNodeRef> = Vec::new();

fn age_of_common_ancestor(distance: u16, age_a: i32, age_b: i32) -> i32 {
    if distance == 0 {
        return cmp::max(age_a, age_b);
    }
    1 //testing
}

fn combine(a: usize, b: usize, new_year: i32) {
    let mut new_node: TreeNode = TreeNode {
        val: None,
        left: Some(languages[a]),
        right: Some(languages[b]),
        year: new_year,
    };
    if a > b {
        languages.remove(a);
        languages.remove(b);
    } else {
        languages.remove(b);
        languages.remove(a);
    }

    languages.push(Rc::new(RefCell::new(new_node)));
}

fn iterate_minimum_distance_model() {
    let mut best_match = (0, 0);
    let mut best_match_value: u16 = u16::MAX;
    for i in 0..(languages.len()-1) {
        for j in (i+1)..languages.len() {
            let this_match_value = compare::compare(languages[i], languages[j]);
            if this_match_value < best_match_value {
                best_match_value = this_match_value;
                best_match = (i,j);
            }
        }
    }

    combine(best_match.0 as usize, best_match.1 as usize, age_of_common_ancestor(best_match_value, year(languages[best_match.0]), year(languages[best_match.1])));
}

fn minimum_distance_model() {
    while languages.len() > 1 {
        iterate_minimum_distance_model();
    }
}