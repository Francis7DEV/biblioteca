use std::{thread, time};

fn main() {
    greeting();
}

fn greeting() {
    println!("__________Bem vindo!__________\n");
    thread::sleep(time::Duration::from_secs_f32(1.5));
    println!("Este código tem como intuito estudar o uso de structs em Rust.");
    thread::sleep(time::Duration::from_secs_f32(1.6));
    println!("Faremos isso através de um sistema fictício de gerenciamento de bibliotecas.");
    thread::sleep(time::Duration::from_secs_f32(2.0));
}
