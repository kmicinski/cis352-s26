use super::ast::*;
use super::parse::{self, SideCondition};
use crate::check::{Diagnostic, Level};
use crate::tree::ProofNode;

/// Check a rule application against the 9 big-step rules.
/// Returns diagnostics for this node only (not recursive).
pub fn check_rule(
    rule_name: &str,
    conclusion: &Judgement,
    premises: &[&ProofNode],
) -> Vec<Diagnostic> {
    match rule_name {
        "Var" => check_var(conclusion, premises),
        "Int" => check_int(conclusion, premises),
        "Neg" => check_neg(conclusion, premises),
        "Add" => check_add(conclusion, premises),
        "If0-True" => check_if0_true(conclusion, premises),
        "If0-False" => check_if0_false(conclusion, premises),
        "Let" => check_let(conclusion, premises),
        "Lam" => check_lam(conclusion, premises),
        "App" => check_app(conclusion, premises),
        _ => vec![Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("Unknown rule '{}'", rule_name),
        }],
    }
}

// ── Helper: compare values allowing some flexibility ─────────────────

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(i1), Value::Int(i2)) => i1 == i2,
        (
            Value::Closure {
                param: p1,
                body: b1,
                env: e1,
            },
            Value::Closure {
                param: p2,
                body: b2,
                env: e2,
            },
        ) => p1 == p2 && b1 == b2 && e1 == e2,
        _ => false,
    }
}

fn exprs_equal(a: &Expr, b: &Expr) -> bool {
    a == b
}

fn envs_equal(a: &Env, b: &Env) -> bool {
    a == b
}

// ── Helper: try parsing a premise's conclusion as a judgement ────────

fn parse_premise_judgement(premise: &ProofNode) -> Result<Judgement, String> {
    parse::parse_judgement(&premise.conclusion)
}

fn parse_premise_side_condition(premise: &ProofNode) -> Option<SideCondition> {
    parse::parse_side_condition(&premise.conclusion)
}

// ── Var rule ────────────────────────────────────────────────────────
// Conclusion: ρ ⊢ x ⇓ v
// Premises: 1 side condition: ρ(x) = v

fn check_var(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    // Expression must be a variable
    let var_name = match &j.expr {
        Expr::Var(x) => x.clone(),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "The Var rule expects a variable, but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    // Should have exactly 1 premise (the lookup side condition)
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("Var expects 1 premise (ρ(x) = v), got {}", premises.len()),
        });
        return diags;
    }

    // Check the side condition
    match parse_premise_side_condition(premises[0]) {
        Some(SideCondition::Lookup { var, val_str, .. }) => {
            // Check variable name matches
            if var != var_name {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: format!(
                        "Lookup variable '{}' doesn't match expression variable '{}'",
                        var, var_name
                    ),
                });
            }
            // Check the lookup result matches the conclusion value
            if let Ok(lookup_val) = parse::parse_value(&val_str) {
                if !values_equal(&lookup_val, &j.value) {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!(
                            "Lookup result {:?} doesn't match conclusion value {:?}",
                            lookup_val, j.value
                        ),
                    });
                }
            }
            // Check the actual environment lookup
            match j.env.get(&var_name) {
                Some(env_val) => {
                    if !values_equal(env_val, &j.value) {
                        diags.push(Diagnostic {
                            level: Level::Error,
                            path: vec![],
                            message: format!(
                                "ρ({}) = {:?}, but the conclusion claims the value is {:?}",
                                var_name, env_val, j.value
                            ),
                        });
                    }
                }
                None => {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![],
                        message: format!(
                            "Variable '{}' is not bound in the environment",
                            var_name
                        ),
                    });
                }
            }
        }
        _ => {
            // If the premise has content but isn't a valid lookup, flag it
            if !premises[0].conclusion.trim().is_empty() {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: format!(
                        "Var premise should be ρ({}) = v, got: {}",
                        var_name, premises[0].conclusion
                    ),
                });
            } else {
                diags.push(Diagnostic {
                    level: Level::Incomplete,
                    path: vec![0],
                    message: "Var premise not filled in".to_string(),
                });
            }
        }
    }

    diags
}

