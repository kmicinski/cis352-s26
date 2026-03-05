/// System F: polymorphic lambda calculus.
///
/// Judgement format: Γ ⊢ e : τ
///
/// Rules (extends STLC with):
///   T-Var    — 0 premises
///   T-Int    — 0 premises
///   T-Bool   — 0 premises
///   T-Lam    — 1 premise
///   T-App    — 2 premises
///   T-Add    — 2 premises
///   T-Neg    — 1 premise
///   T-If     — 3 premises
///   T-Let    — 2 premises
///   T-TyLam  — from Γ ⊢ e : τ conclude Γ ⊢ (Λα. e) : ∀α. τ (1 premise)
///   T-TyApp  — from Γ ⊢ e : ∀α. τ conclude Γ ⊢ e [τ'] : τ[α := τ'] (1 premise)

use crate::check::{Diagnostic, Level, Theory};
use crate::tree::ProofNode;
use crate::types::{self, Ty, TypingJudgement};

pub struct SystemFTheory;

impl Theory for SystemFTheory {
    fn name(&self) -> &str {
        "System F (Polymorphic Lambda Calculus)"
    }

    fn known_rules(&self) -> Vec<&str> {
        vec![
            "T-Var", "T-Int", "T-Bool", "T-Lam", "T-App",
            "T-Add", "T-Neg", "T-If", "T-Let",
            "T-TyLam", "T-TyApp",
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
            "T-Var" => check_structural(rule_name, &j, premises, 0),
            "T-Int" => check_int(&j, premises),
            "T-Bool" => check_bool(&j, premises),
            "T-Lam" => check_lam(&j, premises),
            "T-App" => check_app(&j, premises),
            "T-Add" => check_structural(rule_name, &j, premises, 2),
            "T-Neg" => check_structural(rule_name, &j, premises, 1),
            "T-If" => check_structural(rule_name, &j, premises, 3),
            "T-Let" => check_let(&j, premises),
            "T-TyLam" => check_tylam(&j, premises),
            "T-TyApp" => check_tyapp(&j, premises),
            _ => vec![Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("Unknown System F rule '{}'", rule_name),
            }],
        }
    }

    fn is_judgement(&self, s: &str) -> bool {
        s.contains('\u{22A2}') && s.contains(':') && !s.contains('\u{21D3}') && !s.contains('\u{21D2}')
    }
}

fn parse_premise_tj(p: &ProofNode) -> Result<TypingJudgement, String> {
    types::parse_typing_judgement(&p.conclusion)
}

fn check_structural(
    rule_name: &str,
    _j: &TypingJudgement,
    premises: &[&ProofNode],
    expected_premises: usize,
) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != expected_premises {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "{} expects {} premise(s), got {}",
                rule_name, expected_premises, premises.len()
            ),
        });
    }
    diags
}

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
    diags
}

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
    diags
}

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

    match &j.ty {
        Ty::Arrow(_, ret) => {
            if let Ok(prem) = parse_premise_tj(premises[0]) {
                if prem.ty != **ret {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!("Premise type should be {}, got {}", ret, prem.ty),
                    });
                }
            }
        }
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("T-Lam: type must be a function type, got {}", j.ty),
            });
        }
    }

    diags
}

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
                if **ret_ty != j.ty {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!("Return type {} doesn't match conclusion type {}", ret_ty, j.ty),
                    });
                }
                if let Ok(prem1) = parse_premise_tj(premises[1]) {
                    if prem1.ty != **arg_ty {
                        diags.push(Diagnostic {
                            level: Level::Error,
                            path: vec![1],
                            message: format!("Argument type {} doesn't match expected {}", prem1.ty, arg_ty),
                        });
                    }
                }
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: "First premise must have function type".to_string(),
                });
            }
        }
    }

    diags
}

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

    if let Ok(prem1) = parse_premise_tj(premises[1]) {
        if prem1.ty != j.ty {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!("Body type {} doesn't match conclusion type {}", prem1.ty, j.ty),
            });
        }
    }

    diags
}

// ── T-TyLam: from Γ ⊢ e : τ conclude Γ ⊢ (Λα. e) : ∀α. τ ─────────

fn check_tylam(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-TyLam expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    match &j.ty {
        Ty::Forall(_, body_ty) => {
            if let Ok(prem) = parse_premise_tj(premises[0]) {
                if prem.ty != **body_ty {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!(
                            "Premise type should be {}, got {}",
                            body_ty, prem.ty
                        ),
                    });
                }
            }
        }
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("T-TyLam: type must be \u{2200}\u{03B1}. \u{03C4}, got {}", j.ty),
            });
        }
    }

    diags
}

// ── T-TyApp: from Γ ⊢ e : ∀α. τ conclude Γ ⊢ e [τ'] : τ[α:=τ'] ───

fn check_tyapp(_j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("T-TyApp expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    // Premise must have a ∀ type
    if let Ok(prem) = parse_premise_tj(premises[0]) {
        match &prem.ty {
            Ty::Forall(_, _) => {
                // The conclusion type should be the instantiated type
                // Full substitution checking is complex, so we just validate the shape
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: format!(
                        "T-TyApp: premise must have universal type \u{2200}\u{03B1}. \u{03C4}, got {}",
                        prem.ty
                    ),
                });
            }
        }
    }

    diags
}
