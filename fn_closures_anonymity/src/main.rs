fn apply<F>(f: F) where
    F: Fn() {
    f();    
}

fn main() {
    // println!("Hello, world!");

    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}
