static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    // println!("Hello, world!");

    {
        let static_string = "I'm in read-only memory";

        println!("static string: {}", static_string);
    }

    {
                // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

            // println!("coerced_static: {}", coerced_static);


    println!("NUM: {} stays accessible!", NUM);
}
