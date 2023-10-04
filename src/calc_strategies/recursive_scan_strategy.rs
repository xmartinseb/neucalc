use std::ops::Neg;
use crate::calc_base::{MathEvaluateError, MathParseError};
use crate::calc_base::value::Value;
use crate::calc_strategies::common::*;
use crate::calc_strategies::ICalculatorStrategy;
use crate::s;

/// Strategie, která jen čte výraz jako text a rekurzivně spočítá výsledek. Neprovádí žádné
/// transformace na stromovou strukturu aj. Výhodou je jednoduchost, ale pro velmi dlouhé výrazy je pomalá.
#[derive(Default, Debug)]
pub struct RecursiveScanStrategy<'expr> {
    math_expr: &'expr str
}

impl<'expr> ICalculatorStrategy<'expr> for RecursiveScanStrategy<'expr> {
    ///Tato strategie nepoužívá žádnou speciální strukturu, jen rekurzivně skenuje text
    /// Tato metoda jen částečně zvaliduje správnost výrazu
    fn parse(&mut self, math_expr: &'expr str) -> Result<(), MathParseError> {
        self.math_expr = math_expr;
        Ok(())
    }

    fn evaluate(&mut self) -> Result<Value, MathEvaluateError> {
        evaluate_rec(self.math_expr)
    }
}

fn evaluate_rec(expr: &str) -> Result<Value, MathEvaluateError> {
    let expr = trim_brackets(expr.trim());
    match find_oper(expr) {
        None => {
            return Value::parse(expr);
        }
        Some((oper_symbol, oper_pos)) => {
            let (left, right) = halve_expr(expr, oper_pos);
            if left.is_empty() && right.is_empty() {
                return Err(MathEvaluateError::new(format!("Operátor {oper_symbol} na pozici {oper_pos} nemá žádné operandy")));
            }
            match oper_symbol {
                '+' => {
                    return if left.is_empty() {
                        evaluate_rec(right)
                    } else if right.is_empty(){
                        Err(MathEvaluateError::new(s!("Operátoru + chybí pravý operand")))
                    }  else {
                        evaluate_rec(left)? + evaluate_rec(right)?
                    }
                },
                '-' => {
                    return if left.is_empty() {
                        let r = evaluate_rec(right)?;
                        r.neg()
                    } else if right.is_empty(){
                        Err(MathEvaluateError::new(s!("Operátoru - chybí pravý operand")))
                    }  else {
                        evaluate_rec(left)? - evaluate_rec(right)?
                    }
                },
                '*' => {
                    return if left.is_empty() || right.is_empty(){
                        Err(MathEvaluateError::new(String::from("Operátor * vyžaduje dva operandy")))
                    } else {
                        evaluate_rec(left)? * evaluate_rec(right)?
                    }
                }, '/' => {
                    return if left.is_empty() || right.is_empty() {
                        Err(MathEvaluateError::new(String::from("Operátor / vyžaduje dva operandy")))
                    } else {
                        evaluate_rec(left)? / evaluate_rec(right)?
                    }
                },
                '^' => {
                    return if left.is_empty() || right.is_empty(){
                        Err(MathEvaluateError::new(String::from("Operátor ^ vyžaduje dva operandy")))
                    } else {
                        evaluate_rec(left)?.pow(&evaluate_rec(right)?)
                    }
                },
                _ => Err(MathEvaluateError::new(format!("Znak '{oper_symbol}' není definovaný operátor")))
            }
        }
    }
}

fn halve_expr(expr: &str, oper_pos: usize) -> (&str, &str) {
    if oper_pos == 0 {
        ("", &expr[1..].trim())
    } else {
        (&expr[..oper_pos].trim(), &expr[oper_pos + 1..].trim())
    }
}

/// Vrací nalezený operátor a jeho pozici v textu.
/// POZOR! Nejedná se o pozici ve smyslu index znaku, ale index bajtu!
/// Znak operátoru má mít jeden bajt, ale jiné znaky UTF-8 mohou mít víc bajtů.
fn find_oper(expr: &str) -> Option<(char, usize)> {
    let exprlen = expr.len();
    let mut is_in_string = false;
    let mut curr_depth = 0;
    let mut best_oper_depth = i32::MAX;
    let mut best_oper_priority = i32::MAX;
    let mut best_oper_pos = usize::MAX;
    let mut best_operator_symbol = '\0';

    let mut bytes_scanned: usize = 0;
    for c in expr.chars().rev() {
        if c == '"' {
            is_in_string = !is_in_string;
        } else if !is_in_string {
            if c == ')' {
                curr_depth += 1;
            } else if c == '(' {
                curr_depth -= 1;
            } else {
                let oper_priority = is_oper(c);
                if oper_priority > 0
                    && (curr_depth < best_oper_depth
                    || (curr_depth == best_oper_depth && oper_priority < best_oper_priority))
                {
                    best_oper_priority = oper_priority;
                    best_oper_depth = curr_depth;
                    best_oper_pos = exprlen - bytes_scanned - 1;
                    best_operator_symbol = c;
                }
            }
        }
        bytes_scanned += c.len_utf8();
    }

    return if best_oper_pos == usize::MAX
    { None } else { Some((best_operator_symbol, best_oper_pos)) };
}