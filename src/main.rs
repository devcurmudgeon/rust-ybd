use std::cmp::Ordering;

struct Chunk {
    name: &'static str,
    deps: Vec<&'static str>
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
    let x = Chunk { name : "foo", deps : vec!["bar", "baz"] };
    let y = Chunk { name : "bar", deps : Vec::new() };
    let z = Chunk { name : "baz", deps : Vec::new() };
    println!("Is x a dependency of y: {}", x < y);
    println!("Is y a dependency of x: {}", y < x);
    println!("Is z a dependency of x: {}", z < x);
    println!("Is y a dependency of z: {}", y < z);
}
