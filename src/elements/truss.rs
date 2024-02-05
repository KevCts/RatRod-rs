use sparse_matrix::matrix::coo_mat::CooMat;

use std::rc::Rc;

use crate::{node::Node, material::Material, section::Section};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Truss {
    pub nodes : (Rc<Node>, Rc<Node>),
    pub material : Rc<Material>,
    pub section : Rc<Section>

}

impl Truss {
    pub fn get_length(&self) -> f64{
        let (node1, node2) = &self.nodes;
        ((node1.x - node2.x)*(node1.x - node2.x) + (node1.y - node2.y)*(node1.y - node2.y) + (node1.z - node2.z)*(node1.z - node2.z)).sqrt()
    }

    pub fn get_matrix(&self, dimension : u8, mat_size : usize, i:usize, j:usize) -> CooMat{
        match dimension {
            1 => self.get_1_d_matrix(mat_size, i, j),
            2 => self.get_2_d_matrix(mat_size,i, j),
            3 => self.get_3_d_matrix(mat_size,i, j),
            _ => panic!("Invalid dimension")
        }
    }

    pub fn get_1_d_matrix(&self, mat_size : usize, i:usize, j:usize) -> CooMat {
        let l = self.get_length();
        let stiffness = self.material.e * self.section.s / l;
        let mut matrix = CooMat::new(mat_size,mat_size);
        matrix.add_value(i, i, stiffness);
        matrix.add_value(j, i, -1. * stiffness);
        matrix.add_value(i, j, -1. * stiffness);
        matrix.add_value(j, j, stiffness);
        matrix
    }

    pub fn get_2_d_matrix(&self, mat_size : usize, x:usize, y:usize) -> CooMat {
        let l = self.get_length();
        let stiffness = self.material.e * self.section.s / l;
        let (node1, node2) = &self.nodes;
        let c = [(node1.x - node2.x)/l, (node1.y - node2.y)/l];
        let mut matrix = CooMat::new(mat_size, mat_size);
        for i in 0..2 {
            for j in 0..2 {
                matrix.add_value(3*x + i, 3*x + j, stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, 3*x + j, -1. * stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i, j + 3*y, -1. * stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, j + 3*y, stiffness * c[i] * c[j]);
            }
        }

        matrix
    }

    pub fn get_3_d_matrix(&self, mat_size : usize, x:usize, y:usize) -> CooMat {
        let l = self.get_length();
        let stiffness = self.material.e * self.section.s / l;
        let (node1, node2) = &self.nodes;
        let c = [(node1.x - node2.x)/l, (node1.y - node2.y)/l, (node1.z - node2.z)/l];
        let mut matrix = CooMat::new(mat_size, mat_size);

        for i in 0..3 {
            for j in 0..3 {
                matrix.add_value(3*x + i, 3*x + j, stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, 3*x + j, -1. * stiffness * c[i] * c[j]);
                matrix.add_value(3*x + i, j + 3*y, -1. * stiffness * c[i] * c[j]);
                matrix.add_value(i + 3*y, j + 3*y, stiffness * c[i] * c[j]);
            }
        }

        matrix
    }
}
