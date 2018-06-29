fn main() {
    // println!("Hello, world!");


    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("This is an rustacean among us!"),
                _ => println!("Hello {}", name)
            }
        }
    }

    {
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("This is an rustacean among us!"),
                _ => println!("Hello {}", name)
            }
        }
    }


    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            match name {
                &mut "Ferris" => println!("This is an rustacean among us!"),
                _ => println!("Hello {}", name)
            }
        }
    }
}
