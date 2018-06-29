#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier
}

fn main() {
    use Status::{Poor, Rich};

    use Work::*;

    let status = Poor;

    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilian work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
