/* 	Projeto do Cruzamento Automático - versão s1

    Simula carros em um cruzamento simples
*/

use std::thread::sleep;
use std::time::Duration;

/* Geometria do cruzamento

                        largura V
                margem V|    |
                        |    |
                        |    |margem H
------------------------+----+--------
    ViaH  > > > 	   	|    |	    largura H
------------------------+----+--------
    perímetro H			|    |
                        |    |
                        |    |
                        | ^  |perímetro V
                        | ^  |
                        | ^  |
                        |ViaV|
                        |    |


*/

const _VIA_H_MARGEM: f64 = 15.0; //metros
const _VIA_V_MARGEM: f64 = 15.0; //metros

const VIA_H_LARGURA: f64 = 4.0; //metros
const VIA_V_LARGURA: f64 = 4.0; //metros

const _VIA_H_PERIMETRO: f64 = 150.00; //metros
const _VIA_V_PERIMETRO: f64 = 150.00; //metros

const _CARRO_LARGURA: f64 = 2.0; //metros
const CARRO_COMPRIMENTO: f64 = 4.0; //metros

// velocidade máxima do carro em metros por segundo
const VELOCIDADE_MAXIMA: f64 = 200.0 * (1000.0 / 3600.0);

//Aceleração minima de qualquer veiculo em metros por segundo ao quadrado
const ACELERACAO_MAXIMA: f64 = 3.0;

//Aceleração minima de qualquer veiculo em metros por segundo ao quadrado
const ACELERACAO_MINIMA: f64 = -10.0;

