use crate::calc_base::MathEvaluateError;
use crate::calc_base::value::Value;
use crate::calc_base::std_funcs;

#[derive(Debug, Clone)]
pub struct FuncCall{
    name: String,
    params: Vec<Value>,
}

impl FuncCall {
    pub fn new(name: &str, params: Vec<Value>) -> FuncCall {
        FuncCall {
            name: name.trim().to_lowercase(),
            params
        }
    }
    pub fn name(&self) -> &str {self.name.as_str()}

    /// Najde funkci s daným názvem a zavolá ji. Pokud funkce neexistuje, nebo se
    /// nepovede výpočet, vrátí chybu.
    pub fn eval(&self) -> Result<Value, MathEvaluateError> {
        match self.name.as_str() {
            "abs" => {
                if self.params.len() == 1 {
                    std_funcs::abs(self.params[0].clone().simplify_type_move()?)
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 1 parametr", self.name)))
                }
            },
            "comb" => {
                if self.params.len() == 3 {
                    std_funcs::comb(self.params[0].clone().simplify_type_move()?,
                                   self.params[1].clone().simplify_type_move()?,
                                       self.params[2].clone())
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 3 parametry (int, int, bool)", self.name)))
                }
            },
            "nck" => {
                if self.params.len() == 2 {
                    std_funcs::nck(self.params[0].clone().simplify_type_move()?,
                                   self.params[1].clone().simplify_type_move()?)
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 2 parametry (int, int)", self.name)))
                }
            },
            "fact" => {
                if self.params.len() == 1 {
                    std_funcs::fact(self.params[0].clone().simplify_type_move()?)
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 1 parametr", self.name)))
                }
            },
            "max" => {
                if self.params.len() == 0 {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje aspoň 1 parametr", self.name)))
                } else {
                    std_funcs::max(&self.params)
                }
            },
            "sqrt" => {
                if self.params.len() == 1 {
                    std_funcs::sqrt(self.params[0].clone().simplify_type_move()?)
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 1 parametr", self.name)))
                }
            },
            "sin" => {
                if self.params.len() == 1 {
                    std_funcs::sin(self.params[0].clone().simplify_type_move()?)
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 1 parametr", self.name)))
                }
            },
            "sind" => {
                if self.params.len() == 1 {
                    std_funcs::sind(self.params[0].clone().simplify_type_move()?)
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 1 parametr", self.name)))
                }
            },
            "sinpi" => {
                if self.params.len() == 1 {
                    std_funcs::sinpi(self.params[0].clone().simplify_type_move()?)
                } else {
                    Err(MathEvaluateError::new(format!("Funkce '{}' vyžaduje 1 parametr", self.name)))
                }
            },
            _ => Err(MathEvaluateError::new(format!("Funkce '{}' není definována", self.name)))
        }
    }

    pub fn params_as_string(&self) -> String {
        let params_as_strings : Vec<_> = self.params.iter().map(|val| val.to_string()).collect();
        let joined = params_as_strings.join(", ");
        joined
    }
}