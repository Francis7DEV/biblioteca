use crate::clear;
use crate::library::Library;
use std::io;

pub(crate) fn show(library: &mut Library) {
    loop {
        title(library.get_name());
        match menu() {
            1 => library.add_book(),
            _ => break,
        }
    }
}

fn title(name: &str) {
    clear();
    println!("__________{}__________\n\n", name)
}

fn menu() -> u8 {
    println!(
        "1 - Adicionar livro.\n\
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
