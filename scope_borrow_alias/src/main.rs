struct Point {x: i32, y: i32, z: i32}

fn main() {
    // println!("Hello, world!");

    let mut point = Point {x: 0, y: 0, z: 0};

    {
        let borrowed_point = &point;
        let another_borrowed = &point;

        println!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x,
            another_borrowed.y,
            point.z
        );

    }

    {
        let mutale_borrow = &mut point;

        mutale_borrow.x = 5;
        mutale_borrow.y = 2;
        mutale_borrow.z = 1;


        println!("Point has coordinates: ({}, {}, {})",
            mutale_borrow.x,
            mutale_borrow.y,
            mutale_borrow.z,
        );
    }

    let borrowed_point = &point;

    println!("Point now has cooridnates: ({}, {}, {})",
        borrowed_point.x,
        borrowed_point.y,
        borrowed_point.z
    );
}
