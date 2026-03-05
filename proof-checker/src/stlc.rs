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
                        // Try to decompose (λ (x : τ₁) e)
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
                // Try to decompose (e₁ e₂) — an application (head is not a keyword)
                if let Some(ref parts) = parts {
                    if parts.len() == 2 && !is_keyword(head) {
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
                // Try to decompose (+ e₁ e₂)
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
                // Try to decompose (- e)
                if let Some(ref parts) = parts {
                    if parts.len() == 2 && parts[0] == "-" {
                        return Ok(vec![types::format_typing_judgement_str(ctx, &parts[1], &Ty::Int)]);
                    }
                }
                Ok(vec![types::format_typing_judgement_str(ctx, "?", &Ty::Int)])
            }
            "T-If" => {
                // Try to decompose (if0 e₁ e₂ e₃)
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
                // Try to decompose (let ([x e₁]) e₂)
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
            _ => Err(format!("Unknown STLC rule '{}'", rule_name)),
        }
    }

    fn applicable_rules(&self, conclusion: &str) -> Vec<(&str, bool, Option<String>)> {
        stlc_applicable_rules(conclusion, false)
    }
}

pub fn is_keyword(head: Option<&str>) -> bool {
    matches!(head, Some("+" | "-" | "\u{03BB}" | "lambda" | "if0" | "let" | "\u{039B}" | "Lambda"))
}

