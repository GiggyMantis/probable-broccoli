using svg::{Document, Path};

pub(crate) struct TreeKeyString {
    pub(crate) languoids: Vec<(String, i32)>,
    pub(crate) connections: Vec<(usize, usize, i32)>,
}

pub fn generate(tks: TreeKeyString) -> Document {

}