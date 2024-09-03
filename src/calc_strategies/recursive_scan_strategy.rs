use crate::base::CalcError;
use crate::calc_base::expr::Expr;
use crate::calc_base::func_call::FuncCall;
use crate::calc_base::value::Value;
use crate::calc_strategies::common::*;
use crate::calc_strategies::ICalculatorStrategy;
use crate::s;
use regex::Regex;
use std::ops::Neg;

/// Strategie, která jen čte výraz jako text a rekurzivně spočítá výsledek. Neprovádí žádné
/// transformace na stromovou strukturu aj. Výhodou je jednoduchost, ale pro velmi dlouhé
/// výrazy je pomalá a navíc je kvůli mnoha rekurzím náchylná na chybu StackOverflow.
#[derive(Default, Debug)]
pub struct RecursiveScanStrategy<'expr> {
    math_expr: Expr<'expr>,
}

impl<'expr> ICalculatorStrategy<'expr> for RecursiveScanStrategy<'expr> {
    ///Tato strategie nepoužívá žádnou speciální strukturu, jen rekurzivně skenuje text
    fn parse(&mut self, math_expr: Expr<'expr>) -> Result<(), CalcError> {
        self.math_expr = math_expr;
        Ok(())
    }

    fn evaluate(&mut self) -> Result<Value, CalcError> {
        self.evaluate_rec_simplify(self.math_expr.clone())
    }

    fn parse_func_call(&self, expr: Expr) -> Result<FuncCall, CalcError> {
        let func_call_regex = Regex::new(r"^(?<fname>[a-zA-Z]+) *\((?<params>.*)\)$").unwrap();
        let captures = ok_or_error(func_call_regex.captures(expr.as_str()))?;

        let func_name = ok_or_error(captures.name("fname"))?.as_str();
        let params_as_str = ok_or_error(captures.name("params"))?.as_str().trim();
        let params_vec = if params_as_str.is_empty() {
            Vec::<_>::new()
        } else {
            self.parse_params_str(Expr::new(params_as_str))?
        };

        return Ok(FuncCall::new(func_name, params_vec));

        // Pomocná funkce na vyhazování chyb
        fn ok_or_error<T>(opts: Option<T>) -> Result<T, CalcError> {
            opts.ok_or(CalcError::EvaluateErr(s!(
                "Výraz nemá tvar volání funkce: název(parametry)"
            )))
        }
    }
}

impl<'expr> RecursiveScanStrategy<'expr> {
    /// Parametry, které jsou zadány oddělené čárkami, se převedou na vektor parametrů.
    fn parse_params_str(&self, params_str: Expr) -> Result<Vec<Value>, CalcError> {
        let mut delims = vec![]; // Pozice, podle nichz se string roztrha na jednotlive parametry
        let mut curr_depth = 0; // Carky, ktere oddeluji parametry musi samozrejme byt mimo zavorky
        let mut is_in_string = false; // Ty carky nesmi byt ani ve stringu
        let mut bytes_scanned = 0; // Kazdy znak muze mit jiny pocet bajtu (UTF-8). Proto je dulezita pozice v bajtech, ne znacich!
        for c in params_str.chars() {
            match c {
                '(' => curr_depth += 1,
                ')' => curr_depth -= 1,
                '"' => is_in_string = !is_in_string,
                ',' => {
                    if curr_depth == 0 && !is_in_string {
                        delims.push(bytes_scanned);
                    }
                }
                _ => {}
            };
            bytes_scanned += c.len_utf8();
        }
        let mut params = Vec::<Value>::new();
        if delims.len() == 0 {
            // Parametr je jen jeden, neni potreba nic trhat. Cely string je jeden parametr
            params.push(parse_param(self, params_str.clone())?);
        } else {
            // Pamateru je vic, string se musi rozthrat
            let mut last_substring_begin = 0usize;
            for substring_end in delims {
                params.push(parse_param(
                    self,
                    Expr::new(&params_str.as_str()[last_substring_begin..substring_end]),
                )?);
                last_substring_begin = substring_end + 1;
            }
            params.push(parse_param(
                self,
                Expr::new(&params_str.as_str()[last_substring_begin..]),
            )?);
        }

        return Ok(params);

        // Pomocná funkce, která parsuje jeden parametr. Vrátí ho jako value, nebo vrátí chybu.
        fn parse_param(this: &RecursiveScanStrategy, paramstr: Expr) -> Result<Value, CalcError> {
            this.evaluate_rec_simplify(paramstr)
        }
    }

    /// Používá se k rekurzivnímu vyhodnocení výrazu. Výraz vyhodnotí a zjednoduší
    /// (např. zlomek na celé číslo, pokud to jde. BigInt na integer apod.)
    #[inline]
    fn evaluate_rec_simplify(&self, expr: Expr) -> Result<Value, CalcError> {
        self.evaluate_rec(expr)?.simplify_type_move()
    }

