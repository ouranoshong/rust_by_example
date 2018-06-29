#[allow(dead_code)]
#[derive(Copy, Clone)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: i32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrow {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;

    println!("I mutable borrow {} - {} edittion", book.title, book.year);
}

fn main() {
    // println!("Hello, world!");

    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };


    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    borrow_book(&mutabook);
}
