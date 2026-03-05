use proof_checker::check::{check_tree, Level};
use proof_checker::sexp::parse_proof_sexp;

// ══════════════════════════════════════════════════════════════════════
// Helpers
// ══════════════════════════════════════════════════════════════════════

fn check_bigstep(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    check_tree(&node, &proof_checker::bigstep::BigStepTheory)
}

fn check_smallstep(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    check_tree(&node, &proof_checker::smallstep::SmallStepTheory)
}

fn check_g3ip(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    check_tree(&node, &proof_checker::g3ip::G3ipTheory)
}

fn check_propnd(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    check_tree(&node, &proof_checker::propnd::PropNDTheory)
}

fn check_fond(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    check_tree(&node, &proof_checker::fond::FONDTheory)
}

fn check_stlc(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    check_tree(&node, &proof_checker::stlc::STLCTheory)
}

fn check_systemf(sexp: &str) -> proof_checker::check::CheckResult {
    let node = parse_proof_sexp(sexp).expect("Failed to parse S-expression");
    check_tree(&node, &proof_checker::systemf::SystemFTheory)
}

// ══════════════════════════════════════════════════════════════════════
// Big-step examples (from playground)
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_bigstep_add() {
    let sexp = "((Add :right) ((Int :right) --- \"{} ⊢ 3 ⇓ 3\") ((Int :right) --- \"{} ⊢ 5 ⇓ 5\") \"v = 3 + 5\" --- \"{} ⊢ (+ 3 5) ⇓ 8\")";
    let result = check_bigstep(sexp);
    assert!(result.valid, "bs-add should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_bigstep_neg() {
    let sexp = "((Neg :right) ((Var :right) \"{x ↦ 3}(x) = 3\" --- \"{x ↦ 3} ⊢ x ⇓ 3\") \"v = -3\" --- \"{x ↦ 3} ⊢ (- x) ⇓ -3\")";
    let result = check_bigstep(sexp);
    assert!(result.valid, "bs-neg should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_bigstep_app() {
    let sexp = "((App :right) ((Lam :right) --- \"{} ⊢ (λ (x) (+ x 1)) ⇓ ⟨λ (x) (+ x 1) , {}⟩\") ((Int :right) --- \"{} ⊢ 5 ⇓ 5\") ((Add :right) ((Var :right) \"{x ↦ 5}(x) = 5\" --- \"{x ↦ 5} ⊢ x ⇓ 5\") ((Int :right) --- \"{x ↦ 5} ⊢ 1 ⇓ 1\") \"v = 5 + 1\" --- \"{x ↦ 5} ⊢ (+ x 1) ⇓ 6\") --- \"{} ⊢ ((λ (x) (+ x 1)) 5) ⇓ 6\")";
    let result = check_bigstep(sexp);
    assert!(result.valid, "bs-app should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_bigstep_if0() {
    let sexp = "((\"If0-True\" :right) ((Add :right) ((Int :right) --- \"{} ⊢ 1 ⇓ 1\") ((Neg :right) ((Int :right) --- \"{} ⊢ 1 ⇓ 1\") \"v = -1\" --- \"{} ⊢ (- 1) ⇓ -1\") \"v = 1 + (-1)\" --- \"{} ⊢ (+ 1 (- 1)) ⇓ 0\") ((Int :right) --- \"{} ⊢ 42 ⇓ 42\") --- \"{} ⊢ (if0 (+ 1 (- 1)) 42 0) ⇓ 42\")";
    let result = check_bigstep(sexp);
    assert!(result.valid, "bs-if0 should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_bigstep_let() {
    let sexp = "((Let :right) ((Int :right) --- \"{} ⊢ 10 ⇓ 10\") ((App :right) ((Lam :right) --- \"{x ↦ 10} ⊢ (λ (y) (+ x y)) ⇓ ⟨λ (y) (+ x y) , {x ↦ 10}⟩\") ((Int :right) --- \"{x ↦ 10} ⊢ 7 ⇓ 7\") ((Add :right) ((Var :right) \"{x ↦ 10, y ↦ 7}(x) = 10\" --- \"{x ↦ 10, y ↦ 7} ⊢ x ⇓ 10\") ((Var :right) \"{x ↦ 10, y ↦ 7}(y) = 7\" --- \"{x ↦ 10, y ↦ 7} ⊢ y ⇓ 7\") \"v = 10 + 7\" --- \"{x ↦ 10, y ↦ 7} ⊢ (+ x y) ⇓ 17\") --- \"{x ↦ 10} ⊢ ((λ (y) (+ x y)) 7) ⇓ 17\") --- \"{} ⊢ (let ([x 10]) ((λ (y) (+ x y)) 7)) ⇓ 17\")";
    let result = check_bigstep(sexp);
    assert!(result.valid, "bs-let should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Small-step examples (from playground)
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_smallstep_add() {
    let sexp = "((Add :right) --- \"(+ 3 5) ⟶ 8\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "ss-add should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_smallstep_add_nested() {
    let sexp = "((\"Add-L\" :right) ((Add :right) --- \"(+ 1 2) ⟶ 3\") --- \"(+ (+ 1 2) 5) ⟶ (+ 3 5)\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "ss-add-nested should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_smallstep_beta() {
    let sexp = "((Beta :right) --- \"((λ (x) x) 5) ⟶ 5\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "ss-beta should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_smallstep_neg() {
    let sexp = "((\"Neg-Step\" :right) ((Add :right) --- \"(+ 1 2) ⟶ 3\") --- \"(- (+ 1 2)) ⟶ (- 3)\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "ss-neg should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_smallstep_if0() {
    let sexp = "((\"If0-True\" :right) --- \"(if0 0 1 2) ⟶ 1\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "ss-if0 should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// G3ip sequent calculus examples (from playground)
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_g3ip_identity() {
    let sexp = "((Ax :right) --- \"P ⇒ P\")";
    let result = check_g3ip(sexp);
    assert!(result.valid, "g3-id should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_g3ip_and_swap() {
    let sexp = "((\"∧R\" :right) ((\"∧L\" :right) ((Ax :right) --- \"P, Q ⇒ Q\") --- \"P ∧ Q ⇒ Q\") ((\"∧L\" :right) ((Ax :right) --- \"P, Q ⇒ P\") --- \"P ∧ Q ⇒ P\") --- \"P ∧ Q ⇒ Q ∧ P\")";
    let result = check_g3ip(sexp);
    assert!(result.valid, "g3-swap should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_g3ip_or_comm() {
    let sexp = "((\"∨L\" :right) ((\"∨R₂\" :right) ((Ax :right) --- \"P ⇒ P\") --- \"P ⇒ Q ∨ P\") ((\"∨R₁\" :right) ((Ax :right) --- \"Q ⇒ Q\") --- \"Q ⇒ Q ∨ P\") --- \"P ∨ Q ⇒ Q ∨ P\")";
    let result = check_g3ip(sexp);
    assert!(result.valid, "g3-or-comm should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_g3ip_modus_ponens() {
    let sexp = "((\"→L\" :right) ((Ax :right) --- \"P, P → Q ⇒ P\") ((Ax :right) --- \"Q, P ⇒ Q\") --- \"P, P → Q ⇒ Q\")";
    let result = check_g3ip(sexp);
    assert!(result.valid, "g3-mp should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_g3ip_imp_identity() {
    let sexp = "((\"→R\" :right) ((Ax :right) --- \"P ⇒ P\") --- \"⇒ P → P\")";
    let result = check_g3ip(sexp);
    assert!(result.valid, "g3-imp-id should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Propositional natural deduction examples (from playground)
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_propnd_modus_ponens() {
    let sexp = "((\"→E\" :right) ((Ax :right) --- \"P, P → Q ⊢ P → Q\") ((Ax :right) --- \"P, P → Q ⊢ P\") --- \"P, P → Q ⊢ Q\")";
    let result = check_propnd(sexp);
    assert!(result.valid, "nd-mp should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_propnd_and_intro() {
    let sexp = "((\"∧I\" :right) ((Ax :right) --- \"P, Q ⊢ P\") ((Ax :right) --- \"P, Q ⊢ Q\") --- \"P, Q ⊢ P ∧ Q\")";
    let result = check_propnd(sexp);
    assert!(result.valid, "nd-and-i should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_propnd_or_elim() {
    let sexp = "((\"∨E\" :right) ((Ax :right) --- \"P ∨ Q, P → R, Q → R ⊢ P ∨ Q\") ((\"→E\" :right) ((Ax :right) --- \"P ∨ Q, P → R, Q → R, P ⊢ P → R\") ((Ax :right) --- \"P ∨ Q, P → R, Q → R, P ⊢ P\") --- \"P ∨ Q, P → R, Q → R, P ⊢ R\") ((\"→E\" :right) ((Ax :right) --- \"P ∨ Q, P → R, Q → R, Q ⊢ Q → R\") ((Ax :right) --- \"P ∨ Q, P → R, Q → R, Q ⊢ Q\") --- \"P ∨ Q, P → R, Q → R, Q ⊢ R\") --- \"P ∨ Q, P → R, Q → R ⊢ R\")";
    let result = check_propnd(sexp);
    assert!(result.valid, "nd-or-e should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_propnd_imp_identity() {
    let sexp = "((\"→I\" :right) ((Ax :right) --- \"P ⊢ P\") --- \"⊢ P → P\")";
    let result = check_propnd(sexp);
    assert!(result.valid, "nd-imp-id should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_propnd_contrapositive() {
    let sexp = "((\"¬I\" :right) ((\"¬E\" :right) ((Ax :right) --- \"P, P → Q, ¬Q ⊢ ¬Q\") ((\"→E\" :right) ((Ax :right) --- \"P, P → Q, ¬Q ⊢ P → Q\") ((Ax :right) --- \"P, P → Q, ¬Q ⊢ P\") --- \"P, P → Q, ¬Q ⊢ Q\") --- \"P, P → Q, ¬Q ⊢ ⊥\") --- \"P → Q, ¬Q ⊢ ¬P\")";
    let result = check_propnd(sexp);
    assert!(result.valid, "nd-contra should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// First-order natural deduction examples (from playground)
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_fond_forall_intro() {
    let sexp = "((\"∀I\" :right) ((\"→I\" :right) ((Ax :right) --- \"P(x) ⊢ P(x)\") --- \"⊢ P(x) → P(x)\") --- \"⊢ ∀x.P(x) → P(x)\")";
    let result = check_fond(sexp);
    assert!(result.valid, "fo-forall-i should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_fond_forall_elim() {
    let sexp = "((\"∀E\" :right) ((Ax :right) --- \"∀x.P(x) ⊢ ∀x.P(x)\") --- \"∀x.P(x) ⊢ P(a)\")";
    let result = check_fond(sexp);
    assert!(result.valid, "fo-forall-e should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_fond_exists_intro() {
    let sexp = "((\"∃I\" :right) ((Ax :right) --- \"P(a) ⊢ P(a)\") --- \"P(a) ⊢ ∃x.P(x)\")";
    let result = check_fond(sexp);
    assert!(result.valid, "fo-exists-i should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_fond_exists_elim() {
    let sexp = "((\"∃E\" :right) ((Ax :right) --- \"∃x.P(x), ∀x.P(x) → Q ⊢ ∃x.P(x)\") ((\"→E\" :right) ((\"∀E\" :right) ((Ax :right) --- \"∃x.P(x), ∀x.P(x) → Q, P(x) ⊢ ∀x.P(x) → Q\") --- \"∃x.P(x), ∀x.P(x) → Q, P(x) ⊢ P(x) → Q\") ((Ax :right) --- \"∃x.P(x), ∀x.P(x) → Q, P(x) ⊢ P(x)\") --- \"∃x.P(x), ∀x.P(x) → Q, P(x) ⊢ Q\") --- \"∃x.P(x), ∀x.P(x) → Q ⊢ Q\")";
    let result = check_fond(sexp);
    assert!(result.valid, "fo-exists-e should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_fond_mixed() {
    let sexp = "((\"→E\" :right) ((\"∀E\" :right) ((Ax :right) --- \"∀x.P(x) → Q(x), P(a) ⊢ ∀x.P(x) → Q(x)\") --- \"∀x.P(x) → Q(x), P(a) ⊢ P(a) → Q(a)\") ((Ax :right) --- \"∀x.P(x) → Q(x), P(a) ⊢ P(a)\") --- \"∀x.P(x) → Q(x), P(a) ⊢ Q(a)\")";
    let result = check_fond(sexp);
    assert!(result.valid, "fo-mixed should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// STLC examples (from playground)
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_stlc_identity() {
    let sexp = "((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : int ⊢ x : int\") --- \"⊢ (λ (x : int) x) : int → int\")";
    let result = check_stlc(sexp);
    assert!(result.valid, "stlc-id should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_stlc_const() {
    let sexp = "((\"T-Lam\" :right) ((\"T-Int\" :right) --- \"x : int ⊢ 42 : int\") --- \"⊢ (λ (x : int) 42) : int → int\")";
    let result = check_stlc(sexp);
    assert!(result.valid, "stlc-const should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_stlc_app() {
    let sexp = "((\"T-App\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : int ⊢ x : int\") --- \"⊢ (λ (x : int) x) : int → int\") ((\"T-Int\" :right) --- \"⊢ 5 : int\") --- \"⊢ ((λ (x : int) x) 5) : int\")";
    let result = check_stlc(sexp);
    assert!(result.valid, "stlc-app should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_stlc_add() {
    let sexp = "((\"T-Add\" :right) ((\"T-Var\" :right) --- \"x : int ⊢ x : int\") ((\"T-Int\" :right) --- \"x : int ⊢ 1 : int\") --- \"x : int ⊢ (+ x 1) : int\")";
    let result = check_stlc(sexp);
    assert!(result.valid, "stlc-add should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_stlc_let() {
    let sexp = "((\"T-Let\" :right) ((\"T-Int\" :right) --- \"⊢ 5 : int\") ((\"T-Add\" :right) ((\"T-Var\" :right) --- \"x : int ⊢ x : int\") ((\"T-Int\" :right) --- \"x : int ⊢ 1 : int\") --- \"x : int ⊢ (+ x 1) : int\") --- \"⊢ (let ([x 5]) (+ x 1)) : int\")";
    let result = check_stlc(sexp);
    assert!(result.valid, "stlc-let should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// System F examples (from playground)
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_systemf_poly_id() {
    let sexp = "((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : α ⊢ x : α\") --- \"⊢ (λ (x : α) x) : α → α\") --- \"⊢ (Λα. λ (x : α) x) : ∀α. α → α\")";
    let result = check_systemf(sexp);
    assert!(result.valid, "sf-poly-id should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_systemf_tyapp() {
    let sexp = "((\"T-TyApp\" :right) ((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : α ⊢ x : α\") --- \"⊢ (λ (x : α) x) : α → α\") --- \"⊢ (Λα. λ (x : α) x) : ∀α. α → α\") --- \"⊢ (Λα. λ (x : α) x) [int] : int → int\")";
    let result = check_systemf(sexp);
    assert!(result.valid, "sf-tyapp should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_systemf_const() {
    let sexp = "((\"T-TyLam\" :right) ((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : α, y : β ⊢ x : α\") --- \"x : α ⊢ (λ (y : β) x) : β → α\") --- \"⊢ (λ (x : α) (λ (y : β) x)) : α → β → α\") --- \"⊢ (Λβ. λ (x : α) (λ (y : β) x)) : ∀β. α → β → α\") --- \"⊢ (Λα. Λβ. λ (x : α) (λ (y : β) x)) : ∀α. ∀β. α → β → α\")";
    let result = check_systemf(sexp);
    assert!(result.valid, "sf-const should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_systemf_apply_id() {
    let sexp = "((\"T-App\" :right) ((\"T-TyApp\" :right) ((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : α ⊢ x : α\") --- \"⊢ (λ (x : α) x) : α → α\") --- \"⊢ (Λα. λ (x : α) x) : ∀α. α → α\") --- \"⊢ (Λα. λ (x : α) x) [int] : int → int\") ((\"T-Int\" :right) --- \"⊢ 5 : int\") --- \"⊢ ((Λα. λ (x : α) x) [int] 5) : int\")";
    let result = check_systemf(sexp);
    assert!(result.valid, "sf-apply-id should be valid: {:?}", result.diagnostics);
}

#[test]
fn test_systemf_neg() {
    let sexp = "((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Neg\" :right) ((\"T-Int\" :right) --- \"x : α ⊢ 1 : int\") --- \"x : α ⊢ (- 1) : int\") --- \"⊢ (λ (x : α) (- 1)) : α → int\") --- \"⊢ (Λα. λ (x : α) (- 1)) : ∀α. α → int\")";
    let result = check_systemf(sexp);
    assert!(result.valid, "sf-neg should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Big-step error cases
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_bigstep_empty_tree_is_incomplete() {
    let sexp = "(--- \"\")";
    let result = check_bigstep(sexp);
    assert!(!result.valid);
    assert!(!result.complete);
}

#[test]
fn test_bigstep_unfilled_conclusion_is_incomplete() {
    let sexp = "(--- \"{} ⊢ 3 ⇓ 3\")";
    let result = check_bigstep(sexp);
    assert!(!result.valid);
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Incomplete));
}

#[test]
fn test_bigstep_wrong_rule_name() {
    let sexp = "((Foo :right) --- \"{} ⊢ 3 ⇓ 3\")";
    let result = check_bigstep(sexp);
    assert!(!result.valid);
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Error && d.message.contains("Unknown rule")));
}

#[test]
fn test_bigstep_wrong_rule_for_expr() {
    let sexp = "((Add :right) --- \"{} ⊢ 3 ⇓ 3\")";
    let result = check_bigstep(sexp);
    assert!(!result.valid);
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Error));
}

#[test]
fn test_bigstep_wrong_arithmetic() {
    let sexp = "((Add :right) ((Int :right) --- \"{} ⊢ 3 ⇓ 3\") ((Int :right) --- \"{} ⊢ 5 ⇓ 5\") \"v = 3 + 5\" --- \"{} ⊢ (+ 3 5) ⇓ 9\")";
    let result = check_bigstep(sexp);
    assert!(!result.valid);
    assert!(result.diagnostics.iter().any(|d| d.level == Level::Error));
}

// ══════════════════════════════════════════════════════════════════════
// WASM entry point tests
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_wasm_check_proof_json() {
    let sexp = "((Int :right) --- \"{} ⊢ 3 ⇓ 3\")";
    let json = proof_checker::check_proof(sexp, "big-step");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_check_proof_error() {
    let sexp = "((Foo :right) --- \"{} ⊢ 3 ⇓ 3\")";
    let json = proof_checker::check_proof(sexp, "big-step");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], false);
}

#[test]
fn test_wasm_check_proof_g3ip() {
    let sexp = "((Ax :right) --- \"P ⇒ P\")";
    let json = proof_checker::check_proof(sexp, "g3ip");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_check_proof_propnd() {
    let sexp = "((\"→I\" :right) ((Ax :right) --- \"P ⊢ P\") --- \"⊢ P → P\")";
    let json = proof_checker::check_proof(sexp, "propnd");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_check_proof_fond() {
    let sexp = "((\"∀I\" :right) ((\"→I\" :right) ((Ax :right) --- \"P(x) ⊢ P(x)\") --- \"⊢ P(x) → P(x)\") --- \"⊢ ∀x.P(x) → P(x)\")";
    let json = proof_checker::check_proof(sexp, "fond");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_check_proof_stlc() {
    let sexp = "((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : int ⊢ x : int\") --- \"⊢ (λ (x : int) x) : int → int\")";
    let json = proof_checker::check_proof(sexp, "stlc");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_check_proof_systemf() {
    let sexp = "((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : α ⊢ x : α\") --- \"⊢ (λ (x : α) x) : α → α\") --- \"⊢ (Λα. λ (x : α) x) : ∀α. α → α\")";
    let json = proof_checker::check_proof(sexp, "systemf");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_check_proof_smallstep() {
    let sexp = "((Add :right) --- \"(+ 3 5) ⟶ 8\")";
    let json = proof_checker::check_proof(sexp, "small-step");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], true);
}

#[test]
fn test_wasm_unknown_theory() {
    let sexp = "((Int :right) --- \"{} ⊢ 3 ⇓ 3\")";
    let json = proof_checker::check_proof(sexp, "nonexistent");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["valid"], false);
}

// ══════════════════════════════════════════════════════════════════════
// Generate premises WASM entry point tests
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_generate_premises_bigstep() {
    let json = proof_checker::generate_premises("{} ⊢ (+ 3 5) ⇓ 8", "Add", "big-step");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["ok"], true);
}

#[test]
fn test_generate_premises_g3ip() {
    let json = proof_checker::generate_premises("P ∧ Q ⇒ Q ∧ P", "∧R", "g3ip");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["ok"], true);
}

#[test]
fn test_generate_premises_propnd() {
    let json = proof_checker::generate_premises("⊢ P → P", "→I", "propnd");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["ok"], true);
}

#[test]
fn test_generate_premises_stlc() {
    let json = proof_checker::generate_premises("⊢ (λ (x : int) x) : int → int", "T-Lam", "stlc");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["ok"], true);
}

// ══════════════════════════════════════════════════════════════════════
// Applicable rules WASM entry point tests
// ══════════════════════════════════════════════════════════════════════

#[test]
fn test_applicable_rules_bigstep() {
    let json = proof_checker::applicable_rules("{} ⊢ 3 ⇓ 3", "big-step");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let map = parsed.as_object().unwrap();
    assert!(map.contains_key("Int"));
}

#[test]
fn test_applicable_rules_g3ip() {
    let json = proof_checker::applicable_rules("P ⇒ P", "g3ip");
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let map = parsed.as_object().unwrap();
    assert!(map.contains_key("Ax"));
}
