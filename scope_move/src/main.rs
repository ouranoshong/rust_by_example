
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains: {}", c);

    println!("Destroying againt! {}", c);
}

fn main() {
    // println!("Hello, world!");

    let x = 5u32;
    let y = x; // *Copy* not resource to move

    println!("x is {}, y is {}", x, y);

    
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    let b = a;

    // println!("a contains: {}", a);

    destroy_box(b);

    println!("b contains: {}", b);
}
