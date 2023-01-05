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

// This makes it possible to print Book values with {}.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("Book[{}]{}.", book.title, book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut result = &self.books[0];
        for book in &self.books {
            if book.year < result.year {
                result = book;
            }
        }
        Some(result)
    }
}

#[cfg(test)]
mod library_test {
    use super::*;

    #[test]
    fn test_main() {
        // This shows the desired behavior. Uncomment the code below and
        // implement the missing methods. You will need to update the
        // method signatures, including the "self" parameter!
        let mut library = Library::new();

        println!("Our library is empty: {}", library.is_empty());

        library.add_book(Book::new("Lord of the Rings", 1954));
        library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

        library.print_books();

        match library.oldest_book() {
            Some(book) => println!("My oldest book is {}", book),
            None => println!("My library is empty!"),
        }
        if let Some(book) = library.oldest_book() {
            assert_eq!(book.year, 1865);
        }

        println!("Our library has {} books", library.len());
        assert_eq!(2, library.len());
    }
}
