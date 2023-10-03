use std::cell::Cell;
use std::io;
use std::io::Write;
use std::panic::catch_unwind;
use crate::calc_strategies::recursive_scan_strategy::RecursiveScanStrategy;
use crate::calculator::Calculator;

mod base;
mod calc_strategies;
mod calc_base;
mod calculator;

fn main() {
    loop {
        let operation_result = catch_unwind(|| {
            let mut stdin = io::stdin();
            let mut input = String::default();
            sprint!(">> ");
            let reader_result = stdin.read_line(&mut input);
            if let Err(e) = reader_result {
                println!("Chyba čtení řádky: {}", e);
                return; //break
            } else if input.trim().is_empty(){
                return;
            } else {
                let calculator = Calculator::<RecursiveScanStrategy>::new();
                let calc_result = calculator.evaluate_expr(&input);
                //println!("{:?}", math_tree_result);
                match calc_result {
                    Ok(result) => sprintln!(result),
                    Err(err) => println!("Chyba: {}", err.get_msg()),
                }
            }
        });

        if let Err(err) = operation_result {
            println!("V programu došlo k chybě: {:?}", err);
        }
    }
}