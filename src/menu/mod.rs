use crate::clear;
pub mod library_menu;
use std::io;

pub(super) fn initial(version: u8) -> u8 {
    clear();
    let mut option: String = String::new();
    println!("__________Bibliotecas__________\n\n");
    if version == 0 {
        println!(
            "Deseja cadastrar uma nova Biblioteca?\n\
            1 - SIM.\n\
            * - SAIR.\n"
        );
    } else if version == 1 {
        println!(
            "1 - Acessar biblioteca.\n\
            2 - Criar nova biblioteca.\n\
            3 - Deletar biblioteca.\n\
            * - Sair."
        );
    }
    io::stdin()
        .read_line(&mut option)
        .expect("Erro ao ler entrada.");
    clear();
    match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
