use std::ops::Neg;
use regex::Regex;
use crate::base::IAppError;
use crate::calc_base::{MathEvaluateError, MathParseError};
use crate::calc_base::func_call::FuncCall;
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
        self.evaluate_rec_simplify(self.math_expr)
    }

    fn parse_func_call(&self, expr: &str) -> Result<FuncCall, Box<dyn IAppError>>{
        let func_call_regex = Regex::new(r"^(?<fname>[a-zA-Z]+) *\((?<params>.*)\)$").unwrap();
        let captures = ok_or_error(func_call_regex.captures(expr))?;

        let func_name = ok_or_error(captures.name("fname"))?.as_str();
        let params_as_str = ok_or_error(captures.name("params"))?.as_str().trim();
        let params_vec = if params_as_str.is_empty() {
            Vec::<_>::new()
        } else {
            self.parse_params_str(params_as_str)?
        };

        return Ok(FuncCall::new(func_name, params_vec));

        // Pomocná funkce na vyhazování chyb
        fn ok_or_error<T>(opts: Option<T>) -> Result<T, Box<dyn IAppError>>{
            opts.ok_or(Box::new(MathEvaluateError::new(s!("Výraz nemá tvar volání funkce: název(parametry)"))))
        }
    }
}

impl<'expr> RecursiveScanStrategy<'expr> {
    /// Parametry, které jsou zadány oddělené čárkami, se převedou na vektor parametrů.
    fn parse_params_str(&self, params_str: &str) -> Result<Vec<Value>, Box<dyn IAppError>> {
        let mut delims = vec![]; // Pozice, podle nichz se string roztrha na jednotlive parametry
        let mut curr_depth = 0; // Carky, ktere oddeluji parametry musi samozrejme byt mimo zavorky
        let mut is_in_string = false; // Ty carky nesmi byt ani ve stringu
        let mut bytes_scanned = 0; // Kazdy znak muze mit jiny pocet bajtu (UTF-8). Proto je dulezita pozice v bajtech, ne znacich!
        for c in params_str.chars() {
            match c {
                '(' => curr_depth += 1,
                ')' => curr_depth -= 1,
                '"' => is_in_string = !is_in_string,
                ',' => if curr_depth == 0 && !is_in_string {
                    delims.push(bytes_scanned);
                }
                _ => {}
            };
            bytes_scanned += c.len_utf8();
        }
        let mut params = Vec::<Value>::new();
        if delims.len() == 0 { // Parametr je jen jeden, neni potreba nic trhat. Cely string je jeden parametr
            params.push(parse_param(self, params_str)?);
        } else { // Pamateru je vic, string se musi rozthrat
            let mut last_substring_begin = 0usize;
            for substring_end in delims {
                params.push(parse_param(self, &params_str[last_substring_begin..substring_end])?);
                last_substring_begin = substring_end + 1;
            }
            params.push(parse_param(self, &params_str[last_substring_begin..])?);
        }

        return Ok(params);

        // Pomocná funkce, která parsuje jeden parametr. Vrátí ho jako value, nebo vrátí chybu.
        fn parse_param(this: &RecursiveScanStrategy, paramstr: &str) -> Result<Value, Box<dyn IAppError>> {
            return match this.evaluate_rec_simplify(paramstr) {
                Ok(param) => { Ok(param) }
                Err(e) => {
                    let err: Box<dyn IAppError> = Box::new(e);
                    Err(err)
                }
            }
        }
    }

    #[inline]
    fn evaluate_rec_simplify(&self, expr: &str) -> Result<Value, MathEvaluateError> {
        self.evaluate_rec(expr)?.simplify_type_move()
    }

    fn evaluate_rec(&self, expr: &str) -> Result<Value, MathEvaluateError> {
        let expr = trim_brackets(expr.trim());
        match Self::find_oper(expr) {
            None => {
                return Value::parse(expr, self)?.simplify_type_move();
            }
            Some((oper_symbol, oper_pos)) => {
                let (left, right) = Self::halve_expr(expr, oper_pos);
                if left.is_empty() && right.is_empty() {
                    return Err(MathEvaluateError::new(format!("Operátor {oper_symbol} na pozici {oper_pos} nemá žádné operandy")));
                }
                match oper_symbol {
                    '+' => {
                        return if left.is_empty() {
                            self.evaluate_rec_simplify(right)
                        } else if right.is_empty() {
                            Err(MathEvaluateError::new(s!("Operátoru + chybí pravý operand")))
                        } else {
                            self.evaluate_rec_simplify(left)? + self.evaluate_rec_simplify(right)?
                        }
                    },
                    '-' => {
                        return if left.is_empty() {
                            let r = self.evaluate_rec_simplify(right)?;
                            r.neg()
                        } else if right.is_empty() {
                            Err(MathEvaluateError::new(s!("Operátoru - chybí pravý operand")))
                        } else {
                            self.evaluate_rec_simplify(left)? - self.evaluate_rec_simplify(right)?
                        }
                    },
                    '*' => {
                        return if left.is_empty() || right.is_empty() {
                            Err(MathEvaluateError::new(String::from("Operátor * vyžaduje dva operandy")))
                        } else {
                            self.evaluate_rec_simplify(left)? * self.evaluate_rec_simplify(right)?
                        }
                    },
                    '/' => {
                        return if left.is_empty() || right.is_empty() {
                            Err(MathEvaluateError::new(String::from("Operátor / vyžaduje dva operandy")))
                        } else {
                            self.evaluate_rec_simplify(left)? / self.evaluate_rec_simplify(right)?
                        }
                    },
                    '^' => {
                        return if left.is_empty() || right.is_empty() {
                            Err(MathEvaluateError::new(String::from("Operátor ^ vyžaduje dva operandy")))
                        } else {
                            self.evaluate_rec_simplify(left)?.pow(&self.evaluate_rec_simplify(right)?)
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
                    if oper_priority > 0 // Když == 0, není to znak operátoru!
                        && curr_depth == 0 // Operátor dělení výrazu nesmí být v závorkách!
                        && oper_priority < best_oper_priority
                    {
                        best_oper_priority = oper_priority;
                        best_oper_pos = exprlen - bytes_scanned - 1;
                        best_operator_symbol = c;
                    }
                }
            }
            // Každý znak má jiný počet bajtů (UTF-8). Pozice dělícího operátoru se určuje indexem bajtu!
            bytes_scanned += c.len_utf8();
        }

        return if best_oper_pos == usize::MAX
        { None } else { Some((best_operator_symbol, best_oper_pos)) };
    }
}