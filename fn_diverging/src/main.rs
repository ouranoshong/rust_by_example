fn main() {
    // println!("Hello, world!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {

                true => i,
                false => continue,
            };

            acc += addition;
        }
        acc
    }

    let up_to = 10;

    println!("Sum of odd numbers up to {} (excluding): {}", up_to, sum_odd_numbers(up_to));
}
