use model::Model;
use node::Node;
use elements::truss::Truss;
use material::Material;
use section::Section;
use sparse_matrix::vector::Vector;

pub mod node;
pub mod model;
pub mod elements;
pub mod material;
pub mod section;

fn main() {
    let test1 = Node {
        x : 0.,
        y : 0.,
        z : 0.
    };

    let test2 = Node {
        x : 1.,
        y : 0.,
        z : 0.
    };

    let mat = Material {
        e : 210_000.
    };

    let sec = Section {
        s : 10.
    };

    let test = Truss {
        nodes       :   (&test1, &test2),
        material    :   &mat,
        section     :   &sec,
    };

    let mut model = Model::new(1);

    let matrix = test.get_matrix(model.dimension);

    let mut f = Vector {values : vec![2100000.,2100000.]};

    model.u_boundary_conditions[0] = Some(0.);

    let reduced_problem = model.reduce(&matrix, &f);

    let (m_r, f_r) = reduced_problem;

    let p_r = m_r.to_csr();

    let u_r = p_r.minres(f_r.clone(), 0.).unwrap();

    let u = model.developp(u_r.clone());

    let problem = matrix.to_csr();

    f = (&problem * &u).unwrap();

    println!("{u:?}");
    println!("{f:?}");

}
