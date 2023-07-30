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
}
