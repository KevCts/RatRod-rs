# RatRod.rs
An FEM implementation written in Rust

## Disclaimer
This library is mainly a personal project. It's neither qualified neither guarantied and should not be used to assess actual mechanical structures.

## How to use it ?
The library is built around the Model class.

This class allows to describe the model to be studied using a factory pattern as below :

```
        let mut model = Model::new(2, 0.00000001);
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
```

Then, `model.solve()`, `model.u` and `model.f` can be used to solve the model and get the displacements and forces on the nodes.
