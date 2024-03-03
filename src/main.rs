use probable_broccoli::model::BinaryTree;
use probable_broccoli::dendrogram::ConnectionTuple;
use probable_broccoli::*;

fn main() {
    let mut model = BinaryTree::from_folder("Languages");
    let connections: Vec<ConnectionTuple> = model.naive_minimum_distance_model();

    println!("{}", model.get_debug_representation());
    println!("{:?}", connections)

    //println!("{}", _ASPECT_IMPERFECT | _ASPECT_PERFECTIVE)
}

