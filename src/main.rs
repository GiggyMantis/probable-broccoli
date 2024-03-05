use probable_broccoli::model::BinaryTree;
use probable_broccoli::dendrogram::ConnectionTuple;
use probable_broccoli::dendrogram;
use probable_broccoli::*;

fn main() {
    let mut model = BinaryTree::from_folder("Languages");
    let mut connections: Vec<(usize, usize, i32)> = model.0.naive_minimum_distance_model();

    let doc = dendrogram::generate(model.1, model.0.get_languoid_names_and_years(), connections.as_mut());

    svg::save("image.svg", &doc).unwrap();


    // println!("{}", _MOOD_INDICATIVE | _ASPECT_IMPERFECTIVE | _MOOD_CONDITIONAL | _MOOD_OPTATIVE)
}

