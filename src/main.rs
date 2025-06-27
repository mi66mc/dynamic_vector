mod vector;

use vector::vector::Vector;

fn main() {
    let mut v = Vector::new(1, false, Box::new(|x| {
        if x == 1 {
            return 2
        }
        x * x
    }));

    v.push(2);
    v.push(3);
    v.push(2);
    v.push(3);

    v.push(2);
    v.push(3);

    println!("{:#?}", v);
    println!("{}", v);

    v.reallocate(10);
    let a:Vec<i32> = vec![];

    println!("{:#?}", v);
    println!("{}", v);
}
