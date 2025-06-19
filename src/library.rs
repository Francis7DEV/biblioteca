use std::io;

pub(super) struct Library {
    name: String,
    owner: String,
    books: u32,
}

impl Library {
    pub(super) fn new_lib() -> Library {
        Library {
            name: Self::set_name(),
            owner: Self::set_owner(),
            books: 0,
        }
    }

    pub(super) fn get_name(&self) -> &str {
        &self.name
    }

    pub(super) fn get_owner(&self) -> &str {
        &self.owner
    }

    pub(super) fn get_books(&self) -> &u32 {
        &self.books
    }

    fn set_name() -> String {
        let mut name: String = String::new();
        println!("Qual o nome da sua biblioteca?");
        io::stdin()
            .read_line(&mut name)
            .expect("Erro ao ler entrada!");
        String::from(name.trim())
    }

    fn set_owner() -> String {
        let mut owner: String = String::new();
        println!("Qual o nome do dono da biblioteca?");
        io::stdin()
            .read_line(&mut owner)
            .expect("Erro ao ler entrada!");
        String::from(owner.trim())
    }
}