// ── Int rule ────────────────────────────────────────────────────────
// Conclusion: ρ ⊢ i ⇓ i
// Premises: 0

fn check_int(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    // Expression must be an integer
    let i = match &j.expr {
        Expr::Int(i) => *i,
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "The Int rule expects an integer literal, but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    // No premises
    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("Int expects 0 premises, got {}", premises.len()),
        });
    }

    // Value must equal expression
    match &j.value {
        Value::Int(v) if *v == i => {}
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("Int rule: value must equal the literal ({}), got {:?}", i, j.value),
            });
        }
    }

    diags
}

// ── Neg rule ────────────────────────────────────────────────────────
// Conclusion: ρ ⊢ (- e) ⇓ v
// Premises: 2 — ρ ⊢ e ⇓ i, then v = -i

fn check_neg(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let inner = match &j.expr {
        Expr::Neg(e) => e.as_ref(),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "The Neg rule expects (- e), but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("Neg expects 2 premises (ρ ⊢ e ⇓ i and v = -i), got {}", premises.len()),
        });
        return diags;
    }

    // First premise: ρ ⊢ e ⇓ i
    if let Ok(pj) = parse_premise_judgement(premises[0]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise environment doesn't match conclusion environment".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, inner) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "Premise expression should be the operand of (- ...), got {:?}",
                    pj.expr
                ),
            });
        }
        // Check arithmetic: value should be negation of premise value
        if let (Value::Int(pv), Value::Int(cv)) = (&pj.value, &j.value) {
            if *cv != -*pv {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: format!(
                        "Neg arithmetic error: -({}) = {}, but conclusion says {}",
                        pv,
                        -*pv,
                        cv
                    ),
                });
            }
        }
    }

    // Second premise: v = -i (side condition)
    if let Some(sc) = parse_premise_side_condition(premises[1]) {
        match sc {
            SideCondition::NegResult { result_str, operand_str } => {
                if let (Ok(result), Ok(_operand)) =
                    (parse::parse_side_int(&result_str), parse::parse_side_int(&operand_str))
                {
                    if let Value::Int(cv) = &j.value {
                        if result != *cv {
                            diags.push(Diagnostic {
                                level: Level::Error,
                                path: vec![1],
                                message: format!(
                                    "Side condition says result is {}, but conclusion value is {}",
                                    result, cv
                                ),
                            });
                        }
                    }
                }
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![1],
                    message: "Second premise should be v = -i".to_string(),
                });
            }
        }
    }

    diags
}

// ── Add rule ────────────────────────────────────────────────────────
// Conclusion: ρ ⊢ (+ e₁ e₂) ⇓ v
// Premises: 3 — ρ ⊢ e₁ ⇓ i₁, ρ ⊢ e₂ ⇓ i₂, v = i₁ + i₂

