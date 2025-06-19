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
            owner: String::from("Dono"),
            books: 8,
        }
    }

    fn set_name() -> String {
        let mut name: String = String::new();
        println!("Qual o nome da sua biblioteca?");
        io::stdin()
            .read_line(&mut name)
            .expect("Erro ao ler entrada!");
        String::from(name.trim())
    }

    pub(super) fn get_name(&self) -> &str {
        &self.name
    }
}
