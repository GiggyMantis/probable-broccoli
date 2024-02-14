use std::cmp;

// This Module (compare) compares two languages and returns a "distance" score.
// The higher a distance score is, the more *distantly* two languages are related.

fn compare_individual(lect_a: Option<Box<Languoid>>, lect_b: Option<Box<Languoid>>) -> u16 {
    if lect_a == NONE || lect_b == NONE {
        return 0;
    }
    if lect_a == lect_b {
        return 0;
    }
    1; //testing
}

fn compare(lect_a: Box<TreeNode>, lect_b: Box<TreeNode>) -> u16 {
    return compare_individual(SOME(lect_a::val), SOME(lect_b::val));
}

fn age_of_common_ancestor(distance: u16, age_a: i32, age_b: i32, language_change_rate: f64) -> i32 {
    if distance == 0 {
        return cmp::max(age_a, age_b);
    }
}