// Importações:
use std::io;

#[derive(Debug)]
// Struct de Book
pub(super) struct Book {
    // Título do livro.
    title: String,
    // Nome do autor.
    author: String,
    // Número de páginas.
    n_pages: u32,
    // Ano de publicação.
    y_publication: String,
}

impl Book {
    // Função que cria um novo livro.
    pub(super) fn new() -> Book {
        Book {
            title: Self::set_title(),
            author: Self::set_author(),
            n_pages: Self::set_n_pages(),
            y_publication: Self::set_y_publication(),
        }
    }

    // Getters:
    pub(super) fn get_title(&self) -> &str {
        &self.title
    }

    // Setters:
    fn set_title() -> String {
        let mut title: String = String::new();
        println!("Qual o título do Livro?");
        io::stdin()
            .read_line(&mut title)
            .expect("Erro ao ler entrada!");
        String::from(title.trim())
    }

    fn set_author() -> String {
        let mut author: String = String::new();
        println!("Qual o nome do Autor?");
        io::stdin()
            .read_line(&mut author)
            .expect("Erro ao ler entrada!");
        String::from(author.trim())
    }

    fn set_n_pages() -> u32 {
        let mut n_pages: String = String::new();
        println!("Quantas páginas tem o livro?");
        io::stdin()
            .read_line(&mut n_pages)
            .expect("Erro ao ler entrada!");
        match n_pages.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        }
    }

    fn set_y_publication() -> String {
        let mut y_publication: String = String::new();
        println!("Em que ano foi publicado?");
        io::stdin()
            .read_line(&mut y_publication)
            .expect("Erro ao ler entrada!");
        String::from(y_publication.trim())
    }
}
