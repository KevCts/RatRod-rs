use sparse_matrix::{matrix::coo_mat::CooMat, vector::Vector};

#[derive(Debug, PartialEq)]
pub struct Model {
    pub dimension  :  u8,
    pub u_boundary_conditions : Vec<Option<f64>>
}

impl Model {
    pub fn new(dim : u8) -> Model {
        Model {
            dimension : dim,
            u_boundary_conditions : vec![None; 2 * ((3.0_f64).powf((dim as f64) - 1.)) as usize]
        }
    }

    pub fn reduce(&self, mat : &CooMat, vec : &Vector) -> (CooMat, Vector) {
        let mut reduced_matrix = mat.clone();
        let mut reduced_vector = vec![];
        for i in 0..self.u_boundary_conditions.len(){
            if self.u_boundary_conditions[i] != None {
                reduced_matrix.drop_row(i);
                reduced_matrix.drop_col(i);
                reduced_matrix.rows -= 1;
                reduced_matrix.columns -= 1;
            } else {
                reduced_vector.push(vec.values[i]);
            }
        }
        (reduced_matrix,Vector { values : reduced_vector })
    }

    pub fn developp(&self, v : Vector) -> Vector {
        let mut augmented = vec![];
        let mut j = 0;
        for i in 0..self.u_boundary_conditions.len(){
            match self.u_boundary_conditions[i] {
                Some(value) => augmented.push(value),
                None => {
                    augmented.push(v.values[j]);
                    j+=1;
                },
            }
        }
        Vector { values : augmented }
    }
}
