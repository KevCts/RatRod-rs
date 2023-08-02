use model::Model;
use node::Node;
use material::Material;
use section::Section;
use sparse_matrix::vector::Vector;

use crate::elements::ElementType;

pub mod node;
pub mod model;
pub mod elements;
pub mod material;
pub mod section;

fn main() {
    let mut model = Model::new(3);
    model.add_node(Node {
        x : 0.,
        y : 0.,
        z : 0.
    }).add_node(Node {
        x : 1.,
        y : 0.,
        z : 0.
    }).add_material(Material {
        e : 210_000.
    }).add_section(Section {
        s : 10.
    }).add_element(ElementType::Truss, vec![0,1], 0, 0);

    model.u_boundary_conditions[0] = Some(0.);
    model.u_boundary_conditions[1] = Some(0.);
    model.u_boundary_conditions[2] = Some(0.);

    model.f = Vector {values : vec![2100000., 0., 0., 2100000., 0., 0.]};

    model.solve();

    let u = model.u;
    let f = model.f;

    println!("{u:?}");
    println!("{f:?}");

}
