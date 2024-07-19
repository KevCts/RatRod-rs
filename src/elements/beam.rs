use sparse_matrix::matrix::coo_mat::CooMat;

use std::rc::Rc;

use crate::{node::Node, material::Material, section::Section};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Beam{
    pub nodes : (Rc<Node>, Rc<Node>),
    pub material : Rc<Material>,
    pub section : Rc<Section>
}

impl Beam{
    pub fn get_length(&self) -> f64{
        let (node1, node2) = &self.nodes;
        ((node1.x - node2.x)*(node1.x - node2.x) + (node1.y - node2.y)*(node1.y - node2.y) + (node1.z - node2.z)*(node1.z - node2.z)).sqrt()
    }

    pub fn get_matrix(&self, dimension : u8, mat_size : usize, i:usize, j:usize) -> CooMat{
        match dimension {
            2 => self.get_2_d_matrix(mat_size,i, j),
            3 => self.get_3_d_matrix(mat_size,i, j),
            _ => panic!("Invalid dimension")
        }
    }

    pub fn get_2_d_matrix(&self, mat_size : usize, x:usize, y:usize) -> CooMat {
        let l = self.get_length();
        let truss_stiffness = self.material.e * self.section.s / l;
        let beam_stiffness = self.material.e * self.section.i / l / l / l;
        let (node1, node2) = &self.nodes;
        let mut matrix = CooMat::new(mat_size, mat_size);
        matrix.add_value(3*x    , 3*x    , truss_stiffness);
        matrix.add_value(3*y    , 3*x    , -1. * truss_stiffness);
        matrix.add_value(3*x    , 3*y    , -1. * truss_stiffness);
        matrix.add_value(3*y    , 3*y    , truss_stiffness);
        matrix.add_value(3*x + 1, 3*x + 1, 12. * beam_stiffness);
        matrix.add_value(3*x + 2, 3*x + 1, 6. * l * beam_stiffness);
        matrix.add_value(3*x + 1, 3*x + 2, 6. * l * beam_stiffness);
        matrix.add_value(3*x + 2, 3*x + 2, 4. * l * l * beam_stiffness);
        matrix.add_value(3*y + 1, 3*x + 1, -12. * beam_stiffness);
        matrix.add_value(3*y + 2, 3*x + 1, 6. * l * beam_stiffness);
        matrix.add_value(3*y + 1, 3*x + 2, -6. * l * beam_stiffness);
        matrix.add_value(3*y + 2, 3*x + 2, 2. * l * l * beam_stiffness);
        matrix.add_value(3*x + 1, 3*y + 1, -12. * beam_stiffness);
        matrix.add_value(3*x + 2, 3*y + 1, -6. * l * beam_stiffness);
        matrix.add_value(3*x + 1, 3*y + 2, 6. * l * beam_stiffness);
        matrix.add_value(3*x + 2, 3*y + 2, 2. * l * l * beam_stiffness);
        matrix.add_value(3*y + 1, 3*y + 1, 12. * beam_stiffness);
        matrix.add_value(3*y + 2, 3*y + 1, -6. * l * beam_stiffness);
        matrix.add_value(3*y + 1, 3*y + 2, -6. * l * beam_stiffness);
        matrix.add_value(3*y + 2, 3*y + 2, 4. * l * l * beam_stiffness);
        let mut rotation = CooMat::new(mat_size, mat_size);
        let c = (node1.x - node2.x).abs()/l;
        let s = (node1.y - node2.y).abs()/l;
        rotation.add_value(3*x    , 3*x    ,  c);
        rotation.add_value(3*x + 1, 3*x    , -s);
        rotation.add_value(3*x    , 3*x + 1,  s);
        rotation.add_value(3*x + 1, 3*x + 1,  c);
        rotation.add_value(3*x + 2, 3*x + 2,  1.);
        rotation.add_value(3*y    , 3*y    ,  c);
        rotation.add_value(3*y + 1, 3*y    , -s);
        rotation.add_value(3*y    , 3*y + 1,  s);
        rotation.add_value(3*y + 1, 3*y + 1,  c);
        rotation.add_value(3*y + 2, 3*y + 2,  1.);

        matrix = &(rotation.transposed()) * &(&matrix * &rotation);

        matrix
    }

    pub fn get_3_d_matrix(&self, mat_size : usize, x:usize, y:usize) -> CooMat {
        unimplemented!();
    }
}
