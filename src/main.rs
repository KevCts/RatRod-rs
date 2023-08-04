use model::Model;
use node::Node;
use material::Material;
use section::Section;

use crate::elements::ElementType;

pub mod node;
pub mod model;
pub mod elements;
pub mod material;
pub mod section;

fn main() {
    let mut model = Model::new(2, 0.00001);
    model.add_node(Node {
        x : 0.,
        y : 0.,
        z : 0.
    }).add_node(Node {
        x : 1.,
        y : 0.,
        z : 0.
    }).add_node(Node {
        x : 0.,
        y : 1.,
        z : 0.
    }).add_material(Material {
        e : 1.
    }).add_section(Section {
        s : 1.
    }).add_element(ElementType::Truss, vec![0,1], 0, 0)
    .add_element(ElementType::Truss, vec![1,2], 0, 0)
    .add_u_boundary_condition(0, 0, 0.)
    .add_u_boundary_condition(0, 1, 0.)
    .add_u_boundary_condition(2, 0, 0.)
    .add_u_boundary_condition(2, 1, 0.)
    .set_force(1, 1, -1.);

    model.solve();

    let u = model.u;
    let f = model.f;

    println!("u = {u:?}");
    println!("f = {f:?}");

}
