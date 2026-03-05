/// Simply-typed lambda calculus with int and bool.
///
/// Judgement format: Γ ⊢ e : τ
///
/// Rules:
///   T-Var   — Γ ⊢ x : τ where (x : τ) ∈ Γ (0 premises)
///   T-Int   — Γ ⊢ n : int (0 premises)
///   T-Bool  — Γ ⊢ true/false : bool (0 premises)
///   T-Lam   — from Γ, x:τ₁ ⊢ e : τ₂ conclude Γ ⊢ (λ (x : τ₁) e) : τ₁ → τ₂ (1 premise)
///   T-App   — from Γ ⊢ e₁ : τ₁→τ₂ and Γ ⊢ e₂ : τ₁ conclude Γ ⊢ (e₁ e₂) : τ₂ (2 premises)
///   T-Add   — from Γ ⊢ e₁ : int and Γ ⊢ e₂ : int conclude Γ ⊢ (+ e₁ e₂) : int (2 premises)
///   T-Neg   — from Γ ⊢ e : int conclude Γ ⊢ (- e) : int (1 premise)
///   T-If    — from Γ ⊢ e₁ : int, Γ ⊢ e₂ : τ, Γ ⊢ e₃ : τ conclude Γ ⊢ (if0 e₁ e₂ e₃) : τ (3 premises)
///   T-Let   — from Γ ⊢ e₁ : τ₁, Γ,x:τ₁ ⊢ e₂ : τ₂ conclude Γ ⊢ (let ([x e₁]) e₂) : τ₂ (2 premises)

use crate::check::{Diagnostic, Level, Theory};
use crate::tree::ProofNode;
use crate::types::{self, Ty, TypingJudgement};

pub struct STLCTheory;

impl Theory for STLCTheory {
    fn name(&self) -> &str {
        "Simply-Typed Lambda Calculus"
    }

    fn known_rules(&self) -> Vec<&str> {
        vec![
            "T-Var", "T-Int", "T-Bool", "T-Lam", "T-App",
            "T-Add", "T-Neg", "T-If", "T-Let",
        ]
    }

    fn check_rule(
        &self,
        rule_name: &str,
        conclusion: &str,
        premises: &[&ProofNode],
    ) -> Vec<Diagnostic> {
        let j = match types::parse_typing_judgement(conclusion) {
            Ok(j) => j,
            Err(e) => {
                return vec![Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: format!("Can't parse conclusion as typing judgement: {}", e),
                }];
            }
        };

        match rule_name {
            "T-Var" => check_var(&j, premises),
            "T-Int" => check_int(&j, premises),
            "T-Bool" => check_bool(&j, premises),
            "T-Lam" => check_lam(&j, premises),
            "T-App" => check_app(&j, premises),
            "T-Add" => check_add(&j, premises),
            "T-Neg" => check_neg(&j, premises),
            "T-If" => check_if(&j, premises),
            "T-Let" => check_let(&j, premises),
            _ => vec![Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("Unknown STLC rule '{}'", rule_name),
            }],
        }
    }

    fn is_judgement(&self, s: &str) -> bool {
        // Contains ⊢ and : but not ⇓ (which is big-step) and not ⇒ (which is sequent)
        s.contains('\u{22A2}') && s.contains(':') && !s.contains('\u{21D3}') && !s.contains('\u{21D2}')
    }
}

fn parse_premise_tj(p: &ProofNode) -> Result<TypingJudgement, String> {
    types::parse_typing_judgement(&p.conclusion)
}

// ── T-Var ────────────────────────────────────────────────────────────

fn check_var(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Var expects 0 premises, got {}", premises.len()),
        });
    }

    // Check that (x : τ) ∈ Γ
    let found = j.context.iter().any(|(v, t)| *v == j.expr_str && *t == j.ty);
    if !found {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "T-Var: ({} : {}) not found in context",
                j.expr_str, j.ty
            ),
        });
    }

    diags
}

// ── T-Int ────────────────────────────────────────────────────────────

fn check_int(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Int expects 0 premises, got {}", premises.len()),
        });
    }

    if j.ty != Ty::Int {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Int: type must be int, got {}", j.ty),
        });
    }

    // Check expression is an integer literal
    if j.expr_str.parse::<i64>().is_err() {
        let trimmed = j.expr_str.trim_start_matches('\u{2212}').trim_start_matches('-');
        if trimmed.parse::<u64>().is_err() {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("T-Int: expression '{}' is not an integer literal", j.expr_str),
            });
        }
    }

    diags
}

