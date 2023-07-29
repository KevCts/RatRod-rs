pub mod truss;

pub trait Element {
    fn get_matrix(&self) -> [f64;4];
}