fn check_add(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let (left, right) = match &j.expr {
        Expr::Add(l, r) => (l.as_ref(), r.as_ref()),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "The Add rule expects (+ e₁ e₂), but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    if premises.len() != 3 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "Add expects 3 premises (ρ ⊢ e₁ ⇓ i₁, ρ ⊢ e₂ ⇓ i₂, v = i₁+i₂), got {}",
                premises.len()
            ),
        });
        return diags;
    }

    // First premise: ρ ⊢ e₁ ⇓ i₁
    let mut v0 = None;
    if let Ok(pj) = parse_premise_judgement(premises[0]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "First premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, left) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("First premise expression should be left operand of +"),
            });
        }
        v0 = Some(pj.value);
    }

    // Second premise: ρ ⊢ e₂ ⇓ i₂
    let mut v1 = None;
    if let Ok(pj) = parse_premise_judgement(premises[1]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: "Second premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, right) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!("Second premise expression should be right operand of +"),
            });
        }
        v1 = Some(pj.value);
    }

    // Third premise: v = i₁ + i₂ (side condition)
    if let Some(sc) = parse_premise_side_condition(premises[2]) {
        match sc {
            SideCondition::AddResult {
                result_str,
                left_str,
                right_str,
            } => {
                if let Ok(result) = parse::parse_side_int(&result_str) {
                    if let Ok(l) = parse::parse_side_int(&left_str) {
                        if let Ok(r) = parse::parse_side_int(&right_str) {
                            if result != l + r {
                                diags.push(Diagnostic {
                                    level: Level::Error,
                                    path: vec![2],
                                    message: format!(
                                        "Arithmetic error: {} + {} = {}, not {}",
                                        l,
                                        r,
                                        l + r,
                                        result
                                    ),
                                });
                            }
                        }
                    }
                    if let Value::Int(cv) = &j.value {
                        if result != *cv {
                            diags.push(Diagnostic {
                                level: Level::Error,
                                path: vec![2],
                                message: format!(
                                    "Side condition result {} doesn't match conclusion value {}",
                                    result, cv
                                ),
                            });
                        }
                    }
                }
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![2],
                    message: "Third premise should be v = i₁ + i₂".to_string(),
                });
            }
        }
    }

    // Check overall arithmetic
    if let (Some(Value::Int(i0)), Some(Value::Int(i1)), Value::Int(cv)) =
        (&v0, &v1, &j.value)
    {
        if *cv != i0 + i1 {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "Add arithmetic: {} + {} = {}, but conclusion says {}",
                    i0,
                    i1,
                    i0 + i1,
                    cv
                ),
            });
        }
    }

    diags
}

// ── If0-True rule ───────────────────────────────────────────────────
// Conclusion: ρ ⊢ (if0 eg et ef) ⇓ v
// Premises: 2 — ρ ⊢ eg ⇓ 0, ρ ⊢ et ⇓ v

fn check_if0_true(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let (guard, true_branch, _false_branch) = match &j.expr {
        Expr::If0(g, t, f) => (g.as_ref(), t.as_ref(), f.as_ref()),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "If0-True expects (if0 eg et ef), but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "If0-True expects 2 premises (ρ ⊢ eg ⇓ 0, ρ ⊢ et ⇓ v), got {}",
                premises.len()
            ),
        });
        return diags;
    }

    // First premise: ρ ⊢ eg ⇓ 0
    if let Ok(pj) = parse_premise_judgement(premises[0]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Guard premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, guard) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Guard premise expression doesn't match if0 guard".to_string(),
            });
        }
        if pj.value != Value::Int(0) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "If0-True requires the guard to evaluate to 0, but got {:?}",
                    pj.value
                ),
            });
        }
    }

    // Second premise: ρ ⊢ et ⇓ v
    if let Ok(pj) = parse_premise_judgement(premises[1]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: "True-branch premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, true_branch) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: "True-branch premise expression doesn't match if0 true branch".to_string(),
            });
        }
        if !values_equal(&pj.value, &j.value) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!(
                    "True-branch result {:?} doesn't match conclusion value {:?}",
                    pj.value, j.value
                ),
            });
        }
    }

    diags
}

// ── If0-False rule ──────────────────────────────────────────────────
// Conclusion: ρ ⊢ (if0 eg et ef) ⇓ v
// Premises: 3 — ρ ⊢ eg ⇓ i, i ≠ 0, ρ ⊢ ef ⇓ v

