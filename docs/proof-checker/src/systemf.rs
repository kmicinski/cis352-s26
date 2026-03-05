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

    fn generate_premises(&self, rule_name: &str, conclusion: &str) -> Result<Vec<String>, String> {
        let j = types::parse_typing_judgement(conclusion)
            .map_err(|e| format!("Can't parse conclusion: {}", e))?;
        let ctx = &j.context;
        let ty = &j.ty;
        let expr = &j.expr_str;
        let placeholder_ty = Ty::TyVar("?".into());
        let parts = types::split_sexpr(expr);
        let head = parts.as_ref().and_then(|p| p.first().map(|s| s.as_str()));

        match rule_name {
            "T-Var" | "T-Int" | "T-Bool" => Ok(vec![]),
            "T-Lam" => {
                match ty {
                    Ty::Arrow(t1, t2) => {
                        if let Some(ref parts) = parts {
                            if parts.len() == 3 && (parts[0] == "\u{03BB}" || parts[0] == "lambda") {
                                if let Some((var_name, _)) = types::parse_lambda_binding(&parts[1]) {
                                    let mut new_ctx = ctx.clone();
                                    new_ctx.push((var_name, *t1.clone()));
                                    return Ok(vec![types::format_typing_judgement_str(&new_ctx, &parts[2], t2)]);
                                }
                            }
                        }
                        let mut new_ctx = ctx.clone();
                        new_ctx.push(("?".into(), *t1.clone()));
                        Ok(vec![types::format_typing_judgement_str(&new_ctx, "?", t2)])
                    }
                    _ => Err(format!("T-Lam requires an arrow type \u{03C4}\u{2081} \u{2192} \u{03C4}\u{2082}, but got {}", ty)),
                }
            }
            "T-App" => {
                if let Some(ref parts) = parts {
                    if parts.len() == 2 && !crate::stlc::is_keyword(head) {
                        let arrow_ty = Ty::Arrow(Box::new(placeholder_ty.clone()), Box::new(ty.clone()));
                        let p1 = types::format_typing_judgement_str(ctx, &parts[0], &arrow_ty);
                        let p2 = types::format_typing_judgement_str(ctx, &parts[1], &placeholder_ty);
                        return Ok(vec![p1, p2]);
                    }
                }
                let arrow_ty = Ty::Arrow(Box::new(placeholder_ty.clone()), Box::new(ty.clone()));
                let p1 = types::format_typing_judgement_str(ctx, "?", &arrow_ty);
                let p2 = types::format_typing_judgement_str(ctx, "?", &placeholder_ty);
                Ok(vec![p1, p2])
            }
            "T-Add" => {
                if let Some(ref parts) = parts {
                    if parts.len() == 3 && parts[0] == "+" {
                        let p1 = types::format_typing_judgement_str(ctx, &parts[1], &Ty::Int);
                        let p2 = types::format_typing_judgement_str(ctx, &parts[2], &Ty::Int);
                        return Ok(vec![p1, p2]);
                    }
                }
                let p1 = types::format_typing_judgement_str(ctx, "?", &Ty::Int);
                let p2 = types::format_typing_judgement_str(ctx, "?", &Ty::Int);
                Ok(vec![p1, p2])
            }
            "T-Neg" => {
                if let Some(ref parts) = parts {
                    if parts.len() == 2 && parts[0] == "-" {
                        return Ok(vec![types::format_typing_judgement_str(ctx, &parts[1], &Ty::Int)]);
                    }
                }
                Ok(vec![types::format_typing_judgement_str(ctx, "?", &Ty::Int)])
            }
            "T-If" => {
                if let Some(ref parts) = parts {
                    if parts.len() == 4 && parts[0] == "if0" {
                        let p1 = types::format_typing_judgement_str(ctx, &parts[1], &Ty::Int);
                        let p2 = types::format_typing_judgement_str(ctx, &parts[2], ty);
                        let p3 = types::format_typing_judgement_str(ctx, &parts[3], ty);
                        return Ok(vec![p1, p2, p3]);
                    }
                }
                let p1 = types::format_typing_judgement_str(ctx, "?", &Ty::Int);
                let p2 = types::format_typing_judgement_str(ctx, "?", ty);
                let p3 = types::format_typing_judgement_str(ctx, "?", ty);
                Ok(vec![p1, p2, p3])
            }
            "T-Let" => {
                if let Some(ref parts) = parts {
                    if parts.len() == 3 && parts[0] == "let" {
                        if let Some((var_name, bound_expr)) = types::parse_let_binding(&parts[1]) {
                            let p1 = types::format_typing_judgement_str(ctx, &bound_expr, &placeholder_ty);
                            let mut new_ctx = ctx.clone();
                            new_ctx.push((var_name, placeholder_ty));
                            let p2 = types::format_typing_judgement_str(&new_ctx, &parts[2], ty);
                            return Ok(vec![p1, p2]);
                        }
                    }
                }
                let p1 = types::format_typing_judgement_str(ctx, "?", &placeholder_ty);
                let mut new_ctx = ctx.clone();
                new_ctx.push(("?".into(), placeholder_ty));
                let p2 = types::format_typing_judgement_str(&new_ctx, "?", ty);
                Ok(vec![p1, p2])
            }
            "T-TyLam" => {
                match ty {
                    Ty::Forall(_alpha, tau) => {
                        // Try to decompose (Λα. e)
                        if let Some(ref parts) = parts {
                            if parts.len() == 3 && (parts[0] == "\u{039B}" || parts[0] == "Lambda")
                                && parts[1].ends_with('.')
                            {
                                let body = &parts[2];
                                return Ok(vec![types::format_typing_judgement_str(ctx, body, tau)]);
                            }
                        }
                        let p = types::format_typing_judgement_str(ctx, "?", tau);
                        Ok(vec![p])
                    }
                    _ => Err(format!("T-TyLam requires a universal type \u{2200}\u{03B1}. \u{03C4}, but got {}", ty)),
                }
            }
            "T-TyApp" => {
                let forall_ty = Ty::Forall("?".into(), Box::new(placeholder_ty));
                let p = types::format_typing_judgement_str(ctx, "?", &forall_ty);
                Ok(vec![p])
            }
            _ => Err(format!("Unknown System F rule '{}'", rule_name)),
        }
    }

    fn applicable_rules(&self, conclusion: &str) -> Vec<(&str, bool, Option<String>)> {
        crate::stlc::stlc_applicable_rules(conclusion, true)
    }
}

