use std::cmp;

// This Module (compare) compares two languages and returns a "distance" score.
// The higher a distance score is, the more *distantly* two languages are related.

fn compare_individual(lect_a: Box<Languoid>, lect_b: Box<Languoid>) -> u16 {
    if lect_a == lect_b {
        return 0;
    }
    1;
}

fn compare(lect_a: Box<Tree<Languoid>>, lect_b: Box<Tree<Languoid>>) -> u16 {
    
}

fn age_of_common_ancestor(distance: u16, age_a: i32, age_b: i32, language_change_rate: f64) -> i32 {
    if distance == 0 {
        return cmp::max(age_a, age_b);
    }
}