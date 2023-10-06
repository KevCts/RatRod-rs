use sparse_matrix::matrix::coo_mat::CooMat;

use self::{truss::Truss, beam::Beam};

pub mod truss;
pub mod beam;

#[derive(Debug)]
pub enum Element {
    Truss(Truss),
    Beam(Beam),
}

pub enum ElementType {
    Truss,
    Beam
}

pub trait ElementTrait {
    fn get_matrix(&self, dimension : u8, mat_size : usize, nodes : &Vec<usize>) -> CooMat;
}

impl ElementTrait for Element {
    fn get_matrix(&self, dimension : u8, mat_size : usize, nodes : &Vec<usize>) -> CooMat {
        match self {
            Element::Truss(truss) => truss.get_matrix(dimension, mat_size, nodes[0], nodes[1]),
            Element::Beam(beam) => beam.get_matrix(dimension, mat_size, nodes[0], nodes[1]),
        }
    }
}
