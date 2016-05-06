use std::cmp::Ordering;

struct Chunk {
    name: String,
    deps: Vec<String>
}

impl PartialEq for Chunk {
    fn eq(&self, other : &Chunk) -> bool {
        &self.name == &other.name
    }
}

impl PartialOrd for Chunk {
    fn partial_cmp(&self, other : &Chunk) -> Option<Ordering> {
        if self.deps.contains(&other.name) {
            Some(Ordering::Greater)
        } else if other.deps.contains(&self.name) {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

fn main() {
    let x = Chunk { name : String::from("foo"), deps : vec![String::from("bar"), String::from("baz")] };
    let y = Chunk { name : String::from("bar"), deps : Vec::new() };
    let z = Chunk { name : String::from("baz"), deps : Vec::new() };
    println!("Is x a dependency of y: {}", x < y);
    println!("Is y a dependency of x: {}", y < x);
    println!("Is z a dependency of x: {}", z < x);
    println!("Is y a dependency of z: {}", y < z);
}
