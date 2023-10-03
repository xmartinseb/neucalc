use std::io;
use std::panic::catch_unwind;
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
                    Ok(result) => sprintln!(result),
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