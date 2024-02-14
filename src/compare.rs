use std::cmp;

// This Module (compare) compares two languages and returns a "distance" score.
// The higher a distance score is, the more *distantly* two languages are related.

fn compare_individual(lect_a: Box<Languoid>, lect_b: Box<Languoid>) -> u16 {
    if lect_a == lect_b {
        return 0;
    }
    1; //testing
}

fn compare(lect_a: RefCell<TreeNode>, lect_b: RefCell<TreeNode>) -> u16 {
    if lect_a.val != NONE && lect_b.val != NONE {
        return compare_individual(lect_a.val.unwrap(), lect_b.val.unwrap());
    }
    if lect_a.val != NONE {
        return compare_languoid_and_fam(lect_a, lect_b);
    }
    if lect_b.val != NONE {
        return compare_languoid_and_fam(lect_b, lect_a);
    }

    return compare_fam_and_fam(lect_a, lect_b);
}

fn compare_fam_and_fam(lect_a: Box<TreeNode>, lect_b: RefCell<TreeNode>) -> u16 {
    let left_distance = compare(lect_a.left.unwrap().borrow(), lect_b);
    let right_distance = compare(lect_a.right.unwrap().borrow(), lect_b);
    
    (left_distance + right_distance) / 2;
}

fn compare_languoid_and_fam(lect_a: Box<Languoid>, lect_b: RefCell<TreeNode>) -> u16 {
    let left_distance = compare(lect_a, lect_b.left.unwrap().borrow());
    let right_distance = compare(lect_a, lect_b.right.unwrap().borrow());

    (left_distance + right_distance) / 2;
}

fn age_of_common_ancestor(distance: u16, age_a: i32, age_b: i32, language_change_rate: f64) -> i32 {
    if distance == 0 {
        return cmp::max(age_a, age_b);
    }
    1; //testing
}