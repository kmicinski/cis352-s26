/// Propositional natural deduction (intuitionistic).
///
/// Judgement format: Γ ⊢ A
///
/// Rules:
///   Ax      — Γ ⊢ A where A ∈ Γ (no premises)
///   →I      — from Γ, A ⊢ B conclude Γ ⊢ A → B (1 premise)
///   →E      — from Γ ⊢ A → B and Γ ⊢ A conclude Γ ⊢ B (2 premises)
///   ∧I      — from Γ ⊢ A and Γ ⊢ B conclude Γ ⊢ A ∧ B (2 premises)
///   ∧E₁     — from Γ ⊢ A ∧ B conclude Γ ⊢ A (1 premise)
///   ∧E₂     — from Γ ⊢ A ∧ B conclude Γ ⊢ B (1 premise)
///   ∨I₁     — from Γ ⊢ A conclude Γ ⊢ A ∨ B (1 premise)
///   ∨I₂     — from Γ ⊢ B conclude Γ ⊢ A ∨ B (1 premise)
///   ∨E      — from Γ ⊢ A∨B, Γ,A ⊢ C, Γ,B ⊢ C conclude Γ ⊢ C (3 premises)
///   ⊥E      — from Γ ⊢ ⊥ conclude Γ ⊢ A (1 premise)
///   ¬I      — from Γ, A ⊢ ⊥ conclude Γ ⊢ ¬A (1 premise)
///   ¬E      — from Γ ⊢ ¬A and Γ ⊢ A conclude Γ ⊢ ⊥ (2 premises)

use crate::check::{Diagnostic, Level, Theory};
use crate::formula::{self, Formula, Sequent};
use crate::tree::ProofNode;

pub struct PropNDTheory;

const SEP: char = '\u{22A2}'; // ⊢

impl Theory for PropNDTheory {
    fn name(&self) -> &str {
        "Natural Deduction (Propositional Logic)"
    }

    fn known_rules(&self) -> Vec<&str> {
        vec![
            "Ax", "Id",
            "\u{2192}I", "ImpI", "->I",
            "\u{2192}E", "ImpE", "->E",
            "\u{2227}I", "AndI", "/\\I",
            "\u{2227}E\u{2081}", "AndE1", "/\\E1",
            "\u{2227}E\u{2082}", "AndE2", "/\\E2",
            "\u{2228}I\u{2081}", "OrI1", "\\/I1",
            "\u{2228}I\u{2082}", "OrI2", "\\/I2",
            "\u{2228}E", "OrE", "\\/E",
            "\u{22A5}E", "BotE",
            "\u{00AC}I", "NotI", "~I",
            "\u{00AC}E", "NotE", "~E",
        ]
    }

    fn check_rule(
        &self,
        rule_name: &str,
        conclusion: &str,
        premises: &[&ProofNode],
    ) -> Vec<Diagnostic> {
        let seq = match formula::parse_sequent(conclusion, SEP) {
            Ok(s) => s,
            Err(e) => {
                return vec![Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: format!("Can't parse conclusion as judgement: {}", e),
                }];
            }
        };

        match normalize_rule(rule_name) {
            "Ax" => check_ax(&seq, premises),
            "ImpI" => check_imp_i(&seq, premises),
            "ImpE" => check_imp_e(&seq, premises),
            "AndI" => check_and_i(&seq, premises),
            "AndE1" => check_and_e1(&seq, premises),
            "AndE2" => check_and_e2(&seq, premises),
            "OrI1" => check_or_i1(&seq, premises),
            "OrI2" => check_or_i2(&seq, premises),
            "OrE" => check_or_e(&seq, premises),
            "BotE" => check_bot_e(&seq, premises),
            "NotI" => check_not_i(&seq, premises),
            "NotE" => check_not_e(&seq, premises),
            _ => vec![Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("Unknown natural deduction rule '{}'", rule_name),
            }],
        }
    }

    fn is_judgement(&self, s: &str) -> bool {
        s.contains('\u{22A2}') && !s.contains('\u{21D2}') && !s.contains('\u{21D3}')
    }
}

fn normalize_rule(name: &str) -> &str {
    match name {
        "Ax" | "Id" => "Ax",
        "\u{2192}I" | "ImpI" | "->I" => "ImpI",
        "\u{2192}E" | "ImpE" | "->E" => "ImpE",
        "\u{2227}I" | "AndI" | "/\\I" => "AndI",
        "\u{2227}E\u{2081}" | "AndE1" | "/\\E1" => "AndE1",
        "\u{2227}E\u{2082}" | "AndE2" | "/\\E2" => "AndE2",
        "\u{2228}I\u{2081}" | "OrI1" | "\\/I1" => "OrI1",
        "\u{2228}I\u{2082}" | "OrI2" | "\\/I2" => "OrI2",
        "\u{2228}E" | "OrE" | "\\/E" => "OrE",
        "\u{22A5}E" | "BotE" => "BotE",
        "\u{00AC}I" | "NotI" | "~I" => "NotI",
        "\u{00AC}E" | "NotE" | "~E" => "NotE",
        _ => name,
    }
}

