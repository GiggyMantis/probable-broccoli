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
    if a > b { // This if statement prevents having to check if the location of language b was moved when a was removed or vice versa.
        languages.remove(a);
        languages.remove(b);
    } else {
        languages.remove(b);
        languages.remove(a);
    }

    languages.push(Rc::new(RefCell::new(new_node)));
}

// Iterates one time on the binary tree using the minimum distance model.
// The two closest languages are joined and returned back to the tree.
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

// Naive Minimum Distance Model
// tries to join *all* languages using the minimum distance model.
// this approach assumes all languages are related, which may be false in the case of conlangs and is debated in the case of natural languages.
// https://en.wikipedia.org/wiki/Minimum-distance_estimation
// Does not allow for this: https://en.wikipedia.org/wiki/Language_isolate
fn naive_minimum_distance_model() {
    while languages.len() > 1 {
        iterate_minimum_distance_model();
    }
}

// TODO: Chronic Minimum Distance Model
// Tries to join languages using the minimum distance model, 
// but only when the date of the NCA is recent.
// https://en.wikipedia.org/wiki/Minimum-distance_estimation
// Prevents this from forming: https://en.wikipedia.org/wiki/Proto-human_language

// TODO: Linguistic Similarity Minimum Distance Model
// Tries to join languages using the minimum distance model,
// but only when the probabalistic likelihood (using Bayesian statistics) is above a certain value.
// https://en.wikipedia.org/wiki/Minimum-distance_estimation
// Allows this to occur: https://en.wikipedia.org/wiki/Polygenesis_(linguistics)

// TODO: Bayesian Model
// https://en.wikipedia.org/wiki/Bayesian_statistics
// Tries to join languages using Bayesian statistics.
// This model joins languages in order of most to least likely to be related.
// The Bayesian model takes into account more variables than the Minimum Distance Model,
// But may not be confident enough to create a tree of the desired depth.

// TODO: Most Parsimonious Model
// https://en.wikipedia.org/wiki/Maximum_likelihood_estimation#See_also
// Tries to join languages in a way that requires the least amount of homoplasy.
// https://en.wikipedia.org/wiki/Maximum_parsimony_(phylogenetics)