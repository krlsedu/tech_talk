use std::env;

mod basico;
mod structs;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}",args);

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