fn check_if0_false(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let (guard, _true_branch, false_branch) = match &j.expr {
        Expr::If0(g, t, f) => (g.as_ref(), t.as_ref(), f.as_ref()),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "If0-False expects (if0 eg et ef), but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    if premises.len() != 3 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "If0-False expects 3 premises (ρ ⊢ eg ⇓ i, i ≠ 0, ρ ⊢ ef ⇓ v), got {}",
                premises.len()
            ),
        });
        return diags;
    }

    // First premise: ρ ⊢ eg ⇓ i
    if let Ok(pj) = parse_premise_judgement(premises[0]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Guard premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, guard) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Guard premise expression doesn't match if0 guard".to_string(),
            });
        }
        if pj.value == Value::Int(0) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "If0-False requires the guard to be nonzero, but it evaluated to 0. Use If0-True instead.".to_string(),
            });
        }
    }

    // Second premise: i ≠ 0 (side condition)
    if let Some(sc) = parse_premise_side_condition(premises[1]) {
        match sc {
            SideCondition::Nonzero { val_str } => {
                if let Ok(val) = parse::parse_side_int(&val_str) {
                    if val == 0 {
                        diags.push(Diagnostic {
                            level: Level::Error,
                            path: vec![1],
                            message: "Nonzero side condition claims 0 ≠ 0, which is false"
                                .to_string(),
                        });
                    }
                }
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![1],
                    message: "Second premise should be i ≠ 0".to_string(),
                });
            }
        }
    }

    // Third premise: ρ ⊢ ef ⇓ v
    if let Ok(pj) = parse_premise_judgement(premises[2]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![2],
                message: "False-branch premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, false_branch) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![2],
                message: "False-branch premise expression doesn't match if0 false branch"
                    .to_string(),
            });
        }
        if !values_equal(&pj.value, &j.value) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![2],
                message: format!(
                    "False-branch result {:?} doesn't match conclusion value {:?}",
                    pj.value, j.value
                ),
            });
        }
    }

    diags
}

// ── Let rule ────────────────────────────────────────────────────────
// Conclusion: ρ ⊢ (let ([x e]) eb) ⇓ v₂
// Premises: 2 — ρ ⊢ e ⇓ v₁, ρ[x ↦ v₁] ⊢ eb ⇓ v₂

fn check_let(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let (var, bind_expr, body_expr) = match &j.expr {
        Expr::Let(x, e, eb) => (x.clone(), e.as_ref(), eb.as_ref()),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "The Let rule expects (let ([x e]) eb), but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "Let expects 2 premises (ρ ⊢ e ⇓ v₁, ρ[x↦v₁] ⊢ eb ⇓ v₂), got {}",
                premises.len()
            ),
        });
        return diags;
    }

    // First premise: ρ ⊢ e ⇓ v₁
    let mut v1 = None;
    if let Ok(pj) = parse_premise_judgement(premises[0]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "First premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, bind_expr) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "First premise expression doesn't match let binding expression"
                    .to_string(),
            });
        }
        v1 = Some(pj.value);
    }

    // Second premise: ρ[x ↦ v₁] ⊢ eb ⇓ v₂
    if let Ok(pj) = parse_premise_judgement(premises[1]) {
        // Check the environment is ρ extended with x ↦ v₁
        if let Some(ref bound_val) = v1 {
            let mut expected_env = j.env.clone();
            expected_env.insert(var.clone(), bound_val.clone());
            if !envs_equal(&pj.env, &expected_env) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![1],
                    message: format!(
                        "Second premise environment should be ρ[{} ↦ {:?}]",
                        var, bound_val
                    ),
                });
            }
        }
        if !exprs_equal(&pj.expr, body_expr) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: "Second premise expression doesn't match let body".to_string(),
            });
        }
        if !values_equal(&pj.value, &j.value) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!(
                    "Body result {:?} doesn't match conclusion value {:?}",
                    pj.value, j.value
                ),
            });
        }
    }

    diags
}

// ── Lam rule ────────────────────────────────────────────────────────
// Conclusion: ρ ⊢ (λ (x) e) ⇓ ⟨λ (x) e , ρ⟩
// Premises: 0

