use std::{collections::HashMap, rc::Rc, fs::{self, File}, process::exit, io::Write};

use sparse_matrix::{matrix::coo_mat::CooMat, vector::Vector};

use crate::{elements::{Element, ElementType, truss::Truss, beam::Beam, ElementTrait}, node::Node, material::Material, section::Section};

use serde::{Serialize, Deserialize};

use serde_json_any_key::any_key_map;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Model{
    pub epsilon                 :   f64,
    pub dimension               :   u8,
    pub degrees_of_freedom      :   u8,
    pub f                       :   Vector, 
    pub u                       :   Vector, 
    pub m                       :   CooMat, 
    pub f_r                     :   Vector, 
    pub u_r                     :   Vector, 
    pub m_r                     :   CooMat, 
    pub u_boundary_conditions   :   Vec<Option<f64>>,
    #[serde(with = "any_key_map")]
    pub elements                :   HashMap<Vec<usize>,Box<Element>>,
    pub nodes                   :   Vec<Rc<Node>>,
    pub materials               :   Vec<Rc<Material>>,
    pub sections                :   Vec<Rc<Section>>
}

impl Model{
    pub fn new(dim : u8, epsilon : f64) -> Self{
        let dof = match dim {
            1 => 1,
            2 => 3,
            3 => 6,
            _ => panic!()
        };
        Model {
            epsilon,
            degrees_of_freedom      :   dof,
            dimension               :   dim,
            f                       :   Vector::null(0),
            u                       :   Vector::null(0),
            m                       :   CooMat::new(0, 0),
            f_r                     :   Vector::null(0),
            u_r                     :   Vector::null(0),
            m_r                     :   CooMat::new(0,0),
            u_boundary_conditions   :   vec![],
            elements                :   HashMap::new(),
            nodes                   :   vec![],
            materials               :   vec![],
            sections                :   vec![]
        }
    }

    pub fn save(&self, file: &str) {
        let to_write = serde_json::to_string(&self).expect("Unable to serialize model");
        fs::write(file, to_write).expect("Unable to write file");
    }

    pub fn load(file: &str) -> Self {
        let file_content = fs::read_to_string(file).expect("Unable to read file");
        serde_json::from_str(&file_content).expect("Unable to process the file")
    }

    pub fn add_element(&mut self, element : ElementType, nodes : Vec<usize>, material : usize, section : usize) -> &mut Self{
        self.elements.insert(nodes.clone(), Box::new(
                match element {
                    ElementType::Truss => Element::Truss(Truss { nodes : (self.nodes[nodes[0]].clone(), self.nodes[nodes[1]].clone()), material : self.materials[material].clone(), section : self.sections[section].clone()}),
                    ElementType::Beam => Element::Beam(Beam { nodes : (self.nodes[nodes[0]].clone(), self.nodes[nodes[1]].clone()), material : self.materials[material].clone(), section : self.sections[section].clone()})
                }));
        self
    }

    pub fn add_node(&mut self, node : Node) -> &mut Self {
        self.nodes.push(Rc::new(node));
        self.m.rows += self.degrees_of_freedom as usize;
        self.m.columns += self.degrees_of_freedom as usize;
        for _ in 0..self.degrees_of_freedom{
            self.f.values.push(0.);
            self.u.values.push(0.);
            self.u_boundary_conditions.push(None);
        }
        self
    }

    pub fn add_material(&mut self, material : Material) -> &mut Self {
        self.materials.push(Rc::new(material));
        self
    }

    pub fn add_section(&mut self, section : Section) -> &mut Self {
        self.sections.push(Rc::new(section));
        self
    }

    pub fn solve(&mut self) -> &mut Self{
        for (nodes, element) in &self.elements {
            self.m += element.get_matrix(self.dimension, self.m.rows, nodes);
        }
        self.reduce();
        let p_r = self.m_r.to_csr();
        self.u_r = p_r.minres(&self.f_r, self.epsilon).unwrap();
        self.developp();
        let problem = self.m.to_csr();
        self.f = (&problem * &self.u).unwrap();
        self
    }

    pub fn reduce(&mut self) -> &mut Self {
        self.m_r  = self.m.clone();
        let mut reduced_vector = vec![];
        let mut j = 0;
        for i in 0..self.u_boundary_conditions.len(){
            if self.u_boundary_conditions[i] != None {
                self.m_r.drop_row(j);
                self.m_r.drop_col(j);
                self.m_r.rows -= 1;
                self.m_r.columns -= 1;
            } else {
                reduced_vector.push(self.f.values[i]);
                j+=1;
            }
        }
        self.f_r = Vector { values : reduced_vector };
        self
    }

    pub fn developp(&mut self) -> &mut Self {
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

    pub fn add_u_boundary_condition(&mut self, node : usize, field : u8, value : f64) -> &mut Self {
        self.u_boundary_conditions[self.degrees_of_freedom  as usize * node + field as usize] = Some(value);
        self
    }

    pub fn remove_u_boundary_condition(&mut self, node : usize, field : u8) -> &mut Self {
        self.u_boundary_conditions[self.degrees_of_freedom  as usize * node + field as usize] = None;
        self
    }

    pub fn set_force(&mut self, node : usize, field : u8, value : f64) -> &mut Self{
        self.f.values[self.degrees_of_freedom as usize * node + field as usize] = value;
        self
    }
}
