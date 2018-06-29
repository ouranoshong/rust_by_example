

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    
    println!("{:?} months in a year.", 12);

    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");

    println!("Now {:?} will print!", DebugPrintable(3));

    println!("Now {:?} will print!", Deep(DebugPrintable(3)));

    let name = "Peter";
    let age = 9;
    let peter = Person{ name, age };

    //Pretty Print
    println!("{:#?}", peter);

}