fn check_lam(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let (param, body) = match &j.expr {
        Expr::Lam(x, e) => (x.clone(), e.as_ref().clone()),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "The Lam rule expects (λ (x) e), but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("Lam expects 0 premises, got {}", premises.len()),
        });
    }

    // Value must be ⟨λ (x) e , ρ⟩
    match &j.value {
        Value::Closure {
            param: vp,
            body: vb,
            env: ve,
        } => {
            if *vp != param {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: format!(
                        "Closure parameter '{}' doesn't match lambda parameter '{}'",
                        vp, param
                    ),
                });
            }
            if *vb != body {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: "Closure body doesn't match lambda body".to_string(),
                });
            }
            if !envs_equal(ve, &j.env) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: "Closure environment should capture the current environment ρ"
                        .to_string(),
                });
            }
        }
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "Lam rule: value must be a closure ⟨λ (x) e , ρ⟩".to_string(),
            });
        }
    }

    diags
}

// ── App rule ────────────────────────────────────────────────────────
// Conclusion: ρ ⊢ (e₁ e₂) ⇓ v
// Premises: 3 — ρ ⊢ e₁ ⇓ ⟨λ (x) ebody , ρclo⟩, ρ ⊢ e₂ ⇓ varg,
//               ρclo[x ↦ varg] ⊢ ebody ⇓ v

fn check_app(j: &Judgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let (func_expr, arg_expr) = match &j.expr {
        Expr::App(f, a) => (f.as_ref(), a.as_ref()),
        other => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!(
                    "The App rule expects (e₁ e₂), but the expression is {}",
                    other.form_name()
                ),
            });
            return diags;
        }
    };

    if premises.len() != 3 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "App expects 3 premises (function eval, arg eval, body eval), got {}",
                premises.len()
            ),
        });
        return diags;
    }

    // First premise: ρ ⊢ e₁ ⇓ ⟨λ (x) ebody , ρclo⟩
    let mut closure_info = None;
    if let Ok(pj) = parse_premise_judgement(premises[0]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Function premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, func_expr) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Function premise expression doesn't match".to_string(),
            });
        }
        match &pj.value {
            Value::Closure { param, body, env } => {
                closure_info = Some((param.clone(), body.clone(), env.clone()));
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: "App: function must evaluate to a closure".to_string(),
                });
            }
        }
    }

    // Second premise: ρ ⊢ e₂ ⇓ varg
    let mut arg_val = None;
    if let Ok(pj) = parse_premise_judgement(premises[1]) {
        if !envs_equal(&pj.env, &j.env) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: "Argument premise environment doesn't match".to_string(),
            });
        }
        if !exprs_equal(&pj.expr, arg_expr) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: "Argument premise expression doesn't match".to_string(),
            });
        }
        arg_val = Some(pj.value);
    }

    // Third premise: ρclo[x ↦ varg] ⊢ ebody ⇓ v
    if let Ok(pj) = parse_premise_judgement(premises[2]) {
        if let Some((ref param, ref body, ref clo_env)) = closure_info {
            // Check environment: ρclo[x ↦ varg]
            if let Some(ref av) = arg_val {
                let mut expected_env = clo_env.clone();
                expected_env.insert(param.clone(), av.clone());
                if !envs_equal(&pj.env, &expected_env) {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![2],
                        message: format!(
                            "Body premise environment should be ρ_clo[{} ↦ {:?}]",
                            param, av
                        ),
                    });
                }
            }
            // Check expression matches closure body
            if !exprs_equal(&pj.expr, body) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![2],
                    message: "Body premise expression doesn't match closure body".to_string(),
                });
            }
        }
        // Check result matches conclusion
        if !values_equal(&pj.value, &j.value) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![2],
                message: format!(
                    "Body result {:?} doesn't match conclusion value {:?}",
                    pj.value, j.value
                ),
            });
        }
    }

    diags
}
