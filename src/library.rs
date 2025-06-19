pub(super) struct Library {
    name: String,
    owner: String,
    books: u32,
}

impl Library {
    pub(super) fn new_lib() -> Library {
        Library {
            name: String::from("Teste"),
            owner: String::from("Teste2"),
            books: 3,
        }
    }

    pub(super) fn teste(&self) {
        println!("Temos aqui: {}, {}, {}", self.name, self.owner, self.books)
    }
}
