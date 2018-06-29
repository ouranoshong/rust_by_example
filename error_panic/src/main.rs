
fn give_princess(gift: &str) {
    if gift == "snake" { panic!("AAAaaaaaa!"); }

    println!("I love {}s", gift);
}

fn main() {
    // println!("Hello, world!");

    give_princess("teddy bear");
    give_princess("snake");
}
