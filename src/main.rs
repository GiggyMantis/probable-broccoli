use serde::{Serialize, Deserialize};
use serde_arrays;
use std::{cell::RefCell, rc::Rc, fs, fs::read_to_string, collections::HashMap};
use probable_broccoli::model::BinaryTree;
use probable_broccoli::dendrogram::ConnectionTuple;
use probable_broccoli::*;

fn main() {
    let mut model = BinaryTree::from("Languages", vec!["Old_English", "Latin", "Old_Frisian", "Ancient_Greek", "Gothic", "Old_Norse", "Modern_Dutch", "Afrikaans", "Old_High_German", "Old_Polish", "Old_Irish", "Old_Czech", "Generic_Romani", "Russian", "Bulgarian"]);

    let connections: Vec<ConnectionTuple> = model.naive_minimum_distance_model();

    println!("{}", model.get_debug_representation());
    println!("{:?}", connections)
    //println!("{}", serde_json::to_string_pretty(&lang.clone()).unwrap());

    //println!("{}", MOOD_INDICATIVE + MOOD_CONDITIONAL + MOOD_IMPERATIVE)
}

