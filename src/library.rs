// Importações:
use crate::{book::Book, clear};
use std::io;

// Struct de Library
pub(super) struct Library {
    // Nome da biblioteca.
    name: String,
    // Nome do dono da biblioteca.
    owner: String,
    // Livros da biblioteca.
    books: Vec<Book>,
}

impl Library {
    // Função que cria uma nova Library.
    pub(super) fn new() -> Library {
        Library {
            name: Self::set_name(),
            owner: Self::set_owner(),
            books: Vec::new(),
        }
    }

    // Adiciona um novo livro.
    pub(super) fn add_book(&mut self) {
        self.books.push(Book::new());
    }

    // Remove um livro.
    pub(super) fn remove_book(&mut self) {
        clear();
        // Armazena a opção escolhida.
        let mut option: String = String::new();
        // Contador para print dos livros.
        let mut counter: u8 = 1;
        println!("Qual livro deseja deletar?");
        for book in self.books.iter() {
            println!("{} - {}.", counter, book.get_title());
            counter += 1;
        }
        println!("* - Voltar.");
        io::stdin()
            .read_line(&mut option)
            .expect("Erro ao ler entrada.");
        match option.trim().parse::<u8>() {
            Ok(num) => {
                if num < 1 {
                } else {
                    let index: usize = (num - 1) as usize;
                    self.books.remove(index);
                }
            }
            Err(_) => {}
        }
    }

    // Printa os dados da biblioteca:
    pub(super) fn report(&self) {
        clear();
        println!("__________{}__________\n", self.name);
        println!("{} percente a: {}.\n", self.name, self.owner);
        println!(
            "Atualmente conta com um acervo de {} {}.",
            self.books.len(),
            if self.books.len() > 1 {
                "livros"
            } else {
                "livro"
            }
        );
        println!("\n\n\nPressione ENTER para continuar.");
        io::stdin()
            .read_line(&mut String::new())
            .expect("Erro ao ler entrada.");
    }

    // Edita os dados da biblioteca:
    pub(super) fn edit(&mut self) {
        // String para armazenar opção escolhida.
        loop {
            clear();
            println!("__________{}__________\n", self.name);
            let mut option: String = String::new();
            println!(
                "1 - Editar nome.\n\
                2 - Editar nome do dono.\n\
                3 - Resetar livros armazenados.\n\
                * - Voltar."
            );
            io::stdin()
                .read_line(&mut option)
                .expect("Erro ao ler entrada.");
            match option.trim().parse::<u8>() {
                Ok(num) => match num {
                    1 => self.name = Self::set_name(),
                    2 => self.owner = Self::set_owner(),
                    3 => self.books.clear(),
                    _ => break,
                },
                Err(_) => {}
            }
        }
    }

    // Exibe menu com livros:
    pub(super) fn show_books(&self) {
        loop {
            clear();
            let mut option: String = String::new();
            let mut counter: u8 = 1;
            println!("Qual livro deseja inspecionar?\n");
            for book in self.books.iter() {
                println!("{} - {}.", counter, book.get_title());
                counter += 1;
            }
            print!("* - Voltar.");
            io::stdin()
                .read_line(&mut option)
                .expect("Erro ao ler entrada");
            match option.trim().parse::<u8>() {
                Ok(num) => {
                    if num < 1 {
                        break;
                    } else {
                        self.books[(num - 1) as usize].inspect();
                    }
                }
                Err(_) => {}
            }
        }
    }

    // Getters:
    pub(super) fn get_name(&self) -> &str {
        &self.name
    }

    // Setters:
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
