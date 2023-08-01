use sparse_matrix::{matrix::coo_mat::CooMat, vector::Vector};

#[derive(Debug)]
pub struct Model{
    pub dimension               :   u8,
    pub f                       :   Vector, 
    pub u                       :   Vector, 
    pub m                       :   CooMat, 
    pub f_r                     :   Vector, 
    pub u_r                     :   Vector, 
    pub m_r                     :   CooMat, 
    pub u_boundary_conditions   :   Vec<Option<f64>>
}

impl Model{
    pub fn new(dim : u8) -> Self{
        Model {
            dimension               : dim,
            f                       :   Vector::null(2 * ((3.0_f64).powf((dim as f64) - 1.)) as usize),
            u                       :   Vector::null(2 * ((3.0_f64).powf((dim as f64) - 1.)) as usize),
            m                       :   CooMat::new(2 * ((3.0_f64).powf((dim as f64) - 1.)) as usize,2 * ((3.0_f64).powf((dim as f64) - 1.) as usize)),
            f_r                     :   Vector::null(0),
            u_r                     :   Vector::null(0),
            m_r                     :   CooMat::new(2 * ((3.0_f64).powf((dim as f64) - 1.)) as usize,2 * ((3.0_f64).powf((dim as f64) - 1.) as usize)),
            u_boundary_conditions : vec![None; 2 * ((3.0_f64).powf((dim as f64) - 1.)) as usize]
        }
    }

    pub fn solve(&mut self) -> &Self{
        self.reduce();
        let p_r = self.m_r.to_csr();
        self.u_r = p_r.minres(&self.f_r, 0.).unwrap();
        self.developp();
        let problem = self.m.to_csr();
        self.f = (&problem * &self.u).unwrap();
        self
    }

    pub fn reduce(&mut self) -> &Self {
        self.m_r  = self.m.clone();
        let mut reduced_vector = vec![];
        for i in 0..self.u_boundary_conditions.len(){
            if self.u_boundary_conditions[i] != None {
                self.m_r.drop_row(i);
                self.m_r.drop_col(i);
                self.m_r.rows -= 1;
                self.m_r.columns -= 1;
            } else {
                reduced_vector.push(self.f.values[i]);
            }
        }
        self.f_r = Vector { values : reduced_vector };
        self
    }

    pub fn developp(&mut self) -> &Self {
        let mut j = 0;
        for i in 0..self.u_boundary_conditions.len(){
            match self.u_boundary_conditions[i] {
                Some(value) => self.u.values[i] = value,
                None => {
                    self.u.values[i] = self.u_r.values[j];
                    j+=1;
                },
            }
        }
        self
    }
}
