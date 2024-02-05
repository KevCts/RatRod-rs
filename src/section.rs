use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Section {
    pub s : f64, 
    pub i : f64,
}
