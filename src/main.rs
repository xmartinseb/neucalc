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
                        if let Value::Rational(ratio) = result.clone() {
                            if let Some(ratio_as_int) = ratio.to_bigint(){
                                // Zlomek je vlastně celé číslo
                                let as_int = Value::BigInt(ratio_as_int);
                                sprintln!(as_int);
                            } else {
                                // Zlomek se pro přehlednost vypíše i jako zlomek, i jako reálné číslo
                                sprintln!(result);
                                if let Some(real) = ratio.to_real() {
                                    let as_real = Value::Real(real);
                                    sprintln!(as_real);
                                }
                            }
                        } else {
                            sprintln!(result);
                        }

                        // sprintln!(result);
                        // // Zlomek se pro přehlednost vypíše i jako zlomek, i jako reálné číslo
                        // if let Value::Rational(ratio) = result {
                        //     if let Some(real) = ratio.to_real() {
                        //         let as_real = Value::Real(real);
                        //         sprintln!(as_real);
                        //     }
                        // }
                    },
                    Err(err) => println!("Chyba: {}", err.get_msg()),
                }
            }
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