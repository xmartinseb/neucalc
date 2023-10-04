use std::io;
use std::panic::catch_unwind;
use crate::calc_base::value::Value;
use crate::calc_strategies::recursive_scan_strategy::RecursiveScanStrategy;
use crate::calculator::Calculator;

mod base;
mod calc_strategies;
mod calc_base;
mod calculator;

fn main() {
    println!(" _   _                      _");
    println!("| \\ | |                    | |");
    println!("|  \\| | ___ _   _  ___ __ _| | ___");
    println!("| . ` |/ _ \\ | | |/ __/ _` | |/ __|");
    println!("| |\\  |  __/ |_| | (_| (_| | | (__");
    println!("\\_| \\_/\\___|\\__,_|\\___\\__,_|_|\\___|  Verze 0.1.5, Martin Sebera 2023");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let calculator = Calculator::<RecursiveScanStrategy>::new();
    let stdin = io::stdin();

    loop {
        let operation_result = catch_unwind(|| {
            let mut input = String::default();
            sprint!(">> ");

            if let Err(e) = stdin.read_line(&mut input) {
                println!("Chyba při čtení vstupu z konzole: {:?}", e);
                return false;
            }
            if input.trim().is_empty() {
                return false;
            } else {
                let calc_result = calculator.evaluate_expr(&input);
                match calc_result {
                    Ok(result) => {
                        // Zlomek se pro přehlednost vypíše i jako zlomek, i jako reálné číslo
                        if let Value::Rational(ratio) = result.clone() {
                            sprintln!(result);
                            if let Some(real) = ratio.to_real() {
                                let as_real = Value::Real(real);
                                sprintln!(as_real);
                            }
                        } else {
                            sprintln!(result);
                        }
                    },
                    Err(err) => println!("Chyba: {}", err.get_msg()),
                }
            }
            println!();
            return true;
        });

        match operation_result {
            Ok(val) => {
                if val == false {
                    return; // Konec z důvodu prázdného vstupu
                }
            }
            Err(err) => {
                println!("V programu došlo k chybě: {:?}", err);
                return; // Konec z důvodu závažné chyby v programu
            }
        }
    }
}