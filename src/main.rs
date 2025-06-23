mod vector;

use vector::vector::Vector;

fn main() {
    let mut v = Vector::new(1, false, Box::new(|x| x * 2));

    v.push(2);
    v.push(3);

    println!("{:#?}", v);
    println!("{}", v);
}
