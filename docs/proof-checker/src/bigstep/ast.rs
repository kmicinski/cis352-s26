use std::collections::BTreeMap;

/// Expressions in the object language.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Var(String),
    Int(i64),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    If0(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    Lam(String, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
}

/// Runtime values.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Closure {
        param: String,
        body: Expr,
        env: Env,
    },
}

/// Environment: finite map from variable names to values.
/// Uses BTreeMap for deterministic ordering.
pub type Env = BTreeMap<String, Value>;

/// A parsed judgement: ρ ⊢ e ⇓ v
#[derive(Debug, Clone)]
pub struct Judgement {
    pub env: Env,
    pub expr: Expr,
    pub value: Value,
}

impl Expr {
    /// Returns the "form name" for error messages.
    pub fn form_name(&self) -> &str {
        match self {
            Expr::Var(_) => "variable",
            Expr::Int(_) => "integer literal",
            Expr::Neg(_) => "(- e)",
            Expr::Add(_, _) => "(+ e₁ e₂)",
            Expr::If0(_, _, _) => "(if0 eg et ef)",
            Expr::Let(_, _, _) => "(let ([x e]) eb)",
            Expr::Lam(_, _) => "(λ (x) e)",
            Expr::App(_, _) => "(e₁ e₂)",
        }
    }
}
