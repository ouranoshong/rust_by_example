#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NameBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Ether<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    // println!("Hello, world!");

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NameBorrowed {x: &x, y: &y };
    let number = Ether::Num(y);
    let reference = Ether::Ref(&x);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
