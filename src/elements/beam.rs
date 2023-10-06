use sparse_matrix::matrix::coo_mat::CooMat;

use std::rc::Rc;

use crate::{node::Node, material::Material, section::Section};

#[derive(Debug, PartialEq)]
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
        let c = [(node1.x - node2.x)/l, (node1.y - node2.y)/l];
        let mut matrix = CooMat::new(mat_size, mat_size);
        for i in 0..2 {
            for j in 0..2 {
                matrix.add_value(3*x + i, 3*x + j, truss_stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, 3*x + j, -1. * truss_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i, j + 3*y, -1. * truss_stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, j + 3*y, truss_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*x + j + 1, 12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*x + j + 1, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*x + j + 2, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*x + j + 2, 4. * l * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*x + j + 1, -12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*x + j + 1, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*x + j + 2, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*x + j + 2, 2. * l * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*y + j + 1, -12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*y + j + 1, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*y + j + 2, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*y + j + 2, 2. * l * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*y + j + 1, 12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*y + j + 1, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*y + j + 2, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*y + j + 2, 4. * l * l * beam_stiffness * c[i] * c[j]);
            }
        }

        matrix
    }

    pub fn get_3_d_matrix(&self, mat_size : usize, x:usize, y:usize) -> CooMat {
        let l = self.get_length();
        let truss_stiffness = self.material.e * self.section.s / l;
        let beam_stiffness = self.material.e * self.section.i / l / l / l;
        let (node1, node2) = &self.nodes;
        let c = [(node1.x - node2.x)/l, (node1.y - node2.y)/l, (node1.z - node2.z)/l];
        let mut matrix = CooMat::new(mat_size, mat_size);

        for i in 0..3 {
            for j in 0..3 {
                matrix.add_value(3*x + i, 3*x + j, truss_stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, 3*x + j, -1. * truss_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i, j + 3*y, -1. * truss_stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, j + 3*y, truss_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*x + j + 1, 12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*x + j + 1, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*x + j + 2, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*x + j + 2, 4. * l * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*x + j + 1, -12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*x + j + 1, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*x + j + 2, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*x + j + 2, 2. * l * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*y + j + 1, -12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*y + j + 1, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 1, 3*y + j + 2, 6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i + 2, 3*y + j + 2, 2. * l * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*y + j + 1, 12. * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*y + j + 1, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 1, 3*y + j + 2, -6. * l * beam_stiffness * c[i] * c[j]);
                matrix.add_value(3*y + i + 2, 3*y + j + 2, 4. * l * l * beam_stiffness * c[i] * c[j]);
            }
        }

        matrix
    }
}
