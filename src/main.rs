mod book;
mod library;
mod menu;
use clearscreen;
use library::Library;
use menu::library_menu;
use std::{io::stdin, process, thread, time};

fn main() {
    let mut librarys: Vec<Library> = Vec::new();
    greeting();
    loop {
        if librarys.len() == 0 {
            println!("Nenhuma Biblioteca ainda cadastrada!!");
            sleep(1.0);
            match menu::initial(0) {
                1 => {
                    librarys.push(Library::new());
                }
                _ => process::exit(0),
            }
        } else {
            match menu::initial(1) {
                1 => {
                    acess(&librarys);
                }
                2 => librarys.push(Library::new()),
                _ => process::exit(0),
            }
        }
    }
}

fn greeting() {
    clear();
    println!("__________Bem vindo!__________\n\n");
    sleep(1.0);
    println!("Este código tem como intuito estudar o uso de structs em Rust.");
    sleep(1.5);
    println!("Faremos isso através de um sistema fictício de gerenciamento de bibliotecas.");
    sleep(2.5);
    clear();
}

fn acess(librarys: &Vec<Library>) {
    let mut counter: u8 = 1;
    let mut option: String = String::new();
    clear();
    println!("Qual biblioteca deseja acessar?\n");
    for lib in librarys.iter() {
        println!("{} - {}.", counter, lib.get_name());
        counter += 1;
    }
    println!("* - Voltar.");
    stdin()
        .read_line(&mut option)
        .expect("Erro ao ler entrada.");
    match option.trim().parse::<u8>() {
        Ok(num) => {
            if num < 1 {
            } else {
                let index = (num - 1) as usize;
                library_menu::show(&librarys[index]);
            }
        }
        Err(_) => {}
    };
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
