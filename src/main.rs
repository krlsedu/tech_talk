#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::env;

mod basico;
mod crud;
mod structs;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}",args);
    crud::crud::start();

    if args.contains(&"-w".to_string()) {
        basico::declaracao_basica::contem_warnings();
    }

    for arg in args {
        match arg.as_str() {
            "-w" => basico::declaracao_basica::contem_warnings(),
            "-s" => structs::structs::struts_basic(),
            _ => println!("{}",arg)
        }
    }
}
