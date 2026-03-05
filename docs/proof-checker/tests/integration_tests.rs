use proof_checker::check::{check_tree, Level};
use proof_checker::bigstep::BigStepTheory;
use proof_checker::sexp::parse_proof_sexp;

fn check(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    let theory = BigStepTheory;
    check_tree(&node, &theory)
}

// ── Exercise 1 solution: {x ↦ 3} ⊢ (- x) ⇓ -3 ────────────────────

#[test]
fn test_exercise1_solution_valid() {
    let sexp = r#"((Neg :right) ((Var :right) "{x ↦ 3}(x) = 3" --- "{x ↦ 3} ⊢ x ⇓ 3") "v = -3" --- "{x ↦ 3} ⊢ (- x) ⇓ -3")"#;
    let result = check(sexp);
    assert!(result.valid, "Exercise 1 solution should be valid: {:?}", result.diagnostics);
}

// ── Exercise 2 solution: {x ↦ 5} ⊢ (let ([y 3]) (+ y x)) ⇓ 8 ──────

#[test]
fn test_exercise2_solution_valid() {
    let sexp = r#"((Let :right) ((Int :right) --- "{x ↦ 5} ⊢ 3 ⇓ 3") ((Add :right) ((Var :right) "{x ↦ 5, y ↦ 3}(y) = 3" --- "{x ↦ 5, y ↦ 3} ⊢ y ⇓ 3") ((Var :right) "{x ↦ 5, y ↦ 3}(x) = 5" --- "{x ↦ 5, y ↦ 3} ⊢ x ⇓ 5") "v = 3 + 5" --- "{x ↦ 5, y ↦ 3} ⊢ (+ y x) ⇓ 8") --- "{x ↦ 5} ⊢ (let ([y 3]) (+ y x)) ⇓ 8")"#;
    let result = check(sexp);
    assert!(result.valid, "Exercise 2 solution should be valid: {:?}", result.diagnostics);
}

// ── Exercise 3 solution: {} ⊢ ((λ (x) (+ x 1)) 5) ⇓ 6 ────────────

#[test]
fn test_exercise3_solution_valid() {
    let sexp = r#"((App :right) ((Lam :right) --- "{} ⊢ (λ (x) (+ x 1)) ⇓ ⟨λ (x) (+ x 1) , {}⟩") ((Int :right) --- "{} ⊢ 5 ⇓ 5") ((Add :right) ((Var :right) "{x ↦ 5}(x) = 5" --- "{x ↦ 5} ⊢ x ⇓ 5") ((Int :right) --- "{x ↦ 5} ⊢ 1 ⇓ 1") "v = 5 + 1" --- "{x ↦ 5} ⊢ (+ x 1) ⇓ 6") --- "{} ⊢ ((λ (x) (+ x 1)) 5) ⇓ 6")"#;
    let result = check(sexp);
    assert!(result.valid, "Exercise 3 solution should be valid: {:?}", result.diagnostics);
}

// ── Optional Exercise A: {} ⊢ (if0 (+ 1 (- 1)) 42 0) ⇓ 42 ────────

#[test]
fn test_opt_a_solution_valid() {
    let sexp = r#"(("If0-True" :right) ((Add :right) ((Int :right) --- "{} ⊢ 1 ⇓ 1") ((Neg :right) ((Int :right) --- "{} ⊢ 1 ⇓ 1") "v = -1" --- "{} ⊢ (- 1) ⇓ -1") "v = 1 + (-1)" --- "{} ⊢ (+ 1 (- 1)) ⇓ 0") ((Int :right) --- "{} ⊢ 42 ⇓ 42") --- "{} ⊢ (if0 (+ 1 (- 1)) 42 0) ⇓ 42")"#;
    let result = check(sexp);
    assert!(result.valid, "Opt A solution should be valid: {:?}", result.diagnostics);
}

// ── Optional Exercise B: {} ⊢ (let ([x 10]) ((λ (y) (+ x y)) 7)) ⇓ 17 ──

#[test]
fn test_opt_b_solution_valid() {
    let sexp = r#"((Let :right) ((Int :right) --- "{} ⊢ 10 ⇓ 10") ((App :right) ((Lam :right) --- "{x ↦ 10} ⊢ (λ (y) (+ x y)) ⇓ ⟨λ (y) (+ x y) , {x ↦ 10}⟩") ((Int :right) --- "{x ↦ 10} ⊢ 7 ⇓ 7") ((Add :right) ((Var :right) "{x ↦ 10, y ↦ 7}(x) = 10" --- "{x ↦ 10, y ↦ 7} ⊢ x ⇓ 10") ((Var :right) "{x ↦ 10, y ↦ 7}(y) = 7" --- "{x ↦ 10, y ↦ 7} ⊢ y ⇓ 7") "v = 10 + 7" --- "{x ↦ 10, y ↦ 7} ⊢ (+ x y) ⇓ 17") --- "{x ↦ 10} ⊢ ((λ (y) (+ x y)) 7) ⇓ 17") --- "{} ⊢ (let ([x 10]) ((λ (y) (+ x y)) 7)) ⇓ 17")"#;
    let result = check(sexp);
    assert!(result.valid, "Opt B solution should be valid: {:?}", result.diagnostics);
}

// ── Error cases ─────────────────────────────────────────────────────

#[test]
fn test_empty_tree_is_incomplete() {
    let sexp = r#"(--- "")"#;
    let result = check(sexp);
    assert!(!result.valid);
    assert!(!result.complete);
}

#[test]
fn test_unfilled_conclusion_is_incomplete() {
    let sexp = r#"(--- "{} ⊢ 3 ⇓ 3")"#;
    let result = check(sexp);
    assert!(!result.valid);
    // The root has a judgement but no rule
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Incomplete));
}

#[test]
fn test_wrong_rule_name() {
    let sexp = r#"((Foo :right) --- "{} ⊢ 3 ⇓ 3")"#;
    let result = check(sexp);
    assert!(!result.valid);
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Error && d.message.contains("Unknown rule")));
}

#[test]
fn test_wrong_rule_for_expr() {
    // Using Add rule for an integer literal
    let sexp = r#"((Add :right) --- "{} ⊢ 3 ⇓ 3")"#;
    let result = check(sexp);
    assert!(!result.valid);
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Error));
}

#[test]
fn test_wrong_arithmetic() {
    // 3 + 5 = 9 is wrong
    let sexp = r#"((Add :right) ((Int :right) --- "{} ⊢ 3 ⇓ 3") ((Int :right) --- "{} ⊢ 5 ⇓ 5") "v = 3 + 5" --- "{} ⊢ (+ 3 5) ⇓ 9")"#;
    let result = check(sexp);
    assert!(!result.valid);
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Error));
}

// ── WASM entry point test ──────────────────────────────────────────

#[test]
fn test_wasm_check_proof_json() {
    let sexp = r#"((Int :right) --- "{} ⊢ 3 ⇓ 3")"#;
    let json = proof_checker::check_proof(sexp, "big-step");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_check_proof_error() {
    let sexp = r#"((Foo :right) --- "{} ⊢ 3 ⇓ 3")"#;
    let json = proof_checker::check_proof(sexp, "big-step");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], false);
}
