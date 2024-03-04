use crate::base::CalcError;
use crate::calc_base::expr::Expr;
use crate::s;

/// Pokud je symbol operátor, vrací jeho prioritu, jinak vrací 0
pub fn is_oper(symbol: char) -> i32 {
    match symbol {
        '+' => 1,
        '-' => 1,
        '*' => 2,
        '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

pub fn trim_brackets(expr: Expr) -> Expr {
    let mut subexpr = expr.as_str();
    while subexpr.len() > 2 && subexpr.starts_with('(') && subexpr.ends_with(')') {
        let subexpr_wout_brackets = &subexpr[1..subexpr.len() - 1];
        if check_brackets_and_quots_simple(subexpr_wout_brackets) {
            subexpr = subexpr_wout_brackets;
        } else {
            break;
        }
    }
    Expr::new(subexpr)
}

/// Ještě před zahájením výpočtu je potřeba zkontrolovat správnost výrazu. Tj. správnost postavení
/// závorek a stringů (každý string musí mít ukončovací uvozovku)
pub fn check_brackets_and_quots(expr: &str) -> Result<(), CalcError> {
    let mut is_in_string = false;
    let mut le = 0;
    let mut ri = 0;
    for (i, c) in expr.chars().enumerate() {
        if c == '"' {
            is_in_string = !is_in_string;
        } else if !is_in_string {
            if c == '(' {
                le += 1;
            } else if c == ')' {
                ri += 1;
                if ri > le {
                    return Err(CalcError::ParseErr(format!(
                        "Na pozici {i} počet pravých závorek předběhl počet levých závorek"
                    )));
                }
            }
        }
    }

    return if is_in_string {
        Err(CalcError::ParseErr(s!(
            "Textovému výrazu chybí ukončovací uvozovka."
        )))
    } else if le == ri {
        Ok(())
    } else {
        Err(CalcError::ParseErr(s!(
            "Počet levých a pravých závorek musí být stejný"
        )))
    };
}

pub fn check_brackets_and_quots_simple(expr: &str) -> bool {
    let mut is_in_string = false;
    let mut le = 0;
    let mut ri = 0;
    for c in expr.chars() {
        if c == '"' {
            is_in_string = !is_in_string;
        } else if !is_in_string {
            if c == '(' {
                le += 1;
            } else if c == ')' {
                ri += 1;
                if ri > le {
                    return false;
                }
            }
        }
    }

    return if is_in_string {
        false
    } else if le == ri {
        true
    } else {
        false
    };
}
