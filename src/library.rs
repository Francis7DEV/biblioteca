use crate::{book::Book, clear};
use std::io;

pub(super) struct Library {
    name: String,
    owner: String,
    books: Vec<Book>,
}

impl Library {
    pub(super) fn new() -> Library {
        Library {
            name: Self::set_name(),
            owner: Self::set_owner(),
            books: Vec::new(),
        }
    }

    pub(super) fn get_name(&self) -> &str {
        &self.name
    }

    pub(super) fn get_owner(&self) -> &str {
        &self.owner
    }

    pub(super) fn get_book(&self, index: usize) -> &Book {
        &self.books[index]
    }

    pub(super) fn add_book(&mut self) {
        &mut self.books.push(Book::new());
    }

    fn set_name() -> String {
        clear();
        let mut name: String = String::new();
        println!("Qual o nome da sua biblioteca?");
        io::stdin()
            .read_line(&mut name)
            .expect("Erro ao ler entrada!");
        String::from(name.trim())
    }

    fn set_owner() -> String {
        clear();
        let mut owner: String = String::new();
        println!("Qual o nome do dono da biblioteca?");
        io::stdin()
            .read_line(&mut owner)
            .expect("Erro ao ler entrada!");
        String::from(owner.trim())
    }
}
