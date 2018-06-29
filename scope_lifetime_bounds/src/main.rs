use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T) where T: Debug {
    println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("`print`: t is {:?}", t);
}

fn main() {
    // println!("Hello, world!");
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);

}
