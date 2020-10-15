pub fn contem_warnings() {
    println!("Hello, TechTalkers!");
    //let cria o ponteiro para a variavel e gera atribuição, que pode ser por inferência ou declaração explicita
    let variavel_inferencia = "Tipo &str";
    let variavel_declaracaoExplicita: String = "Tipo String".to_string();

    //exemplo de dois warnings:
    //1) camelCase - por padrão usa snake_case
    //2) unused variable
    let variavel_declaracaoExplicitaNaoUsada: String = String::from("Tipo String");

    //exemplo de variável não usada e ignorada nos warnings do compilador
    let _variavel_nao_usada_sem_warning = "";

    //exemplo de erro que o compilador devido mau gerenciamento de memória
    let copia_variavel = &variavel_declaracaoExplicita;
    println!("Eu sou uma variável do {}", copia_variavel);
    println!("Eu sou uma variável do {}", variavel_declaracaoExplicita);
    println!("Esse valor {}", retornaString());
}

#[allow(non_snake_case)]
fn retornaString() -> String {
    let var: String = String::from("Veio da função!");
    //a última linha executada de uma função que não tiver ; no final fica implicito o return
    var
}