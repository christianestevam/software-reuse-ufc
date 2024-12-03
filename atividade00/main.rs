use std::io;

/// Função que calcula a média dos números positivos em um array de inteiros de tamanho fixo.
/// Retorna `Some(média)` se houver números positivos, ou `None` caso contrário.
fn media_positivos(arr: [i32; 10]) -> Option<f32> {
    // Filtra apenas os números positivos do array e os coleta em um vetor
    let positivos: Vec<i32> = arr.iter().cloned().filter(|&x| x > 0).collect();
    
    // Se não houver números positivos, retorna None
    if positivos.is_empty() {
        None
    } else {
        // Calcula a média dos números positivos e a retorna como Some(valor)
        Some(positivos.iter().sum::<i32>() as f32 / positivos.len() as f32)
    }
}

/// Função que calcula o produto de todos os números pares em um array de inteiros de tamanho fixo.
/// Retorna 1 se não houver números pares.
fn produto_pares(arr: [i32; 10]) -> i32 {
    // Filtra apenas os números pares (excluindo zero) e calcula o produto
    arr.iter().filter(|&&x| x % 2 == 0 && x != 0).product()
}

/// Função principal para executar a lógica da QUESTÃO 1
fn questao_01_main() {
    // Define um array fixo de inteiros com 10 elementos
    let numeros = [2, -3, 7, 0, 8, -1, 5, -4, 6, 10];

    // Chama a função media_positivos e trata o resultado usando match
    match media_positivos(numeros) {
        Some(media) => println!("Média dos números positivos: {}", media),
        None => println!("Não há números positivos."),
    }

    // Chama a função produto_pares e exibe o resultado
    let produto = produto_pares(numeros);
    println!("Produto dos números pares: {}", produto);
}

/// Função auxiliar para ler um número inteiro inserido pelo usuário.
/// Retorna o número lido ou exibe um erro se a entrada for inválida.
fn ler_inteiro() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
    input.trim().parse().expect("Por favor, insira um número inteiro válido")
}

/// Função que analisa uma tupla de três inteiros, retornando uma nova tupla com:
/// - A soma dos três números,
/// - O maior número,
/// - O menor número.
fn analisar_tupla(tupla: (i32, i32, i32)) -> (i32, i32, i32) {
    let soma = tupla.0 + tupla.1 + tupla.2;             // Calcula a soma dos elementos da tupla
    let maior = tupla.0.max(tupla.1).max(tupla.2);      // Determina o maior número entre os três
    let menor = tupla.0.min(tupla.1).min(tupla.2);      // Determina o menor número entre os três
    (soma, maior, menor)
}

/// Função principal para executar a lógica da QUESTÃO 2
fn questao_02_main() {
    println!("Digite o primeiro número inteiro:");
    let num1 = ler_inteiro();                           // Lê o primeiro número do usuário

    println!("Digite o segundo número inteiro:");
    let num2 = ler_inteiro();                           // Lê o segundo número do usuário

    println!("Digite o terceiro número inteiro:");
    let num3 = ler_inteiro();                           // Lê o terceiro número do usuário

    let tupla = (num1, num2, num3);                     // Cria uma tupla com os três números
    let resultado = analisar_tupla(tupla);              // Analisa a tupla e obtém o resultado

    // Exibe a soma, o maior e o menor número
    println!("Soma: {}", resultado.0);
    println!("Maior: {}", resultado.1);
    println!("Menor: {}", resultado.2);
}

/// Função `main` que permite ao usuário escolher qual questão executar.
fn main() {
    println!("Escolha a questão para executar (1 ou 2):");

    // Lê a escolha do usuário
    let mut escolha = String::new();
    io::stdin().read_line(&mut escolha).expect("Erro ao ler a entrada");

    // Executa a questão escolhida pelo usuário
    match escolha.trim() {
        "1" => questao_01_main(),                     
        "2" => questao_02_main(),                      
        _ => println!("Escolha inválida!"),            
    }
}