fn parse_premise_seq(p: &ProofNode) -> Result<Sequent, String> {
    formula::parse_sequent(&p.conclusion, SEP)
}

// ── Ax: Γ ⊢ A where A ∈ Γ ──────────────────────────────────────────

fn check_ax(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("Ax expects 0 premises, got {}", premises.len()),
        });
    }

    if !formula::contains_formula(&seq.antecedents, &seq.succedent) {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!(
                "Ax: {} must appear in the context {}",
                seq.succedent,
                formula::format_formula_list(&seq.antecedents)
            ),
        });
    }

    diags
}

// ── →I: from Γ, A ⊢ B conclude Γ ⊢ A → B ──────────────────────────

fn check_imp_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2192}I expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    let (ant, con) = match &seq.succedent {
        Formula::Imp(a, b) => (a.as_ref(), b.as_ref()),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2192}I requires the conclusion to be A \u{2192} B".to_string(),
            });
            return diags;
        }
    };

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != *con {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise should prove {}, got {}", con, prem.succedent),
            });
        }
        if !formula::multiset_add_one(&seq.antecedents, &prem.antecedents, ant) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "Premise context should be {}, {}",
                    formula::format_formula_list(&seq.antecedents),
                    ant
                ),
            });
        }
    }

    diags
}

// ── →E: from Γ ⊢ A → B and Γ ⊢ A conclude Γ ⊢ B ──────────────────

fn check_imp_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2192}E expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    let prem0 = match parse_premise_seq(premises[0]) {
        Ok(s) => s,
        Err(_) => return diags,
    };
    let prem1 = match parse_premise_seq(premises[1]) {
        Ok(s) => s,
        Err(_) => return diags,
    };

    // First premise should prove A → B
    match &prem0.succedent {
        Formula::Imp(a, b) => {
            // B should equal conclusion succedent
            if **b != seq.succedent {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: format!(
                        "The consequent of the implication ({}) doesn't match the conclusion ({})",
                        b, seq.succedent
                    ),
                });
            }
            // Second premise should prove A
            if prem1.succedent != **a {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![1],
                    message: format!(
                        "Second premise should prove {}, got {}",
                        a, prem1.succedent
                    ),
                });
            }
        }
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "First premise of \u{2192}E must prove an implication".to_string(),
            });
        }
    }

    // Both premises should have the same context as the conclusion
    for (i, prem) in [&prem0, &prem1].iter().enumerate() {
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![i],
                message: "Premise context should match conclusion context".to_string(),
            });
        }
    }

    diags
}

// ── ∧I: from Γ ⊢ A and Γ ⊢ B conclude Γ ⊢ A ∧ B ──────────────────

fn check_and_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2227}I expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    let (left, right) = match &seq.succedent {
        Formula::And(a, b) => (a.as_ref(), b.as_ref()),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2227}I requires the conclusion to be A \u{2227} B".to_string(),
            });
            return diags;
        }
    };

    for (i, (expected, label)) in [(left, "A"), (right, "B")].iter().enumerate() {
        if let Ok(prem) = parse_premise_seq(premises[i]) {
            if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!("Premise {} context should match conclusion context", label),
                });
            }
            if prem.succedent != **expected {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!("Premise {} should prove {}, got {}", label, expected, prem.succedent),
                });
            }
        }
    }

    diags
}

// ── ∧E₁: from Γ ⊢ A ∧ B conclude Γ ⊢ A ────────────────────────────

fn check_and_e1(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2227}E\u{2081} expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        match &prem.succedent {
            Formula::And(a, _) => {
                if **a != seq.succedent {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!(
                            "Left conjunct {} doesn't match conclusion {}",
                            a, seq.succedent
                        ),
                    });
                }
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: "Premise must prove a conjunction A \u{2227} B".to_string(),
                });
            }
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise context should match conclusion context".to_string(),
            });
        }
    }

    diags
}

// ── ∧E₂: from Γ ⊢ A ∧ B conclude Γ ⊢ B ────────────────────────────

fn check_and_e2(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2227}E\u{2082} expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        match &prem.succedent {
            Formula::And(_, b) => {
                if **b != seq.succedent {
                    diags.push(Diagnostic {
                        level: Level::Error,
                        path: vec![0],
                        message: format!(
                            "Right conjunct {} doesn't match conclusion {}",
                            b, seq.succedent
                        ),
                    });
                }
            }
            _ => {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![0],
                    message: "Premise must prove a conjunction A \u{2227} B".to_string(),
                });
            }
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise context should match conclusion context".to_string(),
            });
        }
    }

    diags
}

// ── ∨I₁: from Γ ⊢ A conclude Γ ⊢ A ∨ B ────────────────────────────

