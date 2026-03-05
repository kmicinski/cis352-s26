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
    let sexp = "((\"T-TyApp\" :right) ((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : α ⊢ x : α\") --- \"⊢ (λ (x : α) x) : α → α\") --- \"⊢ (Λα. λ (x : α) x) : ∀α. α → α\") --- \"⊢ ((Λα. λ (x : α) x) [int]) : int → int\")";
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
    let sexp = "((\"T-App\" :right) ((\"T-TyApp\" :right) ((\"T-TyLam\" :right) ((\"T-Lam\" :right) ((\"T-Var\" :right) --- \"x : α ⊢ x : α\") --- \"⊢ (λ (x : α) x) : α → α\") --- \"⊢ (Λα. λ (x : α) x) : ∀α. α → α\") --- \"⊢ ((Λα. λ (x : α) x) [int]) : int → int\") ((\"T-Int\" :right) --- \"⊢ 5 : int\") --- \"⊢ (((Λα. λ (x : α) x) [int]) 5) : int\")";
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
// Larger proofs — Big-step
// ══════════════════════════════════════════════════════════════════════

// {} ⊢ (if0 1 42 7) ⇓ 7  (If0-False with side condition 1 ≠ 0)
#[test]
fn test_bigstep_if0_false() {
    let sexp = "((\"If0-False\" :right) ((Int :right) --- \"{} ⊢ 1 ⇓ 1\") \"1 ≠ 0\" ((Int :right) --- \"{} ⊢ 7 ⇓ 7\") --- \"{} ⊢ (if0 1 42 7) ⇓ 7\")";
    let result = check_bigstep(sexp);
    assert!(result.valid, "If0-False should be valid: {:?}", result.diagnostics);
}

