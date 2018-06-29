fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    // println!("Hello, world!");

    let closure = || println!("I'm a closure");

    call_me(function);

    call_me(closure);
}
