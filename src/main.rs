use probable_broccoli::model::BinaryTree;
use probable_broccoli::dendrogram::ConnectionTuple;
use probable_broccoli::dendrogram;
use probable_broccoli::*;

fn main() {
    let mut model = BinaryTree::from_folder("Languages");
    let mut connections: Vec<ConnectionTuple> = model.0.naive_minimum_distance_model();

    println!("{}", model.0.get_debug_representation());
    println!("{:?}", connections);

    let doc = dendrogram::generate(model.1, connections.as_mut());

    svg::save("image.svg", &doc).unwrap();


    //println!("{}", _ASPECT_IMPERFECT | _ASPECT_PERFECTIVE)
}

