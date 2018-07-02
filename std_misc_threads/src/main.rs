use std::thread;

static NTHREADS: i32 = 10;

fn main() {
    // println!("Hello, world!");

    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("This is thread number: {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
