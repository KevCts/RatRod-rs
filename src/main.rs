use node::Node;
use elements::truss::Truss;
use material::Material;
use section::Section;
use sparse_matrix::vector::Vector;

mod node;
mod elements;
mod material;
mod section;

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

    let matrix = test.get_matrix();

    let problem = matrix.to_csr();

    let f = Vector {values : vec![2100000.,0.,0.,0.,0.,0.,-2100000.,0.,0.,0.,0.,0.]};

    let u = problem.minres(f, 0.);

    println!("{u:?}")
}
