use svg::{Document, Path};
use svg::node::element::Path;
use svg::node::element::path::Data;


pub fn generate(languoids: Vec<(String, i32)>, connections: Vec<(usize, usize, i32)>) -> Document {
    let mut ret = Document::new();

    ret
}