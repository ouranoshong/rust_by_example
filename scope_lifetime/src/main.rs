fn main() {
    // println!("Hello, world!");

    let i = 3;

    {
        let borrow1 = &i;

        println!("borrow1 is {}", borrow1);
    }

    {
        let borrow2 = &i;

        println!("borrow2 is {}", borrow2);
    }

    
}
