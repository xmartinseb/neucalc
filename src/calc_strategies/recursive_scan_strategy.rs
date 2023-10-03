use std::ops::Neg;
use crate::calc_base::{MathEvaluateError, MathParseError};
use crate::calc_base::value::Value;
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
        check_brackets(math_expr)?;
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

fn find_oper(expr: &str) -> Option<(char, usize)> {
    let exprlen = expr.len();
    let mut curr_depth = 0;
    let mut best_oper_depth = i32::MAX;
    let mut best_oper_priority = i32::MAX;
    let mut best_oper_pos = usize::MAX;
    let mut best_operator_symbol = '\0';

    for (idx, c) in expr.chars().rev().enumerate() {
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
                best_oper_pos = exprlen - idx - 1;
                best_operator_symbol = c;
            }
        }
    }

    return if best_oper_pos == usize::MAX
    { None } else { Some((best_operator_symbol, best_oper_pos)) };
}

/// Pokud je symbol operátor, vrací jeho prioritu, jinak vrací 0
fn is_oper(symbol: char) -> i32 {
    match symbol {
        '+' => 1,
        '-' => 1,
        '*' => 2,
        '/' => 2,
        '^' => 3,
        _ => 0
    }
}

fn trim_brackets(expr: &str) -> &str {
    let mut subexpr = expr;
    while subexpr.len() > 2 && subexpr.starts_with('(') && subexpr.ends_with(')') {
        let subexpr_wout_brackets = &subexpr[1..subexpr.len() - 1];
        if check_brackets_simple(subexpr_wout_brackets) {
            subexpr = subexpr_wout_brackets;
        } else {
            break;
        }
    }
    subexpr.trim()
}

fn check_brackets_simple(expr: &str) -> bool {
    let mut le = 0;
    let mut ri = 0;
    for (i, c) in expr.chars().enumerate() {
        if c == '(' {
            le += 1;
        } else if c == ')' {
            ri += 1;
            if ri > le {
                return false;
            }
        }
    }

    if le == ri {
        true
    } else {
        false
    }
}

fn check_brackets(expr: &str) -> Result<(), MathParseError> {
    let mut le = 0;
    let mut ri = 0;
    for (i, c) in expr.chars().enumerate() {
        if c == '(' {
            le += 1;
        } else if c == ')' {
            ri += 1;
            if ri > le {
                return Err(MathParseError::new(format!("Na pozici {i} počet pravých závorek předběhl počet levých závorek")));
            }
        }
    }

    if le == ri {
        Ok(())
    } else {
        Err(MathParseError::new(s!("Počet levých a pravých závorek musí být stejný")))
    }
}