use std::time::Instant;

fn contador(vetor: &[String], alvo: &str) -> usize {
    let mut total = 0;
    for i in vetor{
        if i == alvo{
            total += 1;
        }
    }
    total
}
fn busca_sequencial_simples(vetor: &[String], alvo: &str) -> (Option<usize>, usize){

    let mut operacoes = 0;
    let mut resultado = None;

    for i in 0..vetor.len() {
        operacoes +=1;
        if vetor[i] == alvo{
            resultado = Some(i);
        }
    }
    (resultado, operacoes)
}

fn busca_sequencial_interrompida(vetor: &[String], alvo: &str) -> (Option <usize>, usize){
    let mut operacoes = 0;
    for i in 0..vetor.len(){
        operacoes+= 1;
        if vetor [i] == alvo{
            return (Some(i), operacoes);
        }
    }
    (None, operacoes)
}

fn gerar_vetor(tamanho: usize) -> Vec<String> {
    (1..=tamanho).map(|i| i.to_string()).collect()
}

fn executar_experimento(tamanho: usize, alvo:&str){
    let vetor = gerar_vetor(tamanho);
    let vezes = contador(&vetor, alvo);

    println!("\n{}", "=".repeat(60));
    println!("Tamanho do vetor: {}", tamanho);
    println!("Elemento procurado: {}", alvo);
    println!("{}", "-".repeat(60));

    let inicio = Instant:: now();
    let (resultado1, ops1) = busca_sequencial_simples(&vetor, alvo);
    let tempo1 = inicio.elapsed();

    println!("\n BUSCA SEQUENCIAL SIMPLES:");
    println!(" Resultado: {:?}", resultado1);
    println!(" Operações: {}", ops1);
    println!(" Tempo: {:?}", tempo1);

    let inicio = Instant:: now();
    let (resultado2, ops2) = busca_sequencial_interrompida(&vetor, alvo);
    let tempo2 = inicio.elapsed();

    println!("\n BUSCA SEQUENCIAL INTERROMPIDA:");
    println!(" Resultado: {:?}", resultado2);
    println!(" Operações: {}", ops2);
    println!(" Tempo: {:?}", tempo2);

    println!("\n ANÁLISE COMPARATIVA:");
    println!(" Diferença de operações: {} operações", ops1.saturating_sub(ops2));
    if tempo1 > tempo2 {
        println!(" Algoritmo com interrupção foi mais rápido");
    } else if tempo1 < tempo2 {
        println!(" Algoritmo simples foi mais rápido (provavelmente devido a variação)");
    } else {
        print!(" Tempos praticamente iguais");
    }
    println!("{}", "=".repeat(60));

}
fn main() {
    println!("\n EXPERIMENTO: comparação de algoritmos de busca\n");

    //cenário 1: elemento no inicio do vetor 
    println!("\n cenário 1: elemento no inínio (melhor caso para interrupção)");
    let vezes = contador(&gerar_vetor(1_000_000), "5");
    executar_experimento(1_000, "5");
    executar_experimento(100_000, "5");
    executar_experimento(1_000_000, "5");
    println!("Elemento apareceu {} vezes", vezes);

    //cenário 2: Elemento no meio do vetor
    println!("\n cenário 2: elemento no meio ");
    let vezes = contador(&gerar_vetor(1_000_000), "500000");
    executar_experimento(1_000, "500");
    executar_experimento(100_000, "50000");
    executar_experimento(1_000_000, "500000");
    println!("Elemento apareceu {} vezes", vezes);

    //cenário 3: Elemento no fim
    println!("\n\n  cenário 3: Elemento no final (pior)");
    let vezes = contador(&gerar_vetor(1_000_000), "1000000");
    executar_experimento(1_000, "1000");
    executar_experimento(100_000, "100000");
    executar_experimento(1_000_000, "1000000");
    println!("Elemento apareceu {} vezes", vezes);
    
    //cenário 4: Elemento não existe
    println!("\n\n cenário 4: Elemento não existe no vetor");
    let vezes = contador(&gerar_vetor(1_000_000), "nonexistent");
    executar_experimento(1_000, "nonexistent");
    executar_experimento(100_000, "nonexistent");
    executar_experimento(1_000_000, "nonexistent");
    println!("Elemento apareceu {} vezes", vezes);

    println!("\n\n Experimento concluido!\n");
}
