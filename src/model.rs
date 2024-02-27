use std::option::Option;
use std::cmp;
use std::fs::read_to_string;
use std::time::Duration;
use indicatif;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressFinish, ProgressStyle};
use crate::*;

const RATE_OF_LANGUAGE_CHANGE: f32 = 1.0;

#[derive(Debug)]
pub struct BinaryTree {
    pub(crate) val: Vec<TreeNodeRef>,
}
impl BinaryTree {
    pub fn from(folder: &str, filepaths: Vec<&str>) -> BinaryTree {
        let mut v = Vec::<TreeNodeRef>::new();
        for fp in filepaths.iter() {
            v.push(get_node_from_languoid(Box::new(serde_json::from_str(&*read_to_string(format!("{}/{}.json", folder, fp)).unwrap()).unwrap())));
        }
        return BinaryTree {
            val: v,
        };
    }

    pub fn get_debug_representation(&self) -> String {
        return Self::get_debug_representation_of_node(self.val.first().unwrap());
    }

    fn get_debug_representation_of_node(n: &TreeNodeRef) -> String {
        return if n.val().is_some() {
            n.val().unwrap().languoid_name
        } else {
            format!("({}, {})", Self::get_debug_representation_of_node(&n.left().unwrap()), Self::get_debug_representation_of_node(&n.right().unwrap()))
        }
    }

    pub fn get_svg_representation(&self, connections: Vec<(usize, usize, i32)>) -> svg::Document {
        dendrogram::generate(
            self.get_languoid_names_and_years(),
            connections
        )
    }

    pub fn get_languoid_names_and_years(&self) -> Vec<(String, i32)> {
        let mut ret: Vec<(String, i32)> = Vec::new();
        let mut todo_list: Vec<TreeNodeRef> = Vec::new();
        todo_list.push(self.val[0].clone());
        while (todo_list.len() > 0) {
            let tuple = get_children(todo_list.pop().unwrap());
            if tuple.0.is_some() {
                ret.push(tuple.0.unwrap());
            }
            if tuple.1.is_some() {
                todo_list.push(tuple.1.unwrap());
            }
            if tuple.2.is_some() {
                todo_list.push(tuple.2.unwrap());
            }
        } 

        ret
    }

    pub fn combine(&mut self, a: usize, b: usize, new_year: i32) {
        let new_node: TreeNode = TreeNode {
            val: None,
            left: Some(self.val[a].clone()),
            right: Some(self.val[b].clone()),
            year: new_year,
        };
        if a > b { // This if statement prevents having to check if the location of language b was moved when a was removed or vice versa.
            self.val.remove(a);
            self.val.remove(b);
        } else {
            self.val.remove(b);
            self.val.remove(a);
        }
    
        self.val.push(Rc::new(RefCell::new(new_node)));
    }

    // Iterates one time on the binary tree using the minimum distance model.
    // The two closest languages are joined and returned back to the tree.
    pub fn iterate_minimum_distance_model(&mut self) -> (usize, usize, i32) {
        let matchups: u64 = ((0.5) * (self.val.len() as f64) * ((self.val.len() - 1) as f64)) as u64;
        let pb = ProgressBar::new(matchups).with_position(0);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{percent:>2}% [{bar}] {pos:>9}/{len:9} [{elapsed_precise}] [ eta {eta} ]")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );
        pb.set_draw_target(ProgressDrawTarget::stderr());
        let mut best_match = (0usize, 0usize);
        let mut best_match_value: u16 = u16::MAX;
        for i in 0..self.val.len()-1
        {
            for j in i+1..self.val.len()
            {
                let this_match_value = compare::compare(self.val[i].clone(), self.val[j].clone());

                pb.inc(1);
                if this_match_value < best_match_value {
                    best_match_value = this_match_value;
                    best_match = (i,j);
                }
            }
        }
        pb.finish_and_clear();
        // println!("Joining {} and {}. Distance = {}", best_match.0, best_match.1, best_match_value);
        let best = (best_match.0, best_match.1, age_of_common_ancestor(best_match_value, self.val[best_match.0].year(), self.val[best_match.1].year()));
        self.combine(best.0, best.1, best.2);
        
        best
    }

    // Naive Minimum Distance Model
    // tries to join *all* languages using the minimum distance model.
    // this approach assumes all languages are related, which may be false in the case of conlangs and is debated in the case of natural languages.
    // https://en.wikipedia.org/wiki/Minimum-distance_estimation
    // Does not allow for this: https://en.wikipedia.org/wiki/Language_isolate
    pub fn naive_minimum_distance_model(&mut self) -> Vec<(usize, usize, i32)> {
        let mut ret: Vec<(usize, usize, i32)> = Vec::new();
        let original_length = self.val.len();
        while self.val.len() > 1 {
            println!("Working... Step {} / {}", original_length - self.val.len() + 1, original_length - 1);
            ret.push(self.iterate_minimum_distance_model());
        }

        ret
    }

    // TODO: Chronic Minimum Distance Model
    // Tries to join languages using the minimum distance model,
    // but only when the date of the NCA is recent.
    // https://en.wikipedia.org/wiki/Minimum-distance_estimation
    // Prevents this from forming: https://en.wikipedia.org/wiki/Proto-human_language

    // TODO: Linguistic Similarity Minimum Distance Model
    // Tries to join languages using the minimum distance model,
    // but only when the probabilistic likelihood (using Bayesian statistics) is above a certain value.
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


}

// TODO: Fix... this *waves hands*
fn age_of_common_ancestor(distance: u16, age_a: i32, age_b: i32) -> i32 {
    let min = cmp::min(age_a, age_b);
    if distance == 0 {
        return min;
    }
    
    cmp::min(min, (RATE_OF_LANGUAGE_CHANGE * -(distance as f32) + (min as f32) - (f32::abs((age_a - age_b) as f32))) as i32)
}

fn get_children(t: TreeNodeRef) -> (Option<(String, i32)>, Option<TreeNodeRef>, Option<TreeNodeRef>) {
    (
        if t.val().is_some() {
            Some((t.val().unwrap().languoid_name, t.val().unwrap().year))
        } else {
            None
        },
        t.left(), t.right()
    )
}