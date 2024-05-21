use std::str::Chars;

/// Obsahuje syntakticky platný již zkontrolovaný výraz.
/// To ale neznamená, že v něm nemohou být chyby, např. špatné parametry funkcí. Různé
/// chyby se odhalí různě v závislosti na použité výpočetní strategii kalkulátoru
#[derive(Debug)]
pub struct Expr<'expr> {
    expr: &'expr str,
}

impl<'expr> Expr<'expr> {
    pub fn new(expr: &'expr str) -> Self {
        Expr { expr: expr.trim() }
    }

    pub fn as_str(&self) -> &'expr str {
        self.expr
    }

    pub fn dupl(&self) -> Self {
        Expr::new(self.expr)
    }

    pub fn is_empty(&self) -> bool {
        self.expr.is_empty()
    }

    pub fn chars(&self) -> Chars<'_> {
        self.expr.chars()
    }
}

impl<'expr> Default for Expr<'expr> {
    fn default() -> Self {
        Expr::new("")
    }
}
