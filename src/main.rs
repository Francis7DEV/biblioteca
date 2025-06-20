// Importações:

mod book;
mod library;
mod menu;
use clearscreen;
use library::Library;
use menu::library_menu;
use std::{io, process, thread, time};

fn main() {
    // Armazenas as bibliotecas criadas.
    let mut librarys: Vec<Library> = Vec::new();
    // Exibe a saudação inicial.
    greeting();
    loop {
        // Executa se não houver nenhuma biblioteca cadastrada.
        if librarys.len() == 0 {
            println!("Nenhuma Biblioteca ainda cadastrada!!");
            sleep(1.0);
            // Chama o menu na versão 0.
            match menu::initial(0) {
                1 => {
                    // Adiciona library em librarys.
                    librarys.push(Library::new());
                }
                // Encerra a execução.
                _ => process::exit(0),
            }
        // Executa se houver alguma biblioteca já cadastrada.
        } else {
            // Chama o menu na versão 1.
            match menu::initial(1) {
                1 => {
                    // Acessa a biblioteca.
                    acess(&mut librarys);
                }
                // Adiciona library em librarys.
                2 => librarys.push(Library::new()),
                // Deleta library de librarys.
                3 => delete(&mut librarys),
                // Encerra a execução.
                _ => process::exit(0),
            }
        }
    }
}

// Saudação inicial.
fn greeting() {
    // Limpa o terminal.
    clear();
    println!("__________Bem vindo!__________\n\n");
    // Aguarda em segundos.
    sleep(1.0);
    println!("Este código tem como intuito estudar o uso de structs em Rust.");
    sleep(1.5);
    println!("Faremos isso através de um sistema fictício de gerenciamento de bibliotecas.");
    sleep(2.5);
    clear();
}

// Acessa a biblioteca selecionada.
fn acess(librarys: &mut Vec<Library>) {
    // Contador para print das opções.
    let mut counter: u8 = 1;
    // String para armazenar a opção escolhida.
    let mut option: String = String::new();
    // Limpa o terminal.
    clear();
    // Print das opções.
    println!("Qual biblioteca deseja acessar?\n");
    for lib in librarys.iter() {
        println!("{} - {}.", counter, lib.get_name());
        counter += 1;
    }
    println!("* - Voltar.");
    io::stdin()
        .read_line(&mut option)
        .expect("Erro ao ler entrada.");
    match option.trim().parse::<u8>() {
        Ok(num) => {
            // Não faz nada se a entrada for menor que 1.
            if num < 1 {
            } else {
                // Calcula o index (num-1).
                let index = (num - 1) as usize;
                // Chama o menu da biblioteca selecionada.
                library_menu::show(&mut librarys[index]);
            }
        }
        // Se erro não faz nada.
        Err(_) => {}
    };
}

// Deleta a biblioteca selecionada. Mesma lógica de acess.
fn delete(librarys: &mut Vec<Library>) {
    let mut counter: u8 = 1;
    let mut option: String = String::new();
    clear();
    println!("Qual biblioteca deseja DELETAR?\n");
    for lib in librarys.iter() {
        println!("{} - {}.", counter, lib.get_name());
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
                let index = (num - 1) as usize;
                // Remove a library de librarys com base no index.
                librarys.remove(index);
            }
        }
        Err(_) => {}
    };
}

// Função para esperar.
pub(crate) fn sleep(time: f32) {
    thread::sleep(time::Duration::from_secs_f32(time));
}

// Função para limpar o terminal.
pub(crate) fn clear() {
    match clearscreen::clear() {
        Ok(_) => {}
        Err(_) => {
            println!("\n\n\n")
        }
    };
}
