struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.1 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
    
}

fn defference<A, B, C>(container: &C) -> i32 where 
    C: Contains<A, B> {
        (container.last() - container.first()).abs()
    }

fn main() {
    // println!("Hello, world!");

    let number_1 = 10;
    let number_2 = 4;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));

    println!("Fisrt number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The defference is: {}", defference(&container));
}
