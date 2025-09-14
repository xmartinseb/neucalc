use crate::calc_base::value::Value;
use crate::calc_strategies::recursive_scan_strategy::RecursiveScanStrategy;
use crate::calculator::Calculator;
use std::io;
use std::panic::catch_unwind;

mod base;
mod calc_base;
mod calc_strategies;
mod calculator;

fn main() {
    print_header();
    let calculator = Calculator::<RecursiveScanStrategy>::default();
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
                    }
                    Err(err) => println!("Chyba: {}", err),
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
                println!("Stiskněte enter pro ukončení");
                _ = stdin.read_line(&mut String::new());
                return; // Konec z důvodu závažné chyby v programu
            }
        }
    }
}

/// Vytiskne logo programu Neucalc a přidá autorský podpis a číslo verze.
fn print_header() {
    let app_info = format!(
        "Neucalc - verze {}, Martin Sebera 2025",
        env!("CARGO_PKG_VERSION")
    );
    sprintln!(app_info);
    sprintln!("━".repeat(app_info.chars().count())); // Posledni radek bude podtrzeny
    println!();
}
