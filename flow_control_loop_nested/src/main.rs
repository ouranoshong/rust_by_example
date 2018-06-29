#![allow(unreachable_code)]

fn main() {
    // println!("Hello, world!");

    'outer: loop {
        println!("Entered the outer loop");


        'inner: loop {

            println!("Entered the inner loop");


            break 'outer;

        }

        println!("The point will never be reached!");
    }

    println!("Exited the outer loop");
}
