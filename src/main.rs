use node::Node;

mod node;

fn main() {
    let test = Node {
        x : 1,
        y : 2,
        z : 3
    };
    
    println!("{test:?}");
}
