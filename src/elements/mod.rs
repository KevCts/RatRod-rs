use sparse_matrix::matrix::coo_mat::CooMat;

use self::truss::Truss;

pub mod truss;

#[derive(Debug)]
pub enum Element<'a> {
    Truss(Truss<'a>),
}

pub trait ElementTrait {
    fn get_matrix(&self, dimension : u8) -> CooMat;
}

impl ElementTrait for Element<'_> {
    fn get_matrix(&self, dimension : u8) -> CooMat {
        match self {
            Element::Truss(truss) => truss.get_matrix(dimension),
        }
    }
}
