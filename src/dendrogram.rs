use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;
use svg::Document;
use svg::node::element::{Circle, Path, path::Data, Style};
use svg::node::{Comment, element};
use svg::node::element::tag::Style;
use crate::model::age_of_common_ancestor;
use crate::{model, TreeNode};

type DendrogramDocument = Document;
pub type ConnectionTuple = (usize, usize, i32); // First Index (in stack), Second Index (in stack), Year they should be connected at

pub fn generate(nodes: Vec<(String, i32)>, nodes_in_order: Vec<(String, i32)>, connections: &mut Vec<(usize, usize, i32)>) -> DendrogramDocument {
    let height = 10 + ((nodes.len()+1) * 20);
    let width = 10.0 + (model::RATE_OF_LANGUAGE_CHANGE / 3.5);
    let mut ret: DendrogramDocument = Document::new().set("viewBox", (0, 0, width, height)).add(Style::new(
        "   .s {
        font: bold 3.0px sans-serif;
        }"
    ));
    let mut data = Data::new();
    let mut stack: Vec<(f64, f64)> = Vec::new();

    for (_, &ref node) in nodes.iter().enumerate() {
        let i = nodes_in_order.iter().position(|r| r == node).unwrap();
        let cord = ((100 + (i * 10)) as f64, get_y_from_year(nodes_in_order[i].1));
        let s = &nodes_in_order[i].0;
        ret = ret.add(Comment::new(s)).add(Circle::new().set("cy", cord.0).set("cx", cord.1).set("r", 1.2)).add(element::Text::new(s).set("y", cord.0 + 1.0).set("x", cord.1 + 2.0).set("class", "s"));
        data = data.move_to((i, node.1));
        stack.push(cord);
    }
    ret = ret.add(Comment::new("Lines"));

    for ct in connections.iter() {
        let left: (f64, f64);
        let right: (f64, f64);

        if ct.0 < ct.1 {
            left = stack.remove(ct.1);
            right = stack.remove(ct.0);
        } else {
            left = stack.remove(ct.0);
            right = stack.remove(ct.1);
        }

        let new_cord = ((left.0 + right.0) / 2.0, get_y_from_year(ct.2));

        let d = Data::new()
            .move_to((left.1, left.0))
            .line_to((new_cord.1, left.0))
            .line_to((new_cord.1, new_cord.0))
            .line_to((new_cord.1, right.0))
            .line_to((right.1, right.0));

        let p = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.5)
            .set("d", d);

        ret = ret.add(p);

        stack.push(new_cord);
    }

    let s = stack.pop().unwrap();

    let d = Data::new()
        .move_to((s.1, s.0))
        .line_to((0.0, s.0));

    let p = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.5)
        .set("d", d);

    ret = ret.add(p);

    ret
}

#[inline(always)]
fn get_y_from_year(year: i32) -> f64 {
    (((6000 + year) as f64) / 75.0) + 75.0
}