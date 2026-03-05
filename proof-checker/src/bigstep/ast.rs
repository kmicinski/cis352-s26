use std::collections::BTreeMap;
use std::fmt;

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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(x) => write!(f, "{}", x),
            Expr::Int(i) => write!(f, "{}", i),
            Expr::Neg(e) => write!(f, "(- {})", e),
            Expr::Add(a, b) => write!(f, "(+ {} {})", a, b),
            Expr::If0(g, t, e) => write!(f, "(if0 {} {} {})", g, t, e),
            Expr::Let(x, e, b) => write!(f, "(let ([{} {}]) {})", x, e, b),
            Expr::Lam(x, b) => write!(f, "(λ ({}) {})", x, b),
            Expr::App(a, b) => write!(f, "({} {})", a, b),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Closure { param, body, env } => {
                write!(f, "⟨λ ({}) {} , {}⟩", param, body, format_env(env))
            }
        }
    }
}

pub fn format_env(env: &Env) -> String {
    if env.is_empty() {
        return "{}".to_string();
    }
    let bindings: Vec<String> = env
        .iter()
        .map(|(k, v)| format!("{} ↦ {}", k, v))
        .collect();
    format!("{{{}}}", bindings.join(", "))
}
