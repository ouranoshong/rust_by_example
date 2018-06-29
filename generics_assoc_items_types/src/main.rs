
struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {

    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (number_1 == &self.0) && (number_2 == &self.1)
    }

    fn first(&self) -> i32 { self.0 }

    fn last(&self) -> i32 { self.1 } 
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.first() - container.last()
}

fn main() {
    // println!("Hello, world!");

        let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
