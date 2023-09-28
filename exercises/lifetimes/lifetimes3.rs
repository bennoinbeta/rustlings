// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

// References in struct need explicit definition of liftime
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name: String = String::from("Jill Smith");
    let title: String = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
