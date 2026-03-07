pub mod term;
pub mod parser;
pub mod render;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use crate::term::{Term, Strategy, parse_path};
use crate::parser::parse;
use crate::render::{build_render_tree, build_term_info};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

/// Main engine exposed to JavaScript
#[wasm_bindgen]
pub struct LambdaEngine {
    current: Option<Term>,
    history: Vec<Term>,
    strategy: Strategy,
}

#[wasm_bindgen]
impl LambdaEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LambdaEngine {
        // Could add console_error_panic_hook here if desired
        LambdaEngine {
            current: None,
            history: Vec::new(),
            strategy: Strategy::Normal,
        }
    }

    /// Parse a CIS352-syntax term and set it as current
    pub fn parse_and_set(&mut self, input: &str) -> Result<String, JsValue> {
        let term = parse(input).map_err(|e| JsValue::from_str(&e))?;
        self.history.clear();
        self.current = Some(term);
        self.get_render_tree()
    }

    /// Generate a random term with depth sampled from a Pareto distribution.
    /// Most terms are sizable (depth 5-8), occasionally huge (depth 15+).
    pub fn random_term(&mut self, closed: bool) -> String {
        let var_pool = vec![
            "x".to_string(), "y".to_string(), "z".to_string(),
            "w".to_string(), "u".to_string(), "v".to_string(),
        ];
        // Pareto(x_min=5, α=2.5): long tail, median ~6.6, ~1.5% chance of depth 18+
        let u = random().max(0.001);
        let depth = ((5.0_f64 * u.powf(-1.0 / 2.5)).floor() as u32).min(18);
        let term = gen_random_term(depth, &if closed { vec![] } else { var_pool.clone() }, &var_pool, closed);
        self.history.clear();
        self.current = Some(term);
        self.get_render_tree().unwrap_or_default()
    }

    /// Get the render tree as JSON
    pub fn get_render_tree(&self) -> Result<String, JsValue> {
        let term = self.current.as_ref()
            .ok_or_else(|| JsValue::from_str("No term loaded"))?;
        let scope = HashMap::new();
        let next_path = term.next_redex(&self.strategy)
            .map(|p| term::encode_path(&p));
        let tree = build_render_tree(
            term, "", &scope,
            Some(&self.strategy),
            next_path.as_deref(),
        );
        serde_json::to_string(&tree)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get term info for the status bar
    pub fn get_term_info(&self) -> Result<String, JsValue> {
        let term = self.current.as_ref()
            .ok_or_else(|| JsValue::from_str("No term loaded"))?;
        let info = build_term_info(term, &self.strategy);
        serde_json::to_string(&info)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Get display string in CIS352 syntax
    pub fn get_display(&self) -> String {
        self.current.as_ref()
            .map(|t| t.display_string())
            .unwrap_or_default()
    }

    /// Apply an operation at the given path
    /// op: "beta", "eta", "alpha"
    /// arg: for alpha, the new variable name
    pub fn apply_operation(&mut self, path_str: &str, op: &str, arg: &str) -> Result<String, JsValue> {
        let term = self.current.as_ref()
            .ok_or_else(|| JsValue::from_str("No term loaded"))?;
        let path = parse_path(path_str)
            .map_err(|e| JsValue::from_str(&e))?;

        let new_term = match op {
            "beta" => {
                term.map_at_path(&path, |t| t.beta_reduce())
                    .ok_or_else(|| JsValue::from_str("Beta reduction failed"))?
            }
            "eta" => {
                term.map_at_path(&path, |t| t.eta_convert())
                    .ok_or_else(|| JsValue::from_str("Eta conversion failed"))?
            }
            "alpha" => {
                if arg.is_empty() {
                    return Err(JsValue::from_str("Alpha conversion requires a new variable name"));
                }
                let alpha_err: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
                let result = term.map_at_path(&path, |t| {
                    match t.alpha_convert(arg) {
                        Ok(new_term) => Some(new_term),
                        Err(msg) => {
                            *alpha_err.borrow_mut() = Some(msg);
                            None
                        }
                    }
                });
                match result {
                    Some(new_term) => new_term,
                    None => {
                        let msg = alpha_err.borrow().clone()
                            .unwrap_or_else(|| "Alpha conversion failed at this path".to_string());
                        return Err(JsValue::from_str(&msg));
                    }
                }
            }
            _ => return Err(JsValue::from_str(&format!("Unknown operation: {}", op))),
        };

        self.history.push(term.clone());
        self.current = Some(new_term);
        self.get_render_tree()
    }

    /// Reduce one step according to the current strategy
    pub fn step_strategy(&mut self) -> Result<String, JsValue> {
        let term = self.current.as_ref()
            .ok_or_else(|| JsValue::from_str("No term loaded"))?;
        let new_term = term.reduce_step(&self.strategy)
            .ok_or_else(|| JsValue::from_str("Term is in normal form"))?;
        self.history.push(term.clone());
        self.current = Some(new_term);
        self.get_render_tree()
    }

    /// Set the reduction strategy
    pub fn set_strategy(&mut self, name: &str) -> Result<(), JsValue> {
        self.strategy = Strategy::from_str(name)
            .ok_or_else(|| JsValue::from_str(&format!("Unknown strategy: {}", name)))?;
        Ok(())
    }

    pub fn get_strategy(&self) -> String {
        match self.strategy {
            Strategy::Normal => "normal".to_string(),
            Strategy::Applicative => "applicative".to_string(),
            Strategy::CallByValue => "cbv".to_string(),
            Strategy::CallByName => "cbn".to_string(),
        }
    }

    /// Undo the last operation
    pub fn undo(&mut self) -> Result<String, JsValue> {
        let prev = self.history.pop()
            .ok_or_else(|| JsValue::from_str("Nothing to undo"))?;
        self.current = Some(prev);
        self.get_render_tree()
    }

    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn history_length(&self) -> usize {
        self.history.len()
    }
}

/// Random term generation using js_sys::Math::random()
fn rand_usize(max: usize) -> usize {
    (random() * max as f64).floor() as usize
}

fn rand_bool(p: f64) -> bool {
    random() < p
}

fn gen_random_term(
    depth: u32,
    scope: &[String],
    var_pool: &[String],
    closed: bool,
) -> Term {
    if depth == 0 {
        // Must produce a variable
        if scope.is_empty() && closed {
            // No variables in scope; wrap in a lambda
            let param = var_pool[rand_usize(var_pool.len())].clone();
            return Term::Abs(param.clone(), Box::new(Term::Var(param)));
        }
        let choices: &[String] = if closed && !scope.is_empty() { scope } else if !scope.is_empty() {
            scope
        } else {
            var_pool
        };
        return Term::Var(choices[rand_usize(choices.len())].clone());
    }

    // Bias towards interesting terms
    let r = random();
    if scope.is_empty() && closed {
        // Must create a lambda to get variables in scope
        let param = var_pool[rand_usize(var_pool.len())].clone();
        let mut new_scope = scope.to_vec();
        new_scope.push(param.clone());
        let body = gen_random_term(depth - 1, &new_scope, var_pool, closed);
        return Term::Abs(param, Box::new(body));
    }

    if r < 0.25 {
        // Lambda
        let param = var_pool[rand_usize(var_pool.len())].clone();
        let mut new_scope = scope.to_vec();
        new_scope.push(param.clone());
        let body = gen_random_term(depth - 1, &new_scope, var_pool, closed);
        Term::Abs(param, Box::new(body))
    } else if r < 0.80 {
        // Application (sometimes create a redex)
        if rand_bool(0.3) && depth >= 2 {
            // Create a redex: (λx.body) arg
            let param = var_pool[rand_usize(var_pool.len())].clone();
            let mut new_scope = scope.to_vec();
            new_scope.push(param.clone());
            let body = gen_random_term(depth - 2, &new_scope, var_pool, closed);
            let arg = gen_random_term(depth - 2, scope, var_pool, closed);
            Term::App(
                Box::new(Term::Abs(param, Box::new(body))),
                Box::new(arg),
            )
        } else {
            let func = gen_random_term(depth - 1, scope, var_pool, closed);
            let arg = gen_random_term(depth - 1, scope, var_pool, closed);
            Term::App(Box::new(func), Box::new(arg))
        }
    } else {
        // Variable
        let choices: &[String] = if closed { scope } else { var_pool };
        if choices.is_empty() {
            let param = var_pool[rand_usize(var_pool.len())].clone();
            Term::Var(param)
        } else {
            Term::Var(choices[rand_usize(choices.len())].clone())
        }
    }
}
