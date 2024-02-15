use std::cmp;
use crate::*;

let mut languages: Vec<TreeNodeRef> = Vec::new();

fn age_of_common_ancestor(distance: u16, age_a: i32, age_b: i32, language_change_rate: f64) -> i32 {
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

}