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
    let mut ret: DendrogramDocument = Document::new().set("viewBox", (0, 0, 250 + (nodes.len()+5) * 10, model::RATE_OF_LANGUAGE_CHANGE / 3.5)).add(Style::new(
        ".small {
        font: bold 2.0px sans-serif;
        }"
    ));
    let mut data = Data::new();
    let mut stack: Vec<(f64, f64)> = Vec::new();

    for (_, &ref node) in nodes.iter().enumerate() {
        let i = nodes_in_order.iter().position(|r| r == node).unwrap();
        let cord = ((100 + (i * 10)) as f64, get_y_from_year(nodes_in_order[i].1));
        let s = &nodes_in_order[i].0;
        let length = f64::min(1.0 * (s.len() as f64), 13.0);
        ret = ret.add(Comment::new(s)).add(Circle::new().set("cx", cord.0).set("cy", cord.1).set("r", 1.2)).add(element::Text::new(s).set("x", cord.0 - (length/2.0)).set("y", cord.1 - 2.0).set("class", "small").set("textLength", length));
        data = data.move_to((i, node.1));
        stack.push(cord);
    }

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
            .move_to(left)
            .line_to((left.0, new_cord.1))
            .line_to(new_cord)
            .line_to((right.0, new_cord.1))
            .line_to(right);

        let p = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.5)
            .set("d", d);

        ret = ret.add(p);

        stack.push(new_cord);
    }

    ret
}

#[inline(always)]
fn get_y_from_year(year: i32) -> f64 {
    ((2500 - year) as f64) / 50.0
}