fn check_or_i1(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2228}I\u{2081} expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    let left = match &seq.succedent {
        Formula::Or(a, _) => a.as_ref(),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2228}I\u{2081} requires the conclusion to be A \u{2228} B".to_string(),
            });
            return diags;
        }
    };

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != *left {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise should prove {}, got {}", left, prem.succedent),
            });
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise context should match conclusion context".to_string(),
            });
        }
    }

    diags
}

// ── ∨I₂: from Γ ⊢ B conclude Γ ⊢ A ∨ B ────────────────────────────

fn check_or_i2(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2228}I\u{2082} expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    let right = match &seq.succedent {
        Formula::Or(_, b) => b.as_ref(),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2228}I\u{2082} requires the conclusion to be A \u{2228} B".to_string(),
            });
            return diags;
        }
    };

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != *right {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise should prove {}, got {}", right, prem.succedent),
            });
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise context should match conclusion context".to_string(),
            });
        }
    }

    diags
}

// ── ∨E: from Γ ⊢ A∨B, Γ,A ⊢ C, Γ,B ⊢ C conclude Γ ⊢ C ──────────

fn check_or_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 3 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2228}E expects 3 premises, got {}", premises.len()),
        });
        return diags;
    }

    let prem0 = match parse_premise_seq(premises[0]) {
        Ok(s) => s,
        Err(_) => return diags,
    };

    // First premise should prove A ∨ B
    let (a, b) = match &prem0.succedent {
        Formula::Or(a, b) => (a.as_ref().clone(), b.as_ref().clone()),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "First premise of \u{2228}E must prove a disjunction A \u{2228} B".to_string(),
            });
            return diags;
        }
    };

    if !formula::multiset_eq(&prem0.antecedents, &seq.antecedents) {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![0],
            message: "First premise context should match conclusion context".to_string(),
        });
    }

    // Second premise: Γ, A ⊢ C
    if let Ok(prem1) = parse_premise_seq(premises[1]) {
        if prem1.succedent != seq.succedent {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!("Second premise should prove {}", seq.succedent),
            });
        }
        if !formula::multiset_add_one(&seq.antecedents, &prem1.antecedents, &a) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![1],
                message: format!("Second premise context should be \u{0393}, {}", a),
            });
        }
    }

    // Third premise: Γ, B ⊢ C
    if let Ok(prem2) = parse_premise_seq(premises[2]) {
        if prem2.succedent != seq.succedent {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![2],
                message: format!("Third premise should prove {}", seq.succedent),
            });
        }
        if !formula::multiset_add_one(&seq.antecedents, &prem2.antecedents, &b) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![2],
                message: format!("Third premise context should be \u{0393}, {}", b),
            });
        }
    }

    diags
}

// ── ⊥E: from Γ ⊢ ⊥ conclude Γ ⊢ A ─────────────────────────────────

fn check_bot_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{22A5}E expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != Formula::Bot {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise must prove \u{22A5}, got {}", prem.succedent),
            });
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise context should match conclusion context".to_string(),
            });
        }
    }

    diags
}

// ── ¬I: from Γ, A ⊢ ⊥ conclude Γ ⊢ ¬A ─────────────────────────────

fn check_not_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{00AC}I expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    let inner = match &seq.succedent {
        Formula::Not(a) => a.as_ref(),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{00AC}I requires the conclusion to be \u{00AC}A".to_string(),
            });
            return diags;
        }
    };

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != Formula::Bot {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise must prove \u{22A5}, got {}", prem.succedent),
            });
        }
        if !formula::multiset_add_one(&seq.antecedents, &prem.antecedents, inner) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise context should be \u{0393}, {}", inner),
            });
        }
    }

    diags
}

// ── ¬E: from Γ ⊢ ¬A and Γ ⊢ A conclude Γ ⊢ ⊥ ─────────────────────

fn check_not_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{00AC}E expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    if seq.succedent != Formula::Bot {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{00AC}E conclusion must be \u{22A5}".to_string(),
        });
    }

    let prem0 = match parse_premise_seq(premises[0]) {
        Ok(s) => s,
        Err(_) => return diags,
    };
    let prem1 = match parse_premise_seq(premises[1]) {
        Ok(s) => s,
        Err(_) => return diags,
    };

    // First premise: Γ ⊢ ¬A
    let inner = match &prem0.succedent {
        Formula::Not(a) => a.as_ref().clone(),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "First premise must prove a negation \u{00AC}A".to_string(),
            });
            return diags;
        }
    };

    // Second premise: Γ ⊢ A
    if prem1.succedent != inner {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![1],
            message: format!("Second premise should prove {}, got {}", inner, prem1.succedent),
        });
    }

    // Check contexts
    for (i, prem) in [&prem0, &prem1].iter().enumerate() {
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![i],
                message: "Premise context should match conclusion context".to_string(),
            });
        }
    }

    diags
}
