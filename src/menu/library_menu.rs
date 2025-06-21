// Importações:
use crate::clear;
use crate::library::Library;
use std::io;

// Exibe o menu que faz alterações na library passada.
pub(crate) fn show(library: &mut Library) {
    loop {
        // Exibe o título da library.
        title(library.get_name());
        // Chama menu e faz match da opção escolhida.
        match menu() {
            1 => library.add_book(),
            2 => library.remove_book(),
            3 => library.report(),
            4 => library.edit(),
            5 => library.show_books(),
            _ => break,
        }
    }
}

// Printa o título passado.
fn title(name: &str) {
    clear();
    println!("__________{}__________\n\n", name)
}

// Menu que interage com a library
fn menu() -> u8 {
    println!(
        "1 - Adicionar livro.\n\
        2 - Remover livro.\n\
        3 - Exibir relatório.\n\
        4 - Editar biblioteca.\n\
        5 - Visualizar livros.\
        * - Voltar."
    );
    let mut option: String = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Erro ao ler entrada.");
    match option.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