    /// Používá se k rekurzivnímu vyhodnocení výrazu.
    fn evaluate_rec(&self, expr: Expr) -> Result<Value, CalcError> {
        let expr = trim_brackets(expr);
        match Self::find_oper(expr.clone()) {
            None => {
                // Není-li ve výrazu dělící operátor, pak to bude buď volání funkce, nebo atomická hodnota
                return if let Ok(func_call) = self.parse_func_call(expr.clone()) {
                    func_call.eval()
                } else {
                    Value::parse(expr.as_str())?.simplify_type_move()
                };
            }
            Some((oper_symbol, oper_pos)) => {
                let (left, right) = Self::halve_expr(expr, oper_pos);
                if left.is_empty() && right.is_empty() {
                    return Err(CalcError::EvaluateErr(format!(
                        "Operátor {oper_symbol} na pozici {oper_pos} nemá žádné operandy"
                    )));
                }
                match oper_symbol {
                    '+' => {
                        return if left.is_empty() {
                            self.evaluate_rec_simplify(right)
                        } else if right.is_empty() {
                            Err(CalcError::EvaluateErr(s!(
                                "Operátoru + chybí pravý operand"
                            )))
                        } else {
                            self.evaluate_rec_simplify(left)? + self.evaluate_rec_simplify(right)?
                        }
                    }
                    '-' => {
                        return if left.is_empty() {
                            let r = self.evaluate_rec_simplify(right)?;
                            r.neg()
                        } else if right.is_empty() {
                            Err(CalcError::EvaluateErr(s!(
                                "Operátoru - chybí pravý operand"
                            )))
                        } else {
                            self.evaluate_rec_simplify(left)? - self.evaluate_rec_simplify(right)?
                        }
                    }
                    '*' => {
                        return if left.is_empty() || right.is_empty() {
                            Err(CalcError::EvaluateErr(String::from(
                                "Operátor * vyžaduje dva operandy",
                            )))
                        } else {
                            self.evaluate_rec_simplify(left)? * self.evaluate_rec_simplify(right)?
                        }
                    }
                    '/' => {
                        return if left.is_empty() || right.is_empty() {
                            Err(CalcError::EvaluateErr(String::from(
                                "Operátor / vyžaduje dva operandy",
                            )))
                        } else {
                            self.evaluate_rec_simplify(left)? / self.evaluate_rec_simplify(right)?
                        }
                    }
                    '^' => {
                        return if left.is_empty() || right.is_empty() {
                            Err(CalcError::EvaluateErr(String::from(
                                "Operátor ^ vyžaduje dva operandy",
                            )))
                        } else {
                            self.evaluate_rec_simplify(left)?
                                .pow(&self.evaluate_rec_simplify(right)?)
                        }
                    }
                    _ => Err(CalcError::EvaluateErr(format!(
                        "Znak '{oper_symbol}' není definovaný operátor"
                    ))),
                }
            }
        }
    }

    fn halve_expr(expr: Expr, oper_pos: usize) -> (Expr, Expr) {
        if oper_pos == 0 {
            (Expr::new(""), Expr::new(&expr.as_str()[1..].trim()))
        } else {
            (
                Expr::new(&expr.as_str()[..oper_pos].trim()),
                Expr::new(&expr.as_str()[oper_pos + 1..]),
            )
        }
    }

    /// Vrací nalezený operátor a jeho pozici v textu.
    /// POZOR! Nejedná se o pozici ve smyslu index znaku, ale index bajtu!
    /// Znak operátoru má mít jeden bajt, ale jiné znaky UTF-8 mohou mít víc bajtů.
    fn find_oper(expr: Expr) -> Option<(char, usize)> {
        let expr_bytes_len = expr.as_str().len();
        let mut is_in_string = false;
        let mut curr_depth = 0;
        let mut best_oper_priority = i32::MAX;
        let mut best_oper_pos = usize::MAX;
        let mut best_operator_symbol = '\0';

        let mut bytes_scanned: usize = 0;
        for c in expr.as_str().chars().rev() {
            if c == '"' {
                is_in_string = !is_in_string;
            } else if !is_in_string {
                if c == ')' {
                    curr_depth += 1;
                } else if c == '(' {
                    curr_depth -= 1;
                } else {
                    if let Some(oper_priority) = is_operator_get_priority(c) {
                        if curr_depth == 0 // Operátor dělení výrazu nesmí být v závorkách!
                        && oper_priority < best_oper_priority
                        {
                            best_oper_priority = oper_priority;
                            best_oper_pos = expr_bytes_len - bytes_scanned - 1;
                            best_operator_symbol = c;
                        }
                    }
                }
            }
            // Každý znak má jiný počet bajtů (UTF-8). Pozice dělícího operátoru se určuje indexem bajtu!
            bytes_scanned += c.len_utf8();
        }

        return if best_oper_pos == usize::MAX {
            None
        } else {
            Some((best_operator_symbol, best_oper_pos))
        };
    }
}