// {} ⊢ (let ([x (+ 2 3)]) (- x)) ⇓ -5  (Let with nested Add and Neg)
#[test]
fn test_bigstep_let_with_arithmetic() {
    let sexp = "((Let :right) ((Add :right) ((Int :right) --- \"{} ⊢ 2 ⇓ 2\") ((Int :right) --- \"{} ⊢ 3 ⇓ 3\") \"v = 2 + 3\" --- \"{} ⊢ (+ 2 3) ⇓ 5\") ((Neg :right) ((Var :right) \"{x ↦ 5}(x) = 5\" --- \"{x ↦ 5} ⊢ x ⇓ 5\") \"v = -5\" --- \"{x ↦ 5} ⊢ (- x) ⇓ -5\") --- \"{} ⊢ (let ([x (+ 2 3)]) (- x)) ⇓ -5\")";
    let result = check_bigstep(sexp);
    assert!(result.valid, "Let with arithmetic should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Larger proofs — Small-step
// ══════════════════════════════════════════════════════════════════════

// (+ 3 (+ 1 2)) ⟶ (+ 3 3)  via Add-R (reduces right operand when left is a value)
#[test]
fn test_smallstep_add_r() {
    let sexp = "((\"Add-R\" :right) ((Add :right) --- \"(+ 1 2) ⟶ 3\") --- \"(+ 3 (+ 1 2)) ⟶ (+ 3 3)\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "Add-R should be valid: {:?}", result.diagnostics);
}

// (if0 0 1 2) ⟶ 1  is already tested; add If0-False: (if0 1 42 7) ⟶ 7
#[test]
fn test_smallstep_if0_false() {
    let sexp = "((\"If0-False\" :right) --- \"(if0 1 42 7) ⟶ 7\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "If0-False should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Larger proofs — G3ip (Hypothetical Syllogism + Distribution)
// ══════════════════════════════════════════════════════════════════════

// ⇒ (P → Q) → (Q → R) → P → R   (hypothetical syllogism / transitivity)
// Uses →R three times, then →L twice (on P→Q, then Q→R), closed by Ax.
#[test]
fn test_g3ip_hypothetical_syllogism() {
    let sexp = concat!(
        "((\"→R\" :right) ",
          "((\"→R\" :right) ",
            "((\"→R\" :right) ",
              "((\"→L\" :right) ",
                "((Ax :right) --- \"P, P → Q, Q → R ⇒ P\") ",
                "((\"→L\" :right) ",
                  "((Ax :right) --- \"Q, P, Q → R ⇒ Q\") ",
                  "((Ax :right) --- \"R, Q, P ⇒ R\") ",
                  "--- \"Q, P, Q → R ⇒ R\") ",
                "--- \"P, P → Q, Q → R ⇒ R\") ",
              "--- \"P → Q, Q → R ⇒ P → R\") ",
            "--- \"P → Q ⇒ (Q → R) → P → R\") ",
          "--- \"⇒ (P → Q) → (Q → R) → P → R\")"
    );
    let result = check_g3ip(sexp);
    assert!(result.valid, "Hypothetical syllogism should be valid: {:?}", result.diagnostics);
}

// P ∧ (Q ∨ R) ⇒ (P ∧ Q) ∨ (P ∧ R)   (distribution of ∧ over ∨)
// Uses ∧L, ∨L, ∨R₁, ∨R₂, ∧R, Ax.
#[test]
fn test_g3ip_distribution() {
    let sexp = concat!(
        "((\"∧L\" :right) ",
          "((\"∨L\" :right) ",
            "((\"∨R₁\" :right) ",
              "((\"∧R\" :right) ",
                "((Ax :right) --- \"Q, P ⇒ P\") ",
                "((Ax :right) --- \"Q, P ⇒ Q\") ",
                "--- \"Q, P ⇒ P ∧ Q\") ",
              "--- \"Q, P ⇒ (P ∧ Q) ∨ (P ∧ R)\") ",
            "((\"∨R₂\" :right) ",
              "((\"∧R\" :right) ",
                "((Ax :right) --- \"R, P ⇒ P\") ",
                "((Ax :right) --- \"R, P ⇒ R\") ",
                "--- \"R, P ⇒ P ∧ R\") ",
              "--- \"R, P ⇒ (P ∧ Q) ∨ (P ∧ R)\") ",
            "--- \"P, Q ∨ R ⇒ (P ∧ Q) ∨ (P ∧ R)\") ",
          "--- \"P ∧ (Q ∨ R) ⇒ (P ∧ Q) ∨ (P ∧ R)\")"
    );
    let result = check_g3ip(sexp);
    assert!(result.valid, "Distribution should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Larger proofs — PropND (S combinator + Conjunction commutativity)
// ══════════════════════════════════════════════════════════════════════

// ⊢ (P → Q → R) → (P → Q) → P → R   (S combinator / Frege's theorem)
// Uses →I three times, then →E twice with nested →E applications.
#[test]
fn test_propnd_s_combinator() {
    let sexp = concat!(
        "((\"→I\" :right) ",
          "((\"→I\" :right) ",
            "((\"→I\" :right) ",
              "((\"→E\" :right) ",
                "((\"→E\" :right) ",
                  "((Ax :right) --- \"P → Q → R, P → Q, P ⊢ P → Q → R\") ",
                  "((Ax :right) --- \"P → Q → R, P → Q, P ⊢ P\") ",
                  "--- \"P → Q → R, P → Q, P ⊢ Q → R\") ",
                "((\"→E\" :right) ",
                  "((Ax :right) --- \"P → Q → R, P → Q, P ⊢ P → Q\") ",
                  "((Ax :right) --- \"P → Q → R, P → Q, P ⊢ P\") ",
                  "--- \"P → Q → R, P → Q, P ⊢ Q\") ",
                "--- \"P → Q → R, P → Q, P ⊢ R\") ",
              "--- \"P → Q → R, P → Q ⊢ P → R\") ",
            "--- \"P → Q → R ⊢ (P → Q) → P → R\") ",
          "--- \"⊢ (P → Q → R) → (P → Q) → P → R\")"
    );
    let result = check_propnd(sexp);
    assert!(result.valid, "S combinator should be valid: {:?}", result.diagnostics);
}

// ⊢ (P ∧ Q) → (Q ∧ P)   (conjunction commutativity)
// Uses →I, ∧I, ∧E₁, ∧E₂, Ax.
#[test]
fn test_propnd_and_comm() {
    let sexp = concat!(
        "((\"→I\" :right) ",
          "((\"∧I\" :right) ",
            "((\"∧E₂\" :right) ",
              "((Ax :right) --- \"P ∧ Q ⊢ P ∧ Q\") ",
              "--- \"P ∧ Q ⊢ Q\") ",
            "((\"∧E₁\" :right) ",
              "((Ax :right) --- \"P ∧ Q ⊢ P ∧ Q\") ",
              "--- \"P ∧ Q ⊢ P\") ",
            "--- \"P ∧ Q ⊢ Q ∧ P\") ",
          "--- \"⊢ P ∧ Q → Q ∧ P\")"
    );
    let result = check_propnd(sexp);
    assert!(result.valid, "Conjunction commutativity should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Larger proofs — FOND (∀-distribution + ∀-∧ distribution)
// ══════════════════════════════════════════════════════════════════════

// ∀x.(P(x) → Q(x)) ⊢ (∀x.P(x)) → ∀x.Q(x)   (universal distributes over →)
// Uses →I, ∀I, →E, ∀E, Ax.
#[test]
fn test_fond_forall_distributes_over_imp() {
    let sexp = concat!(
        "((\"→I\" :right) ",
          "((\"∀I\" :right) ",
            "((\"→E\" :right) ",
              "((\"∀E\" :right) ",
                "((Ax :right) --- \"∀x.P(x) → Q(x), ∀x.P(x) ⊢ ∀x.P(x) → Q(x)\") ",
                "--- \"∀x.P(x) → Q(x), ∀x.P(x) ⊢ P(x) → Q(x)\") ",
              "((\"∀E\" :right) ",
                "((Ax :right) --- \"∀x.P(x) → Q(x), ∀x.P(x) ⊢ ∀x.P(x)\") ",
                "--- \"∀x.P(x) → Q(x), ∀x.P(x) ⊢ P(x)\") ",
              "--- \"∀x.P(x) → Q(x), ∀x.P(x) ⊢ Q(x)\") ",
            "--- \"∀x.P(x) → Q(x), ∀x.P(x) ⊢ ∀x.Q(x)\") ",
          "--- \"∀x.P(x) → Q(x) ⊢ (∀x.P(x)) → ∀x.Q(x)\")"
    );
    let result = check_fond(sexp);
    assert!(result.valid, "∀ distributes over → should be valid: {:?}", result.diagnostics);
}

// ⊢ (∀x.P(x) ∧ Q(x)) → (∀x.P(x)) ∧ (∀x.Q(x))   (universal distributes over ∧)
// Uses →I, ∧I, ∀I, ∧E₁, ∧E₂, ∀E, Ax.
#[test]
fn test_fond_forall_distributes_over_and() {
    let sexp = concat!(
        "((\"→I\" :right) ",
          "((\"∧I\" :right) ",
            "((\"∀I\" :right) ",
              "((\"∧E₁\" :right) ",
                "((\"∀E\" :right) ",
                  "((Ax :right) --- \"∀x.P(x) ∧ Q(x) ⊢ ∀x.P(x) ∧ Q(x)\") ",
                  "--- \"∀x.P(x) ∧ Q(x) ⊢ P(x) ∧ Q(x)\") ",
                "--- \"∀x.P(x) ∧ Q(x) ⊢ P(x)\") ",
              "--- \"∀x.P(x) ∧ Q(x) ⊢ ∀x.P(x)\") ",
            "((\"∀I\" :right) ",
              "((\"∧E₂\" :right) ",
                "((\"∀E\" :right) ",
                  "((Ax :right) --- \"∀x.P(x) ∧ Q(x) ⊢ ∀x.P(x) ∧ Q(x)\") ",
                  "--- \"∀x.P(x) ∧ Q(x) ⊢ P(x) ∧ Q(x)\") ",
                "--- \"∀x.P(x) ∧ Q(x) ⊢ Q(x)\") ",
              "--- \"∀x.P(x) ∧ Q(x) ⊢ ∀x.Q(x)\") ",
            "--- \"∀x.P(x) ∧ Q(x) ⊢ (∀x.P(x)) ∧ (∀x.Q(x))\") ",
          "--- \"⊢ (∀x.P(x) ∧ Q(x)) → (∀x.P(x)) ∧ (∀x.Q(x))\")"
    );
    let result = check_fond(sexp);
    assert!(result.valid, "∀ distributes over ∧ should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Larger proofs — STLC (higher-order function + curried addition)
// ══════════════════════════════════════════════════════════════════════

// ⊢ (λ (f : int → int) (f 5)) : (int → int) → int   (higher-order function)
#[test]
fn test_stlc_higher_order() {
    let sexp = concat!(
        "((\"T-Lam\" :right) ",
          "((\"T-App\" :right) ",
            "((\"T-Var\" :right) --- \"f : int → int ⊢ f : int → int\") ",
            "((\"T-Int\" :right) --- \"f : int → int ⊢ 5 : int\") ",
            "--- \"f : int → int ⊢ (f 5) : int\") ",
          "--- \"⊢ (λ (f : int → int) (f 5)) : (int → int) → int\")"
    );
    let result = check_stlc(sexp);
    assert!(result.valid, "Higher-order function should be valid: {:?}", result.diagnostics);
}

// ⊢ (λ (x : int) (λ (y : int) (+ x y))) : int → int → int   (curried addition)
#[test]
fn test_stlc_curried_add() {
    let sexp = concat!(
        "((\"T-Lam\" :right) ",
          "((\"T-Lam\" :right) ",
            "((\"T-Add\" :right) ",
              "((\"T-Var\" :right) --- \"x : int, y : int ⊢ x : int\") ",
              "((\"T-Var\" :right) --- \"x : int, y : int ⊢ y : int\") ",
              "--- \"x : int, y : int ⊢ (+ x y) : int\") ",
            "--- \"x : int ⊢ (λ (y : int) (+ x y)) : int → int\") ",
          "--- \"⊢ (λ (x : int) (λ (y : int) (+ x y))) : int → int → int\")"
    );
    let result = check_stlc(sexp);
    assert!(result.valid, "Curried addition should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Larger proofs — System F (polymorphic application + polymorphic if0)
// ══════════════════════════════════════════════════════════════════════

// ⊢ (Λα. Λβ. λ (f : α → β) (λ (x : α) (f x))) : ∀α. ∀β. (α → β) → α → β
// Polymorphic function application wrapper.
#[test]
fn test_systemf_poly_apply() {
    let sexp = concat!(
        "((\"T-TyLam\" :right) ",
          "((\"T-TyLam\" :right) ",
            "((\"T-Lam\" :right) ",
              "((\"T-Lam\" :right) ",
                "((\"T-App\" :right) ",
                  "((\"T-Var\" :right) --- \"f : α → β, x : α ⊢ f : α → β\") ",
                  "((\"T-Var\" :right) --- \"f : α → β, x : α ⊢ x : α\") ",
                  "--- \"f : α → β, x : α ⊢ (f x) : β\") ",
                "--- \"f : α → β ⊢ (λ (x : α) (f x)) : α → β\") ",
              "--- \"⊢ (λ (f : α → β) (λ (x : α) (f x))) : (α → β) → α → β\") ",
            "--- \"⊢ (Λβ. λ (f : α → β) (λ (x : α) (f x))) : ∀β. (α → β) → α → β\") ",
          "--- \"⊢ (Λα. Λβ. λ (f : α → β) (λ (x : α) (f x))) : ∀α. ∀β. (α → β) → α → β\")"
    );
    let result = check_systemf(sexp);
    assert!(result.valid, "Polymorphic apply should be valid: {:?}", result.diagnostics);
}

// ⊢ (Λα. λ (x : α) (λ (y : α) (if0 0 x y))) : ∀α. α → α → α
// Polymorphic if0 expression with type variables in branches.
#[test]
fn test_systemf_poly_if0() {
    let sexp = concat!(
        "((\"T-TyLam\" :right) ",
          "((\"T-Lam\" :right) ",
            "((\"T-Lam\" :right) ",
              "((\"T-If\" :right) ",
                "((\"T-Int\" :right) --- \"x : α, y : α ⊢ 0 : int\") ",
                "((\"T-Var\" :right) --- \"x : α, y : α ⊢ x : α\") ",
                "((\"T-Var\" :right) --- \"x : α, y : α ⊢ y : α\") ",
                "--- \"x : α, y : α ⊢ (if0 0 x y) : α\") ",
              "--- \"x : α ⊢ (λ (y : α) (if0 0 x y)) : α → α\") ",
            "--- \"⊢ (λ (x : α) (λ (y : α) (if0 0 x y))) : α → α → α\") ",
          "--- \"⊢ (Λα. λ (x : α) (λ (y : α) (if0 0 x y))) : ∀α. α → α → α\")"
    );
    let result = check_systemf(sexp);
    assert!(result.valid, "Polymorphic if0 should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Additional examples — Big-step (nested arithmetic + if0-false)
// ══════════════════════════════════════════════════════════════════════

// {} ⊢ (+ (- 3) (+ 4 5)) ⇓ 6
#[test]
fn test_bigstep_nested_arithmetic() {
    let sexp = concat!(
        "((Add :right) ",
          "((Neg :right) ((Int :right) --- \"{} ⊢ 3 ⇓ 3\") \"v = -3\" --- \"{} ⊢ (- 3) ⇓ -3\") ",
          "((Add :right) ((Int :right) --- \"{} ⊢ 4 ⇓ 4\") ((Int :right) --- \"{} ⊢ 5 ⇓ 5\") \"v = 4 + 5\" --- \"{} ⊢ (+ 4 5) ⇓ 9\") ",
          "\"v = (-3) + 9\" ",
          "--- \"{} ⊢ (+ (- 3) (+ 4 5)) ⇓ 6\")"
    );
    let result = check_bigstep(sexp);
    assert!(result.valid, "Nested arithmetic should be valid: {:?}", result.diagnostics);
}

// {} ⊢ (if0 1 42 (+ 2 3)) ⇓ 5
#[test]
fn test_bigstep_if0_false_with_add() {
    let sexp = concat!(
        "((\"If0-False\" :right) ",
          "((Int :right) --- \"{} ⊢ 1 ⇓ 1\") ",
          "\"1 ≠ 0\" ",
          "((Add :right) ((Int :right) --- \"{} ⊢ 2 ⇓ 2\") ((Int :right) --- \"{} ⊢ 3 ⇓ 3\") \"v = 2 + 3\" --- \"{} ⊢ (+ 2 3) ⇓ 5\") ",
          "--- \"{} ⊢ (if0 1 42 (+ 2 3)) ⇓ 5\")"
    );
    let result = check_bigstep(sexp);
    assert!(result.valid, "If0-false with add should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Additional examples — Small-step (Let-Step + Let)
// ══════════════════════════════════════════════════════════════════════

// (let ([x (+ 1 2)]) x) ⟶ (let ([x 3]) x)
#[test]
fn test_smallstep_let_step() {
    let sexp = "((\"Let-Step\" :right) ((Add :right) --- \"(+ 1 2) ⟶ 3\") --- \"(let ([x (+ 1 2)]) x) ⟶ (let ([x 3]) x)\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "Let-Step should be valid: {:?}", result.diagnostics);
}

// (let ([x 5]) x) ⟶ 5
#[test]
fn test_smallstep_let() {
    let sexp = "((Let :right) --- \"(let ([x 5]) x) ⟶ 5\")";
    let result = check_smallstep(sexp);
    assert!(result.valid, "Let should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Additional examples — G3ip (3 complex proofs)
// ══════════════════════════════════════════════════════════════════════

// ⇒ (P → Q → R) → P ∧ Q → R  (uncurrying)
#[test]
fn test_g3ip_uncurry() {
    let sexp = concat!(
        "((\"→R\" :right) ",
          "((\"→R\" :right) ",
            "((\"→L\" :right) ",
              "((\"∧L\" :right) ",
                "((Ax :right) --- \"P, Q, P → Q → R ⇒ P\") ",
                "--- \"P → Q → R, P ∧ Q ⇒ P\") ",
              "((\"→L\" :right) ",
                "((\"∧L\" :right) ",
                  "((Ax :right) --- \"Q → R, P, Q ⇒ Q\") ",
                  "--- \"Q → R, P ∧ Q ⇒ Q\") ",
                "((Ax :right) --- \"R, P ∧ Q ⇒ R\") ",
                "--- \"Q → R, P ∧ Q ⇒ R\") ",
              "--- \"P → Q → R, P ∧ Q ⇒ R\") ",
            "--- \"P → Q → R ⇒ P ∧ Q → R\") ",
          "--- \"⇒ (P → Q → R) → P ∧ Q → R\")"
    );
    let result = check_g3ip(sexp);
    assert!(result.valid, "Uncurrying should be valid: {:?}", result.diagnostics);
}

// ⇒ (P ∧ Q → R) → P → Q → R  (currying)
#[test]
fn test_g3ip_curry() {
    let sexp = concat!(
        "((\"→R\" :right) ",
          "((\"→R\" :right) ",
            "((\"→R\" :right) ",
              "((\"→L\" :right) ",
                "((\"∧R\" :right) ",
                  "((Ax :right) --- \"P ∧ Q → R, P, Q ⇒ P\") ",
                  "((Ax :right) --- \"P ∧ Q → R, P, Q ⇒ Q\") ",
                  "--- \"P ∧ Q → R, P, Q ⇒ P ∧ Q\") ",
                "((Ax :right) --- \"R, P, Q ⇒ R\") ",
                "--- \"P ∧ Q → R, P, Q ⇒ R\") ",
              "--- \"P ∧ Q → R, P ⇒ Q → R\") ",
            "--- \"P ∧ Q → R ⇒ P → Q → R\") ",
          "--- \"⇒ (P ∧ Q → R) → P → Q → R\")"
    );
    let result = check_g3ip(sexp);
    assert!(result.valid, "Currying should be valid: {:?}", result.diagnostics);
}

// ⇒ (P → R) → (Q → R) → P ∨ Q → R  (∨-elimination in sequent form)
#[test]
fn test_g3ip_or_elim_sequent() {
    let sexp = concat!(
        "((\"→R\" :right) ",
          "((\"→R\" :right) ",
            "((\"→R\" :right) ",
              "((\"∨L\" :right) ",
                "((\"→L\" :right) ",
                  "((Ax :right) --- \"P → R, Q → R, P ⇒ P\") ",
                  "((Ax :right) --- \"R, Q → R, P ⇒ R\") ",
                  "--- \"P → R, Q → R, P ⇒ R\") ",
                "((\"→L\" :right) ",
                  "((Ax :right) --- \"P → R, Q → R, Q ⇒ Q\") ",
                  "((Ax :right) --- \"R, P → R, Q ⇒ R\") ",
                  "--- \"P → R, Q → R, Q ⇒ R\") ",
                "--- \"P → R, Q → R, P ∨ Q ⇒ R\") ",
              "--- \"P → R, Q → R ⇒ P ∨ Q → R\") ",
            "--- \"P → R ⇒ (Q → R) → P ∨ Q → R\") ",
          "--- \"⇒ (P → R) → (Q → R) → P ∨ Q → R\")"
    );
    let result = check_g3ip(sexp);
    assert!(result.valid, "∨-elim sequent should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Additional examples — PropND (K combinator + ∧ commutativity)
// ══════════════════════════════════════════════════════════════════════

// ⊢ P → Q → P  (K combinator / weakening)
#[test]
fn test_propnd_k_combinator() {
    let sexp = concat!(
        "((\"→I\" :right) ",
          "((\"→I\" :right) ",
            "((Ax :right) --- \"P, Q ⊢ P\") ",
            "--- \"P ⊢ Q → P\") ",
          "--- \"⊢ P → Q → P\")"
    );
    let result = check_propnd(sexp);
    assert!(result.valid, "K combinator should be valid: {:?}", result.diagnostics);
}

// P ∧ Q ⊢ Q ∧ P  (commutativity of ∧)
#[test]
fn test_propnd_and_comm_judgement() {
    let sexp = concat!(
        "((\"∧I\" :right) ",
          "((\"∧E₂\" :right) ",
            "((Ax :right) --- \"P ∧ Q ⊢ P ∧ Q\") ",
            "--- \"P ∧ Q ⊢ Q\") ",
          "((\"∧E₁\" :right) ",
            "((Ax :right) --- \"P ∧ Q ⊢ P ∧ Q\") ",
            "--- \"P ∧ Q ⊢ P\") ",
          "--- \"P ∧ Q ⊢ Q ∧ P\")"
    );
    let result = check_propnd(sexp);
    assert!(result.valid, "∧ commutativity should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Additional examples — FOND (∀E + →E chain, ∀E wrapped in →I)
// ══════════════════════════════════════════════════════════════════════

// ∀x.P(x), P(a) → Q ⊢ Q  (∀E then →E)
#[test]
fn test_fond_forall_then_imp() {
    let sexp = concat!(
        "((\"→E\" :right) ",
          "((Ax :right) --- \"∀x.P(x), P(a) → Q ⊢ P(a) → Q\") ",
          "((\"∀E\" :right) ",
            "((Ax :right) --- \"∀x.P(x), P(a) → Q ⊢ ∀x.P(x)\") ",
            "--- \"∀x.P(x), P(a) → Q ⊢ P(a)\") ",
          "--- \"∀x.P(x), P(a) → Q ⊢ Q\")"
    );
    let result = check_fond(sexp);
    assert!(result.valid, "∀E then →E should be valid: {:?}", result.diagnostics);
}

// ⊢ (∀x.P(x)) → P(a)  (∀E wrapped in →I)
#[test]
fn test_fond_forall_to_instance() {
    let sexp = concat!(
        "((\"→I\" :right) ",
          "((\"∀E\" :right) ",
            "((Ax :right) --- \"∀x.P(x) ⊢ ∀x.P(x)\") ",
            "--- \"∀x.P(x) ⊢ P(a)\") ",
          "--- \"⊢ (∀x.P(x)) → P(a)\")"
    );
    let result = check_fond(sexp);
    assert!(result.valid, "∀E in →I should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Additional examples — STLC (3 impressively large proofs)
// ══════════════════════════════════════════════════════════════════════

// ⊢ (λ (f : int → int) (λ (x : int) (f (f x)))) : (int → int) → int → int
// Double application of a function.
#[test]
fn test_stlc_double_apply() {
    let sexp = concat!(
        "((\"T-Lam\" :right) ",
          "((\"T-Lam\" :right) ",
            "((\"T-App\" :right) ",
              "((\"T-Var\" :right) --- \"f : int → int, x : int ⊢ f : int → int\") ",
              "((\"T-App\" :right) ",
                "((\"T-Var\" :right) --- \"f : int → int, x : int ⊢ f : int → int\") ",
                "((\"T-Var\" :right) --- \"f : int → int, x : int ⊢ x : int\") ",
                "--- \"f : int → int, x : int ⊢ (f x) : int\") ",
              "--- \"f : int → int, x : int ⊢ (f (f x)) : int\") ",
            "--- \"f : int → int ⊢ (λ (x : int) (f (f x))) : int → int\") ",
          "--- \"⊢ (λ (f : int → int) (λ (x : int) (f (f x)))) : (int → int) → int → int\")"
    );
    let result = check_stlc(sexp);
    assert!(result.valid, "Double apply should be valid: {:?}", result.diagnostics);
}

// ⊢ (λ (f : int → int) (λ (g : int → int) (λ (x : int) (f (g x))))) : (int → int) → (int → int) → int → int
// Function composition.
#[test]
fn test_stlc_compose() {
    let sexp = concat!(
        "((\"T-Lam\" :right) ",
          "((\"T-Lam\" :right) ",
            "((\"T-Lam\" :right) ",
              "((\"T-App\" :right) ",
                "((\"T-Var\" :right) --- \"f : int → int, g : int → int, x : int ⊢ f : int → int\") ",
                "((\"T-App\" :right) ",
                  "((\"T-Var\" :right) --- \"f : int → int, g : int → int, x : int ⊢ g : int → int\") ",
                  "((\"T-Var\" :right) --- \"f : int → int, g : int → int, x : int ⊢ x : int\") ",
                  "--- \"f : int → int, g : int → int, x : int ⊢ (g x) : int\") ",
                "--- \"f : int → int, g : int → int, x : int ⊢ (f (g x)) : int\") ",
              "--- \"f : int → int, g : int → int ⊢ (λ (x : int) (f (g x))) : int → int\") ",
            "--- \"f : int → int ⊢ (λ (g : int → int) (λ (x : int) (f (g x)))) : (int → int) → int → int\") ",
          "--- \"⊢ (λ (f : int → int) (λ (g : int → int) (λ (x : int) (f (g x))))) : (int → int) → (int → int) → int → int\")"
    );
    let result = check_stlc(sexp);
    assert!(result.valid, "Compose should be valid: {:?}", result.diagnostics);
}

// ⊢ (let ([double (λ (x : int) (+ x x))]) (double 21)) : int
// Let with a doubling function.
#[test]
fn test_stlc_let_double() {
    let sexp = concat!(
        "((\"T-Let\" :right) ",
          "((\"T-Lam\" :right) ",
            "((\"T-Add\" :right) ",
              "((\"T-Var\" :right) --- \"x : int ⊢ x : int\") ",
              "((\"T-Var\" :right) --- \"x : int ⊢ x : int\") ",
              "--- \"x : int ⊢ (+ x x) : int\") ",
            "--- \"⊢ (λ (x : int) (+ x x)) : int → int\") ",
          "((\"T-App\" :right) ",
            "((\"T-Var\" :right) --- \"double : int → int ⊢ double : int → int\") ",
            "((\"T-Int\" :right) --- \"double : int → int ⊢ 21 : int\") ",
            "--- \"double : int → int ⊢ (double 21) : int\") ",
          "--- \"⊢ (let ([double (λ (x : int) (+ x x))]) (double 21)) : int\")"
    );
    let result = check_stlc(sexp);
    assert!(result.valid, "Let double should be valid: {:?}", result.diagnostics);
}

// ══════════════════════════════════════════════════════════════════════
// Additional examples — System F (Church true + let-poly)
// ══════════════════════════════════════════════════════════════════════

// ⊢ (Λα. λ (x : α) (λ (y : α) x)) : ∀α. α → α → α  (Church boolean true)
#[test]
fn test_systemf_church_true() {
    let sexp = concat!(
        "((\"T-TyLam\" :right) ",
          "((\"T-Lam\" :right) ",
            "((\"T-Lam\" :right) ",
              "((\"T-Var\" :right) --- \"x : α, y : α ⊢ x : α\") ",
              "--- \"x : α ⊢ (λ (y : α) x) : α → α\") ",
            "--- \"⊢ (λ (x : α) (λ (y : α) x)) : α → α → α\") ",
          "--- \"⊢ (Λα. λ (x : α) (λ (y : α) x)) : ∀α. α → α → α\")"
    );
    let result = check_systemf(sexp);
    assert!(result.valid, "Church true should be valid: {:?}", result.diagnostics);
}

// ⊢ (Λα. λ (x : α) (λ (y : α) y)) : ∀α. α → α → α  (Church boolean false)
#[test]
fn test_systemf_church_false() {
    let sexp = concat!(
        "((\"T-TyLam\" :right) ",
          "((\"T-Lam\" :right) ",
            "((\"T-Lam\" :right) ",
              "((\"T-Var\" :right) --- \"x : α, y : α ⊢ y : α\") ",
              "--- \"x : α ⊢ (λ (y : α) y) : α → α\") ",
            "--- \"⊢ (λ (x : α) (λ (y : α) y)) : α → α → α\") ",
          "--- \"⊢ (Λα. λ (x : α) (λ (y : α) y)) : ∀α. α → α → α\")"
    );
    let result = check_systemf(sexp);
    assert!(result.valid, "Church false should be valid: {:?}", result.diagnostics);
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
