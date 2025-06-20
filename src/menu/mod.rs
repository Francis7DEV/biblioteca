// Importações:
use crate::clear;
pub mod library_menu;
use std::io;

// Menu inicial:
pub(super) fn initial(version: u8) -> u8 {
    // Limpa o terminal.
    clear();
    // String para armazenar a option escolhida.
    let mut option: String = String::new();
    println!("__________Bibliotecas__________\n\n");
    // Se receber 0 como version:
    if version == 0 {
        println!(
            "Deseja cadastrar uma nova Biblioteca?\n\
            1 - SIM.\n\
            * - SAIR.\n"
        );
    // Se receber 1 como version:
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