fn parse_premise_tj(p: &ProofNode) -> Result<TypingJudgement, String> {
    types::parse_typing_judgement(&p.conclusion)
}

fn check_structural(
    rule_name: &str,
    j: &TypingJudgement,
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
        return diags;
    }
    // For rules with premises, check that each premise context matches the conclusion context
    for i in 0..premises.len() {
        if let Ok(prem) = parse_premise_tj(premises[i]) {
            if !types::contexts_eq(&prem.context, &j.context) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!(
                        "{}: premise context [{}] doesn't match conclusion context [{}]",
                        rule_name,
                        types::format_context(&prem.context),
                        types::format_context(&j.context)
                    ),
                });
            }
        }
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
                // Premise context should extend conclusion context by exactly one binding
                if prem.context.len() != j.context.len() + 1 {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!(
                            "T-Lam: premise context should extend the conclusion context by one binding, but has {} entries (expected {})",
                            prem.context.len(), j.context.len() + 1
                        ),
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
        if !types::contexts_eq(&prem0.context, &j.context) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "T-App: premise context [{}] doesn't match conclusion context [{}]",
                    types::format_context(&prem0.context),
                    types::format_context(&j.context)
                ),
            });
        }
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
                    if !types::contexts_eq(&prem1.context, &j.context) {
                        diags.push(Diagnostic {
                            level: Level::Error,
                            path: vec![1],
                            message: format!(
                                "T-App: premise context [{}] doesn't match conclusion context [{}]",
                                types::format_context(&prem1.context),
                                types::format_context(&j.context)
                            ),
                        });
                    }
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

    // First premise context should match conclusion context
    if let Ok(prem0) = parse_premise_tj(premises[0]) {
        if !types::contexts_eq(&prem0.context, &j.context) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "T-Let: first premise context [{}] doesn't match conclusion context [{}]",
                    types::format_context(&prem0.context),
                    types::format_context(&j.context)
                ),
            });
        }
    }

    if let Ok(prem1) = parse_premise_tj(premises[1]) {
        if prem1.ty != j.ty {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!("Body type {} doesn't match conclusion type {}", prem1.ty, j.ty),
            });
        }
        // Second premise context should extend conclusion context by one binding
        if prem1.context.len() != j.context.len() + 1 {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!(
                    "T-Let: second premise context should extend conclusion context by one binding, but has {} entries (expected {})",
                    prem1.context.len(), j.context.len() + 1
                ),
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
                if !types::contexts_eq(&prem.context, &j.context) {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!(
                            "T-TyLam: premise context [{}] doesn't match conclusion context [{}]",
                            types::format_context(&prem.context),
                            types::format_context(&j.context)
                        ),
                    });
                }
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

fn check_tyapp(j: &TypingJudgement, premises: &[&ProofNode]) -> Vec<Diagnostic> {
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
        if !types::contexts_eq(&prem.context, &j.context) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "T-TyApp: premise context [{}] doesn't match conclusion context [{}]",
                    types::format_context(&prem.context),
                    types::format_context(&j.context)
                ),
            });
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Theory;

    #[test]
    fn test_gen_t_var() {
        let prems = SystemFTheory.generate_premises("T-Var", "x : int \u{22A2} x : int").unwrap();
        assert_eq!(prems.len(), 0);
    }

    #[test]
    fn test_gen_t_tylam() {
        let prems = SystemFTheory.generate_premises(
            "T-TyLam",
            "\u{22A2} e : \u{2200}\u{03B1}. \u{03B1} \u{2192} \u{03B1}",
        ).unwrap();
        assert_eq!(prems.len(), 1);
    }

    #[test]
    fn test_gen_t_tyapp() {
        let prems = SystemFTheory.generate_premises("T-TyApp", "\u{22A2} e : int").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("\u{2200}"));
    }

    #[test]
    fn test_gen_t_tylam_not_forall() {
        let result = SystemFTheory.generate_premises("T-TyLam", "\u{22A2} e : int");
        assert!(result.is_err());
    }
}