// Simula 2 carros até saírem do perímetro controlado ou colidirem
// retorna se houve colisão ou não
fn simular_carros(
    via_carro_1: char,
    acel_carro_1: f64,
    via_carro_2: char,
    acel_carro_2: f64,
) -> bool {
    // descrição do carro 1
    let mut placa_1: String = String::from("ABC1111"); // identificação de carro
    let via_1: char = via_carro_1; // via deste carro
    let _acel_max_1 = ACELERACAO_MAXIMA; // metros por segundo ao quadrado
    let _acel_min_1 = ACELERACAO_MINIMA; // metros por segundo ao quadrado
    let vel_max_1 = VELOCIDADE_MAXIMA; // metros por segundo
    let comprimento_1 = CARRO_COMPRIMENTO; // metros
    let mut pos_atual_1: f64 = -80.0; // metros do cruzamento
    let mut vel_atual_1: f64 = 0.0; // metros por segundo
    let acel_atual_1: f64; // metros por segundo ao quadrado

    // descrição do carro 2
    let mut placa_2: String = String::from("xyz2222"); // identificação de carro
    let via_2: char = via_carro_2; // via deste carro
    let _acel_max_2 = ACELERACAO_MAXIMA; // metros por segundo ao quadrado
    let _acel_min_2 = ACELERACAO_MINIMA; // metros por segundo ao quadrado
    let vel_max_2 = VELOCIDADE_MAXIMA; // metros por segundo
    let comprimento_2 = CARRO_COMPRIMENTO; // metros
    let mut pos_atual_2: f64 = -100.0; // metros do cruzamento
    let mut vel_atual_2: f64 = 0.0; // metros por segundo
    let acel_atual_2: f64; // metros por segundo ao quadrado

    //verifica a validade das placas
    placa_1 = placa_1.to_uppercase();
    placa_2 = placa_2.to_uppercase();

    if !valida_placa(&placa_1) {
        panic!("Placa inválida: {}", placa_1);
    }

    if !valida_placa(&placa_2) {
        panic!("Placa inválida: {}", placa_2);
    }

    acel_atual_1 = acel_carro_1;
    acel_atual_2 = acel_carro_2;
    println!("inicio da simulação");
    let mut tickms: f64; // tempo que passou em cada tick, milissegundos

    loop {
        // Ao final do ticker devemos atualizar o estado do carro
        sleep(Duration::from_millis(100)); // simulação a cada 100ms
        tickms = 100.0;

        // Atualiza o carro 1
        let old_position = pos_atual_1;

        pos_atual_1 +=
            vel_atual_1 * (tickms / 1000.0) * acel_atual_1 * (tickms / 1000.0) * (tickms / 1000.0)
                / 2.0;
        vel_atual_1 += acel_atual_1 * (tickms / 1000.0);

        // restrições carro 1
        if pos_atual_1 < old_position {
            vel_atual_1 = old_position;
        }

        if vel_atual_1 < 0.0 {
            vel_atual_1 = 0.0;
        }

        if vel_atual_1 > vel_max_1 {
            vel_atual_1 = vel_max_1;
        }

        println!(
            "Carro 1 {} na posição {}{}, velocidade {}, aceleração {}",
            placa_1, via_1, pos_atual_1, vel_atual_1, acel_atual_1
        );

        // Atualiza o carro 2
        let old_position = pos_atual_2;

        pos_atual_2 +=
            vel_atual_2 * (tickms / 1000.0) * acel_atual_2 * (tickms / 1000.0) * (tickms / 1000.0)
                / 2.0;
        vel_atual_2 += acel_atual_2 * (tickms / 1000.0);

        // restrições carro 2
        if pos_atual_2 < old_position {
            vel_atual_2 = old_position;
        }

        if vel_atual_2 < 0.0 {
            vel_atual_2 = 0.0;
        }

        if vel_atual_2 > vel_max_2 {
            vel_atual_2 = vel_max_2;
        }

        println!(
            "Carro 2 {} na posição {}{}, velocidade {}, aceleração {}",
            placa_2, via_2, pos_atual_2, vel_atual_2, acel_atual_2
        );

        // Detecta Colisão na via H
        if via_1 == 'H' && via_2 == 'H' {
            if colissao_longitudinal(pos_atual_1, comprimento_1, pos_atual_2) {
                println!("Colisão na via H!");
                return true;
            }
        }

        // Detecta Colisão na via V
        if via_1 == 'V' && via_2 == 'V' {
            if colissao_longitudinal(pos_atual_1, comprimento_1, pos_atual_2) {
                println!("Colisão na via V!");
                return true;
            }
        }

        // Detecta colisão no cruzamento
        if via_1 != via_2 {
            if dentro_cruzamento(pos_atual_1, comprimento_1, via_1)
                && dentro_cruzamento(pos_atual_2, comprimento_2, via_2)
            {
                println!("Colisão no cruzamento!");
                return true;
            }
        }

        // Verifica se o carro 1 saiu do saiu do perímetro controlado (falta a margem)
        if pos_atual_1
            > comprimento_1
                + if via_1 == 'H' {
                    VIA_V_LARGURA
                } else {
                    VIA_H_LARGURA
                }
        {
            break;
        }

        // Verifica se o carro 2 saiu do saiu do perímetro controlado (falta a margem)
        if pos_atual_2
            > comprimento_2
                + if via_2 == 'H' {
                    VIA_V_LARGURA
                } else {
                    VIA_H_LARGURA
                }
        {
            break;
        }
    }
    false
}

// Colisão de dois carros ao longo da mesma via
fn colissao_longitudinal(posicao_frente: f64, comprimento: f64, posicao_atras: f64) -> bool {
    posicao_frente - comprimento <= posicao_atras
}

// Detecta carro dentro do cruzamento
fn dentro_cruzamento(posicao: f64, comprimento: f64, via: char) -> bool {
    posicao > 0.0
        && posicao
            <= comprimento
                + if via == 'H' {
                    VIA_V_LARGURA
                } else {
                    VIA_H_LARGURA
                }
}

fn valida_placa(placa: &str) -> bool {
    // só aceita caracteres ASCII
    if !placa.is_ascii() {
        println!("Placa não é ASCII");
        return false;
    }

    // só aceita placas velhas
    if placa.len() != 7 {
        println!("Placa não tem letras tamanho certo");
        return false;
    }

    let inicio = &placa[0..3];
    let fim = &placa[3..];

    for x in inicio.chars() {
        if !x.is_alphabetic() {
            println!("Placa não começa com letras");
            return false;
        }
    }

    for x in fim.chars() {
        if !x.is_ascii_digit() {
            println!("Placa não termina com números");
            return false;
        }
    }

    true
}

fn main() {
    println!("Inicio do programa");
    simular_carros('H', ACELERACAO_MAXIMA / 10.0, 'H', ACELERACAO_MAXIMA);
    println!("Fim da simulação");
}
