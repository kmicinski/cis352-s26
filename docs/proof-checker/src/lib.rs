pub mod sexp;
pub mod tree;
pub mod check;
pub mod bigstep;
pub mod formula;
pub mod types;
pub mod g3ip;
pub mod propnd;
pub mod stlc;
pub mod systemf;
pub mod smallstep;

use wasm_bindgen::prelude::*;

/// Check a proof tree S-expression against a named theory.
/// Returns JSON: { valid, complete, diagnostics: [{level, path, message}] }
#[wasm_bindgen]
pub fn check_proof(sexp_str: &str, theory: &str) -> String {
    let node = match sexp::parse_proof_sexp(sexp_str) {
        Ok(n) => n,
        Err(e) => {
            let result = check::CheckResult {
                valid: false,
                complete: false,
                diagnostics: vec![check::Diagnostic {
                    level: check::Level::Error,
                    path: vec![],
                    message: format!("Failed to parse proof tree: {}", e),
                }],
            };
            return serde_json::to_string(&result).unwrap();
        }
    };

    let available = "big-step, small-step, g3ip, propnd, stlc, systemf";

    let result = match theory {
        "big-step" | "bigstep" | "" => {
            let th = bigstep::BigStepTheory;
            check::check_tree(&node, &th)
        }
        "small-step" | "smallstep" => {
            let th = smallstep::SmallStepTheory;
            check::check_tree(&node, &th)
        }
        "g3ip" | "G3ip" | "sequent" => {
            let th = g3ip::G3ipTheory;
            check::check_tree(&node, &th)
        }
        "propnd" | "prop-nd" | "natural-deduction" => {
            let th = propnd::PropNDTheory;
            check::check_tree(&node, &th)
        }
        "stlc" | "STLC" | "simply-typed" => {
            let th = stlc::STLCTheory;
            check::check_tree(&node, &th)
        }
        "systemf" | "system-f" | "SystemF" => {
            let th = systemf::SystemFTheory;
            check::check_tree(&node, &th)
        }
        _ => check::CheckResult {
            valid: false,
            complete: false,
            diagnostics: vec![check::Diagnostic {
                level: check::Level::Error,
                path: vec![],
                message: format!("Unknown theory '{}'. Available: {}", theory, available),
            }],
        },
    };

    serde_json::to_string(&result).unwrap()
}

/// Parse a judgement string for debugging. Returns JSON.
#[wasm_bindgen]
pub fn parse_judgement(s: &str) -> String {
    match bigstep::parse::parse_judgement(s) {
        Ok(j) => serde_json::to_string(&serde_json::json!({
            "ok": true,
            "env": format!("{:?}", j.env),
            "expr": format!("{:?}", j.expr),
            "value": format!("{:?}", j.value),
        }))
        .unwrap(),
        Err(e) => serde_json::to_string(&serde_json::json!({
            "ok": false,
            "error": e,
        }))
        .unwrap(),
    }
}

/// List available theories. Returns JSON array of {id, name} objects.
#[wasm_bindgen]
pub fn list_theories() -> String {
    serde_json::to_string(&serde_json::json!([
        {"id": "big-step", "name": "Big-Step Operational Semantics"},
        {"id": "small-step", "name": "Small-Step Operational Semantics"},
        {"id": "g3ip", "name": "G3ip Sequent Calculus"},
        {"id": "propnd", "name": "Propositional Natural Deduction"},
        {"id": "stlc", "name": "Simply-Typed Lambda Calculus"},
        {"id": "systemf", "name": "System F"},
    ]))
    .unwrap()
}
