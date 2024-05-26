use std::io;
mod ascii;
use ascii::logo;
fn main() {
    println!("{}",logo::get_logo());
    start_controller();
}
fn start_controller() {
    let mut opt: i32 = 1;
    while opt != 0 {
        println!("\n[1] - variáveis\n[2]- funções\n[3]- laços de repetição\n[0] - sair");
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("erro ao computar sua resposta!");

        opt = response.trim().parse::<i32>().expect("use apenas numeros!");

        if opt == 1 {
            resume_variables()
        }
        if opt == 2 {
            resume_functions()
        }
        if opt == 3 {
            resume_repeat_flows()
        }
        if opt == 0 {
            println!("bye bye")
        }
    }
}

fn resume_repeat_flows() {
    println!("\n\n Fluxos de repetição são responsáveis por executar um bloco de código n vezes conforme uma condição.");
    println!(" É importante ter uma condição possível para que não ocorra um loop e o programa nunca termine.");
    println!(" Os laços de repetição em Rust podem ser:\n");

    println!("While:");
    println!(
        "um laço de repetição que depende de uma condição, declarado dessa forma: while condição"
    );
    println!("exemplo de um while de incremento em uma variável mutável");

    let mut while_index = 0;

    while while_index < 5 {
        println!("incremento while {}", while_index);
        while_index += 1;
    }

    println!("\nFor:");
    println!(
        "o for no rust atua de uma forma um pouco diferente, ele pode ser usado para consumir matrizes de forma segura."
    );
    let matrix=[1,2,3,4,9,100];
    
    for element in matrix{
        println!("item da matriz: {element}")
    }
}

fn resume_functions() {
    println!("\n\nFunções são blocos de código executados após sua chamada");
    println!("As funções em rust são declaradas com o prefixo fn");
    println!("As funções podem ter parametros onde o tipo de dado deve ser explicitado:");
    println!("fn sum(x:f32, y:f32)...");
    println!("para chamar uma função em determinada parte do programa basta user 'nomedafunção(parametro1)'.  ");
    println!("uma função pode retornar um valor que pode por exemplo ser atribuido a uma variavel");
    let sum: i32 = exem_sum(1, 2);
    fn exem_sum(x: i32, y: i32) -> i32 {
        return x + y;
    }
    println!("a variável sum = exem_sum(1,2) terá o valor {}", sum);
}

fn resume_variables() {
    print!(
        "\n\nvariáveis alocam dados em endereços de memória durante a execução de um programa\n"
    );
    println!("Para declarar uma variável é necessário o prefixo 'let' seguido pelo nome da variável e a atribuição de seu valor.");
    println!("Por exemplo: let variable=12;");

    print!("As variáveis em rust são imutáveis(a menos que seja adicionado 'mut' após o prefixo 'let'.\n");
    let mut mutable: i32 = 1;
    println!("por exemplo a variável mutable vale: {}", mutable);
    mutable += 1;
    println!("após um incremento mutable agora vale: {}", mutable);

    println!("você pode adicionar um tipo logo após nomear a variável\n por exemplo: let variable:f32=10;");
    println!("variáveis possuem tipos primitivos os principais em rust são:");

    let integer: i32 = 1;
    let float: f32 = 1.50;
    let boolean: bool = true;
    let character: char = 'A';

    println!("inteiros -> {} [declarados como i32]", integer);
    println!("flutuantes -> {} [declarados como f32]", float);
    println!("booleanos -> {} [declarados como bool]", boolean);
    println!("caracteres -> {} [declarados como char]", character);
}
