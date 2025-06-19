mod book;
mod library;
use clearscreen;
use library::Library;
use std::{io, process, thread, time};

fn main() {
    let mut librarys: Vec<Library> = Vec::new();
    greeting();
    loop {
        if librarys.len() == 0 {
            println!("Nenhuma Biblioteca ainda cadastrada!!");
            sleep(1.0);
            match menu(0) {
                1 => {
                    librarys.push(Library::new());
                    break;
                }
                _ => process::exit(0),
            }
        } else {
        }
    }
}

fn greeting() {
    clear();
    println!("__________Bem vindo!__________\n");
    sleep(1.0);
    println!("Este código tem como intuito estudar o uso de structs em Rust.");
    sleep(1.5);
    println!("Faremos isso através de um sistema fictício de gerenciamento de bibliotecas.");
    sleep(2.5);
    clear();
}

fn menu(version: u8) -> u8 {
    clear();
    let mut option: String = String::new();
    println!("__________Bibliotecas__________\n\n\n");
    if version == 0 {
        println!(
            "Deseja cadastrar uma nova Biblioteca?\n\n \
            1 - SIM.\n \
            * - SAIR.\n"
        );
    } else if version == 1 {
        println!(
            "1 - Acessar biblioteca.\n \
            2 - Criar nova biblioteca.\n \
            3 - Editar biblioteca.\n \
            4 - Deletar biblioteca.\n \
            * - Sair."
        );
    }
    io::stdin()
        .read_line(&mut option)
        .expect("Erro ao ler entrada.");
    match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}

pub(crate) fn sleep(time: f32) {
    thread::sleep(time::Duration::from_secs_f32(time));
}

pub(crate) fn clear() {
    match clearscreen::clear() {
        Ok(_) => {}
        Err(_) => {
            println!("\n\n\n")
        }
    };
}
