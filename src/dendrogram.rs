use svg::node::element::{Path, path::Data};

type DendrogramDocument = svg::Document;
pub type ConnectionTuple = (usize, usize, i32); // First Index (in stack), Second Index (in stack), Year they should be connected at

pub fn generate(languoids: Vec<(String, i32)>, connections: &mut Vec<ConnectionTuple>) -> DendrogramDocument {
    let mut ret = DendrogramDocument::populate(languoids);

    /*
    while connections.len() != 0 {
        ret.connect(connections.pop().unwrap());
    }
    */
    
    ret.to_owned()
}

trait DendrogramDocumentTrait {
    fn connect(&mut self, connection: ConnectionTuple) -> ();
    fn populate(nodes: Vec<(String, i32)>) -> Self;
}

impl DendrogramDocumentTrait for DendrogramDocument {
    /// Creates a new connection in the DendrogramDocument according to the ConnectionTuple supplied.
    fn connect(&mut self, connection: ConnectionTuple) -> () {
        // TODO: add line(s) connecting the two nodes in the DendrogramDocument
    }

    /// Populates the starting nodes into the DendrogramDocument.
    fn populate(nodes: Vec<(String, i32)>) -> Self {

        let data = Data::new()
            .move_to((10, 10))
            .line_by((0, 50))
            .line_by((50, 0))
            .line_by((0, -50))
            .close();

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data);

       svg::Document::new().set("viewBox", (0, 0, 70, 70)).add(path)
    }
}