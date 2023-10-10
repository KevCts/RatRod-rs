pub mod node;
pub mod model;
pub mod elements;
pub mod material;
pub mod section;

#[cfg(test)]
mod tests {
    use sparse_matrix::vector::Vector;

    use crate::{elements::ElementType, model::Model, node::Node, material::Material, section::Section};

    use more_asserts::assert_le;
    
    #[test]
    fn spring_in_tension() {
        let mut model = Model::new(2, 0.00001);
        model.add_node(Node {
            x : 0.,
            y : 0.,
            z : 0.
        }).add_node(Node {
            x : 1.,
            y : 0.,
            z : 0.
        }).add_material(Material {
            e : 1.
        }).add_section(Section {
            s : 1.,
            i : 1.
        }).add_element(ElementType::Truss, vec![0,1], 0, 0)
        .add_u_boundary_condition(0, 0, 0.)
            .add_u_boundary_condition(0, 1, 0.)
            .set_force(1, 0, 1.);

        model.solve();

        let u = model.u;
        let f = model.f;
        assert_eq!(u, Vector { values: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0] });
        assert_eq!(f, Vector { values: vec![-1.0, 0.0, 0.0, 1.0, 0.0, 0.0] });
    }

    #[test]
    fn beam_in_flexion() {
        let mut model = Model::new(2, 0.00001);
        model.add_node(Node {
            x : 0.,
            y : 0.,
            z : 0.
        }).add_node(Node {
            x : 1.,
            y : 0.,
            z : 0.
        }).add_material(Material {
            e : 1.
        }).add_section(Section {
            s : 1.,
            i : 1.
        }).add_element(ElementType::Beam, vec![0,1], 0, 0)
        .add_u_boundary_condition(0, 0, 0.)
        .add_u_boundary_condition(0, 1, 0.)
        .add_u_boundary_condition(0, 2, 0.)
        .set_force(1, 1, 3.);

        model.solve();

        let u = model.u;
        let f = model.f;
        assert_le!((&(&u - &Vector { values: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.5] }).unwrap() * &(&u - &Vector { values: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.5] }).unwrap()).unwrap().sqrt(), 0.01);
        assert_le!((&(&f - &Vector { values: vec![0.0, -3.0, -3.0, 0.0, 3.0, 0.0] }).unwrap() * &(&u - &Vector { values: vec![0.0, -3.0, -3.0, 0.0, 3.0, 0.0] }).unwrap()).unwrap().sqrt(), 0.01);
    }
}

