

fn main() {
    // println!("Hello, world!");

    let mut _mutable_integer = 7i32;

    {
        let _large_integer = &_mutable_integer;

        // _mutable_integer = 50;
    }

    _mutable_integer = 30;
}
