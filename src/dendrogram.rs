use svg::node::element::{Path, path::Data};

type DendrogramDocument = svg::Document;
pub(crate) type ConnectionTuple = (usize, usize, i32); // First Index (in stack), Second Index (in stack), Year they should be connected at

pub fn generate(languoids: Vec<(String, i32)>, connections: mut Vec<ConnectionTuple>) -> DendrogramDocument {
    let mut ret = DendrogramDocument::new();

    while connections.len() != 0 {
        ret.connect(connections.pop());
    }
    
    ret
}

trait DendrogramDocumentTrait {
    fn connect(&mut self, connection: ConnectionTuple) -> ();
    fn populate(&mut self, Vec<(String, i32)>) -> ();
}

impl DendrogramDocumentTrait for DendrogramDocument {
    fn connect(&mut self, connection: ConnectionTuple) -> () {
        // TODO: add line(s) connecting the two nodes in the DendrogramDocument
    }
}