// ── T-Bool ───────────────────────────────────────────────────────────

fn check_bool(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Bool expects 0 premises, got {}", premises.len()),
        });
    }

    if j.ty != Ty::Bool {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Bool: type must be bool, got {}", j.ty),
        });
    }

    let e = j.expr_str.trim();
    if e != "true" && e != "false" && e != "#t" && e != "#f" {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Bool: expression '{}' is not a boolean literal", j.expr_str),
        });
    }

    diags
}

// ── T-Lam ────────────────────────────────────────────────────────────

fn check_lam(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Lam expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    // Type must be an arrow
    match &j.ty {
        Ty::Arrow(_, _) => {}
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("T-Lam: type must be \u{03C4}\u{2081} \u{2192} \u{03C4}\u{2082}, got {}", j.ty),
            });
        }
    }

    // Check premise parses as a valid typing judgement
    if let Ok(prem) = parse_premise_tj(premises[0]) {
        // Premise type should be the return type
        if let Ty::Arrow(_, ret) = &j.ty {
            if prem.ty != **ret {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: format!(
                        "Premise type should be {}, got {}",
                        ret, prem.ty
                    ),
                });
            }
        }
    }

    diags
}

// ── T-App ────────────────────────────────────────────────────────────

fn check_app(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-App expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    if let Ok(prem0) = parse_premise_tj(premises[0]) {
        match &prem0.ty {
            Ty::Arrow(arg_ty, ret_ty) => {
                // Return type should match conclusion type
                if **ret_ty != j.ty {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!(
                            "Function return type {} doesn't match conclusion type {}",
                            ret_ty, j.ty
                        ),
                    });
                }
                // Second premise type should match argument type
                if let Ok(prem1) = parse_premise_tj(premises[1]) {
                    if prem1.ty != **arg_ty {
                        diags.push(Diagnostic {
                            level: Level::Error,
                            path: vec![1],
                            message: format!(
                                "Argument type {} doesn't match expected {}",
                                prem1.ty, arg_ty
                            ),
                        });
                    }
                }
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: "First premise of T-App must have function type".to_string(),
                });
            }
        }
    }

    diags
}

// ── T-Add ────────────────────────────────────────────────────────────

fn check_add(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Add expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    if j.ty != Ty::Int {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Add: result type must be int, got {}", j.ty),
        });
    }

    for i in 0..2 {
        if let Ok(prem) = parse_premise_tj(premises[i]) {
            if prem.ty != Ty::Int {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!("T-Add: operand type must be int, got {}", prem.ty),
                });
            }
        }
    }

    diags
}

// ── T-Neg ────────────────────────────────────────────────────────────

fn check_neg(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Neg expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    if j.ty != Ty::Int {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Neg: result type must be int, got {}", j.ty),
        });
    }

    if let Ok(prem) = parse_premise_tj(premises[0]) {
        if prem.ty != Ty::Int {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("T-Neg: operand type must be int, got {}", prem.ty),
            });
        }
    }

    diags
}

// ── T-If ─────────────────────────────────────────────────────────────

fn check_if(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 3 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-If expects 3 premises, got {}", premises.len()),
        });
        return diags;
    }

    // First premise: guard must be int (for if0)
    if let Ok(prem0) = parse_premise_tj(premises[0]) {
        if prem0.ty != Ty::Int {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("T-If: guard type must be int, got {}", prem0.ty),
            });
        }
    }

    // Second and third premises must have the same type as the conclusion
    for i in 1..3 {
        if let Ok(prem) = parse_premise_tj(premises[i]) {
            if prem.ty != j.ty {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!(
                        "T-If: branch type {} doesn't match conclusion type {}",
                        prem.ty, j.ty
                    ),
                });
            }
        }
    }

    diags
}

// ── T-Let ────────────────────────────────────────────────────────────

fn check_let(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-Let expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    // Second premise type should match conclusion type
    if let Ok(prem1) = parse_premise_tj(premises[1]) {
        if prem1.ty != j.ty {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!(
                    "T-Let: body type {} doesn't match conclusion type {}",
                    prem1.ty, j.ty
                ),
            });
        }
    }

    diags
}