/// Check which STLC rules can potentially apply to a given conclusion.
/// `include_systemf_rules` adds T-TyLam and T-TyApp.
pub fn stlc_applicable_rules(conclusion: &str, include_systemf_rules: bool) -> Vec<(&'static str, bool, Option<String>)> {
    let j = match types::parse_typing_judgement(conclusion) {
        Ok(j) => j,
        Err(_) => {
            let mut rules: Vec<(&str, bool, Option<String>)> = vec![
                "T-Var", "T-Int", "T-Bool", "T-Lam", "T-App",
                "T-Add", "T-Neg", "T-If", "T-Let",
            ].into_iter().map(|r| (r, true, None)).collect();
            if include_systemf_rules {
                rules.push(("T-TyLam", true, None));
                rules.push(("T-TyApp", true, None));
            }
            return rules;
        }
    };

    let expr = j.expr_str.trim();
    let parts = types::split_sexpr(expr);
    let head = parts.as_ref().and_then(|p| p.first().map(|s| s.as_str()));
    let is_sexpr = parts.is_some();
    let is_int_lit = expr.parse::<i64>().is_ok()
        || expr.trim_start_matches('\u{2212}').trim_start_matches('-').parse::<u64>().is_ok();
    let is_bool_lit = matches!(expr, "true" | "false" | "#t" | "#f");
    let is_var = !is_sexpr && !expr.is_empty() && !is_int_lit && !is_bool_lit;
    let is_app = is_sexpr && !is_keyword(head);
    let is_lam_arrow = matches!(&j.ty, Ty::Arrow(_, _));
    let is_forall = matches!(&j.ty, Ty::Forall(_, _));

    let mut rules = vec![
        ("T-Var", is_var,
         if !is_var { Some("expression is not a variable".into()) } else { None }),
        ("T-Int", is_int_lit,
         if !is_int_lit { Some("expression is not an integer literal".into()) } else { None }),
        ("T-Bool", is_bool_lit,
         if !is_bool_lit { Some("expression is not a boolean literal".into()) } else { None }),
        ("T-Lam", head == Some("\u{03BB}") || head == Some("lambda"),
         if !(head == Some("\u{03BB}") || head == Some("lambda")) {
             if !is_lam_arrow { Some("expression is not a \u{03BB} and type is not an arrow".into()) }
             else { Some("expression is not a \u{03BB} abstraction".into()) }
         } else { None }),
        ("T-App", is_app,
         if !is_app {
             if !is_sexpr { Some("expression is not an application".into()) }
             else { Some(format!("head '{}' is a special form, not application", head.unwrap_or("?"))) }
         } else { None }),
        ("T-Add", head == Some("+"),
         if head != Some("+") { Some("expression is not (+ ...)".into()) } else { None }),
        ("T-Neg", head == Some("-"),
         if head != Some("-") { Some("expression is not (- ...)".into()) } else { None }),
        ("T-If", head == Some("if0"),
         if head != Some("if0") { Some("expression is not (if0 ...)".into()) } else { None }),
        ("T-Let", head == Some("let"),
         if head != Some("let") { Some("expression is not (let ...)".into()) } else { None }),
    ];

    if include_systemf_rules {
        rules.push(("T-TyLam", head == Some("\u{039B}") || head == Some("Lambda"),
            if !(head == Some("\u{039B}") || head == Some("Lambda")) {
                if !is_forall { Some("expression is not \u{039B} and type is not \u{2200}".into()) }
                else { Some("expression is not a \u{039B} abstraction".into()) }
            } else { None }));
        rules.push(("T-TyApp", is_app,
            if !is_app { Some("expression is not a type application".into()) } else { None }));
    }

    rules
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
            if !types::contexts_eq(&prem.context, &j.context) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!(
                        "T-Add: premise context [{}] doesn't match conclusion context [{}]",
                        types::format_context(&prem.context),
                        types::format_context(&j.context)
                    ),
                });
            }
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
        if !types::contexts_eq(&prem.context, &j.context) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "T-Neg: premise context [{}] doesn't match conclusion context [{}]",
                    types::format_context(&prem.context),
                    types::format_context(&j.context)
                ),
            });
        }
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
        if !types::contexts_eq(&prem0.context, &j.context) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "T-If: premise context [{}] doesn't match conclusion context [{}]",
                    types::format_context(&prem0.context),
                    types::format_context(&j.context)
                ),
            });
        }
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
            if !types::contexts_eq(&prem.context, &j.context) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!(
                        "T-If: premise context [{}] doesn't match conclusion context [{}]",
                        types::format_context(&prem.context),
                        types::format_context(&j.context)
                    ),
                });
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Theory;

    #[test]
    fn test_gen_t_var() {
        let prems = STLCTheory.generate_premises("T-Var", "x : int \u{22A2} x : int").unwrap();
        assert_eq!(prems.len(), 0);
    }

    #[test]
    fn test_gen_t_lam() {
        let prems = STLCTheory.generate_premises("T-Lam", "\u{22A2} f : int \u{2192} int").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("int"));
    }

    #[test]
    fn test_gen_t_app() {
        let prems = STLCTheory.generate_premises("T-App", "\u{22A2} e : int").unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("?"));
        assert!(prems[0].contains("int"));
    }

    #[test]
    fn test_gen_t_add() {
        let prems = STLCTheory.generate_premises("T-Add", "\u{22A2} e : int").unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("int"));
    }

    #[test]
    fn test_gen_t_lam_not_arrow() {
        let result = STLCTheory.generate_premises("T-Lam", "\u{22A2} f : int");
        assert!(result.is_err());
    }

    #[test]
    fn test_context_mismatch_t_app() {
        use crate::tree::ProofNode;
        let th = STLCTheory;
        let conclusion = "\u{22A2} (f 5) : int";
        let prem0 = ProofNode {
            conclusion: "x : bool \u{22A2} f : int \u{2192} int".to_string(),
            rule_name: Some("T-Var".to_string()),

            premises: vec![],
        };
        let prem1 = ProofNode {
            conclusion: "\u{22A2} 5 : int".to_string(),
            rule_name: Some("T-Int".to_string()),

            premises: vec![],
        };
        let diags = th.check_rule("T-App", conclusion, &[&prem0, &prem1]);
        assert!(diags.iter().any(|d| d.message.contains("context") && d.message.contains("doesn't match")),
            "Should detect context mismatch in T-App premise: {:?}", diags);
    }

    #[test]
    fn test_gen_t_app_decompose() {
        // (f 5) should decompose into f and 5
        let prems = STLCTheory.generate_premises("T-App", "\u{22A2} (f 5) : int").unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("f"), "first premise should mention f: {}", prems[0]);
        assert!(prems[1].contains("5"), "second premise should mention 5: {}", prems[1]);
    }

    #[test]
    fn test_gen_t_add_decompose() {
        let prems = STLCTheory.generate_premises("T-Add", "\u{22A2} (+ 3 5) : int").unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("3"), "first premise should mention 3: {}", prems[0]);
        assert!(prems[1].contains("5"), "second premise should mention 5: {}", prems[1]);
    }

    #[test]
    fn test_gen_t_lam_decompose() {
        let prems = STLCTheory.generate_premises(
            "T-Lam",
            "\u{22A2} (\u{03BB} (x : int) (+ x 1)) : int \u{2192} int"
        ).unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("x : int"), "premise should have x : int in ctx: {}", prems[0]);
        assert!(prems[0].contains("(+ x 1)"), "premise should have body (+ x 1): {}", prems[0]);
    }

    #[test]
    fn test_gen_t_if_decompose() {
        let prems = STLCTheory.generate_premises("T-If", "\u{22A2} (if0 x 1 2) : int").unwrap();
        assert_eq!(prems.len(), 3);
        assert!(prems[0].contains("x"), "guard should mention x: {}", prems[0]);
        assert!(prems[1].contains("1"), "then should mention 1: {}", prems[1]);
        assert!(prems[2].contains("2"), "else should mention 2: {}", prems[2]);
    }

    #[test]
    fn test_gen_t_let_decompose() {
        let prems = STLCTheory.generate_premises(
            "T-Let",
            "\u{22A2} (let ([x 5]) (+ x 1)) : int"
        ).unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("5"), "binding should mention 5: {}", prems[0]);
        assert!(prems[1].contains("x : ?"), "body ctx should have x: {}", prems[1]);
        assert!(prems[1].contains("(+ x 1)"), "body should have (+ x 1): {}", prems[1]);
    }

    #[test]
    fn test_applicable_rules_app() {
        let rules = stlc_applicable_rules("\u{22A2} (f 5) : int", false);
        let app = rules.iter().find(|r| r.0 == "T-App").unwrap();
        assert!(app.1, "T-App should be applicable for (f 5)");
        let var = rules.iter().find(|r| r.0 == "T-Var").unwrap();
        assert!(!var.1, "T-Var should not be applicable for (f 5)");
        let add = rules.iter().find(|r| r.0 == "T-Add").unwrap();
        assert!(!add.1, "T-Add should not be applicable for (f 5)");
    }

    #[test]
    fn test_gen_t_app_nested() {
        // ((λ (x : int) x) 5) should decompose to (λ (x : int) x) and 5
        let prems = STLCTheory.generate_premises(
            "T-App",
            "\u{22A2} ((\u{03BB} (x : int) x) 5) : int"
        ).unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("\u{03BB}"), "first premise should have lambda: {}", prems[0]);
        assert!(prems[1].contains("5"), "second premise should have 5: {}", prems[1]);
    }

    #[test]
    fn test_applicable_rules_int() {
        let rules = stlc_applicable_rules("\u{22A2} 42 : int", false);
        let int = rules.iter().find(|r| r.0 == "T-Int").unwrap();
        assert!(int.1, "T-Int should be applicable for 42");
        let var = rules.iter().find(|r| r.0 == "T-Var").unwrap();
        assert!(!var.1, "T-Var should not be applicable for 42");
    }

    #[test]
    fn test_context_ok_t_app() {
        use crate::tree::ProofNode;
        let th = STLCTheory;
        let conclusion = "\u{22A2} (f 5) : int";
        let prem0 = ProofNode {
            conclusion: "\u{22A2} f : int \u{2192} int".to_string(),
            rule_name: Some("T-Var".to_string()),

            premises: vec![],
        };
        let prem1 = ProofNode {
            conclusion: "\u{22A2} 5 : int".to_string(),
            rule_name: Some("T-Int".to_string()),

            premises: vec![],
        };
        let diags = th.check_rule("T-App", conclusion, &[&prem0, &prem1]);
        assert!(!diags.iter().any(|d| d.level == Level::Error && d.message.contains("context")),
            "Should not flag context error when contexts match: {:?}", diags);
    }
}
