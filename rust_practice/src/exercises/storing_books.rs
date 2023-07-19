struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    // return the number of books in the library
    fn len(&self) -> usize {
        self.books.len()
    }
    // return true if the library is empty
    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    // add a book to the library
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // print all the books in the library
    fn print_books(&self) {
        for book in &self.books {
            println!("Title: {}, Year: {}", book.title, book.year);
        }
    }

    // return the oldest book in the library
    fn oldest_book(&self) -> Option<&Book> {
        self.books.iter().min_by_key(|book| book.year)
    }
}

pub fn main() {
    let mut library = Library::new();

    println!(
        "The library is empty: {} -> {}",
        "library.is_empty()",
        library.is_empty()
    );

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!(
        "The library is no longer empty: {} -> {}",
        "library.is_empty()",
        library.is_empty()
    );

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}
