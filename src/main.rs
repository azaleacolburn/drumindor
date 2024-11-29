mod vector;
fn main() {
    let mut vec = vector::Vector::new();
    vec.push(3);
    vec.push(2);
    vec.push(2);
    vec.push(2);
    vec.push(2);
    println!("{}", vec);
    let t = vec.pop().unwrap();
    println!("{} + {}", vec, t);
}
