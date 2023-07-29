use sparse_matrix::matrix::coo_mat::CooMat;

use crate::{node::Node, material::Material, section::Section};

#[derive(Debug)]
pub struct Truss<'a, 'b, 'c, 'd> {
    pub nodes : (&'a Node, &'b Node),
    pub material : &'c Material,
    pub section : &'d Section

}

impl Truss<'_, '_, '_, '_> {
    pub fn get_length(&self) -> f64{
        let (node1, node2) = self.nodes;
        ((node1.x - node2.x)*(node1.x - node2.x) + (node1.y - node2.y)*(node1.y - node2.y) + (node1.z - node2.z)*(node1.z - node2.z)).sqrt()
    }

    pub fn get_matrix(&self) -> CooMat{
        let l = self.get_length();
        let stiffness = self.material.e * self.section.s / l;
        let (node1, node2) = self.nodes;
        let c = [(node1.x - node2.x)/l, (node1.y - node2.y)/l, (node1.z - node2.z)/l];
        let mut matrix = CooMat::new(12, 12);

        for i in 0..3 {
            for j in 0..3 {
                matrix.add(i, j, stiffness * c[i] * c[j]);
                matrix.add(i + 6, j, -1. * stiffness * c[i] * c[j]);
                matrix.add(i, j + 6, -1. * stiffness * c[i] * c[j]);
                matrix.add(i + 6, j + 6, stiffness * c[i] * c[j]);
            }
        }

        matrix
    }
}
