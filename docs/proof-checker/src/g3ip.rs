/// G3ip: Gentzen's sequent calculus for intuitionistic propositional logic.
///
/// Judgement format: Γ ⇒ A
///
/// Rules:
///   Ax      — P, Γ ⇒ P (P atomic, no premises)
///   ⊥L      — ⊥, Γ ⇒ C (no premises)
///   ⊤R      — Γ ⇒ ⊤ (no premises)
///   →R      — from A, Γ ⇒ B conclude Γ ⇒ A → B (1 premise)
///   →L      — from A→B, Γ ⇒ A and B, Γ ⇒ C conclude A→B, Γ ⇒ C (2 premises)
///   ∧R      — from Γ ⇒ A and Γ ⇒ B conclude Γ ⇒ A ∧ B (2 premises)
///   ∧L      — from A, B, Γ ⇒ C conclude A ∧ B, Γ ⇒ C (1 premise)
///   ∨R₁     — from Γ ⇒ A conclude Γ ⇒ A ∨ B (1 premise)
///   ∨R₂     — from Γ ⇒ B conclude Γ ⇒ A ∨ B (1 premise)
///   ∨L      — from A, Γ ⇒ C and B, Γ ⇒ C conclude A ∨ B, Γ ⇒ C (2 premises)

use crate::check::{Diagnostic, Level, Theory};
use crate::formula::{self, Formula, Sequent};
use crate::tree::ProofNode;

pub struct G3ipTheory;

const SEP: char = '\u{21D2}'; // ⇒

impl Theory for G3ipTheory {
    fn name(&self) -> &str {
        "G3ip (Intuitionistic Sequent Calculus)"
    }

    fn known_rules(&self) -> Vec<&str> {
        vec![
            "Ax", "Id",
            "\u{22A5}L", "BotL",
            "\u{22A4}R", "TopR",
            "\u{2192}R", "ImpR", "->R",
            "\u{2192}L", "ImpL", "->L",
            "\u{2227}R", "AndR", "/\\R",
            "\u{2227}L", "AndL", "/\\L",
            "\u{2228}R\u{2081}", "OrR1", "\\/R1",
            "\u{2228}R\u{2082}", "OrR2", "\\/R2",
            "\u{2228}L", "OrL", "\\/L",
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
                    message: format!("Can't parse conclusion as sequent: {}", e),
                }];
            }
        };

        match normalize_rule(rule_name) {
            "Ax" => check_ax(&seq, premises),
            "BotL" => check_bot_l(&seq, premises),
            "TopR" => check_top_r(&seq, premises),
            "ImpR" => check_imp_r(&seq, premises),
            "ImpL" => check_imp_l(&seq, premises),
            "AndR" => check_and_r(&seq, premises),
            "AndL" => check_and_l(&seq, premises),
            "OrR1" => check_or_r1(&seq, premises),
            "OrR2" => check_or_r2(&seq, premises),
            "OrL" => check_or_l(&seq, premises),
            _ => vec![Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("Unknown G3ip rule '{}'", rule_name),
            }],
        }
    }

    fn is_judgement(&self, s: &str) -> bool {
        s.contains('\u{21D2}') // ⇒
    }

    fn applicable_rules(&self, conclusion: &str) -> Vec<(&str, bool, Option<String>)> {
        let seq = match formula::parse_sequent(conclusion, SEP) {
            Ok(s) => s,
            Err(_) => return self.known_rules().into_iter().map(|r| (r, true, None)).collect(),
        };
        let suc = &seq.succedent;
        let has_bot = formula::contains_formula(&seq.antecedents, &Formula::Bot);
        let has_imp = seq.antecedents.iter().any(|f| matches!(f, Formula::Imp(_, _)));
        let has_and = seq.antecedents.iter().any(|f| matches!(f, Formula::And(_, _)));
        let has_or  = seq.antecedents.iter().any(|f| matches!(f, Formula::Or(_, _)));
        let suc_is_atom = matches!(suc, Formula::Atom(_));
        let atom_in_ant = suc_is_atom && formula::contains_formula(&seq.antecedents, suc);

        // Only return the canonical rule names used in the dropdown
        vec![
            ("Ax",    atom_in_ant, if !atom_in_ant { Some("succedent must be an atom present in antecedent".into()) } else { None }),
            ("\u{22A5}L", has_bot, if !has_bot { Some("\u{22A5} not in antecedent".into()) } else { None }),
            ("\u{22A4}R", *suc == Formula::Top, if *suc != Formula::Top { Some("succedent is not \u{22A4}".into()) } else { None }),
            ("\u{2227}R", matches!(suc, Formula::And(_, _)), if !matches!(suc, Formula::And(_, _)) { Some("succedent is not a conjunction".into()) } else { None }),
            ("\u{2227}L", has_and, if !has_and { Some("no conjunction in antecedent".into()) } else { None }),
            ("\u{2228}R\u{2081}", matches!(suc, Formula::Or(_, _)), if !matches!(suc, Formula::Or(_, _)) { Some("succedent is not a disjunction".into()) } else { None }),
            ("\u{2228}R\u{2082}", matches!(suc, Formula::Or(_, _)), if !matches!(suc, Formula::Or(_, _)) { Some("succedent is not a disjunction".into()) } else { None }),
            ("\u{2228}L", has_or, if !has_or { Some("no disjunction in antecedent".into()) } else { None }),
            ("\u{2192}R", matches!(suc, Formula::Imp(_, _)), if !matches!(suc, Formula::Imp(_, _)) { Some("succedent is not an implication".into()) } else { None }),
            ("\u{2192}L", has_imp, if !has_imp { Some("no implication in antecedent".into()) } else { None }),
        ]
    }

    fn generate_premises(&self, rule_name: &str, conclusion: &str) -> Result<Vec<String>, String> {
        let seq = formula::parse_sequent(conclusion, SEP)
            .map_err(|e| format!("Can't parse conclusion: {}", e))?;
        let ants = &seq.antecedents;
        let suc = &seq.succedent;

        match normalize_rule(rule_name) {
            "Ax" | "BotL" | "TopR" => Ok(vec![]),
            "AndR" => {
                match suc {
                    Formula::And(a, b) => {
                        let p1 = formula::format_sequent_str(ants, a, SEP);
                        let p2 = formula::format_sequent_str(ants, b, SEP);
                        Ok(vec![p1, p2])
                    }
                    _ => Err(format!("\u{2227}R requires the succedent to be A \u{2227} B, but got {}", suc)),
                }
            }
            "AndL" => {
                // Find first conjunction in antecedent
                let pos = ants.iter().position(|f| matches!(f, Formula::And(_, _)));
                match pos {
                    Some(idx) => {
                        if let Formula::And(a, b) = &ants[idx] {
                            let mut new_ants = ants.to_vec();
                            new_ants.remove(idx);
                            new_ants.insert(idx, *b.clone());
                            new_ants.insert(idx, *a.clone());
                            let p = formula::format_sequent_str(&new_ants, suc, SEP);
                            Ok(vec![p])
                        } else {
                            unreachable!()
                        }
                    }
                    None => Err(format!("\u{2227}L requires a conjunction A \u{2227} B in the antecedent, but found only: {}", formula::format_formula_list(ants))),
                }
            }
            "OrR1" => {
                match suc {
                    Formula::Or(a, _) => {
                        let p = formula::format_sequent_str(ants, a, SEP);
                        Ok(vec![p])
                    }
                    _ => Err(format!("\u{2228}R\u{2081} requires the succedent to be A \u{2228} B, but got {}", suc)),
                }
            }
            "OrR2" => {
                match suc {
                    Formula::Or(_, b) => {
                        let p = formula::format_sequent_str(ants, b, SEP);
                        Ok(vec![p])
                    }
                    _ => Err(format!("\u{2228}R\u{2082} requires the succedent to be A \u{2228} B, but got {}", suc)),
                }
            }
            "OrL" => {
                let pos = ants.iter().position(|f| matches!(f, Formula::Or(_, _)));
                match pos {
                    Some(idx) => {
                        if let Formula::Or(a, b) = &ants[idx] {
                            let mut gamma_prime = ants.to_vec();
                            gamma_prime.remove(idx);
                            let mut ants1 = vec![*a.clone()];
                            ants1.extend(gamma_prime.clone());
                            let mut ants2 = vec![*b.clone()];
                            ants2.extend(gamma_prime);
                            let p1 = formula::format_sequent_str(&ants1, suc, SEP);
                            let p2 = formula::format_sequent_str(&ants2, suc, SEP);
                            Ok(vec![p1, p2])
                        } else {
                            unreachable!()
                        }
                    }
                    None => Err(format!("\u{2228}L requires a disjunction A \u{2228} B in the antecedent, but found only: {}", formula::format_formula_list(ants))),
                }
            }
            "ImpR" => {
                match suc {
                    Formula::Imp(a, b) => {
                        let mut new_ants = vec![*a.clone()];
                        new_ants.extend(ants.to_vec());
                        let p = formula::format_sequent_str(&new_ants, b, SEP);
                        Ok(vec![p])
                    }
                    _ => Err(format!("\u{2192}R requires the succedent to be A \u{2192} B, but got {}", suc)),
                }
            }
            "ImpL" => {
                let pos = ants.iter().position(|f| matches!(f, Formula::Imp(_, _)));
                match pos {
                    Some(idx) => {
                        if let Formula::Imp(a, b) = &ants[idx] {
                            // First premise: Γ ⇒ A (keep full antecedent including A→B)
                            let p1 = formula::format_sequent_str(ants, a, SEP);
                            // Second premise: B, Γ' ⇒ C where Γ' has A→B replaced by B
                            let mut new_ants = ants.to_vec();
                            new_ants[idx] = *b.clone();
                            let p2 = formula::format_sequent_str(&new_ants, suc, SEP);
                            Ok(vec![p1, p2])
                        } else {
                            unreachable!()
                        }
                    }
                    None => Err(format!("\u{2192}L requires an implication A \u{2192} B in the antecedent, but found only: {}", formula::format_formula_list(ants))),
                }
            }
            _ => Err(format!("Unknown G3ip rule '{}'", rule_name)),
        }
    }
}

fn normalize_rule(name: &str) -> &str {
    match name {
        "Ax" | "Id" => "Ax",
        "\u{22A5}L" | "BotL" => "BotL",
        "\u{22A4}R" | "TopR" => "TopR",
        "\u{2192}R" | "ImpR" | "->R" => "ImpR",
        "\u{2192}L" | "ImpL" | "->L" => "ImpL",
        "\u{2227}R" | "AndR" | "/\\R" => "AndR",
        "\u{2227}L" | "AndL" | "/\\L" => "AndL",
        "\u{2228}R\u{2081}" | "OrR1" | "\\/R1" => "OrR1",
        "\u{2228}R\u{2082}" | "OrR2" | "\\/R2" => "OrR2",
        "\u{2228}L" | "OrL" | "\\/L" => "OrL",
        _ => name,
    }
}

fn parse_premise_seq(p: &ProofNode) -> Result<Sequent, String> {
    formula::parse_sequent(&p.conclusion, SEP)
}

// ── Ax: P, Γ ⇒ P (P atomic) ────────────────────────────────────────

fn check_ax(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("Ax expects 0 premises, got {}", premises.len()),
        });
    }

    // Succedent must be an atom
    match &seq.succedent {
        Formula::Atom(p) => {
            if !formula::contains_formula(&seq.antecedents, &Formula::Atom(p.clone())) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: format!("Ax: atom {} must appear in the antecedent", p),
                });
            }
        }
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "Ax rule requires the succedent to be an atom".to_string(),
            });
        }
    }

    diags
}

// ── ⊥L: ⊥, Γ ⇒ C ──────────────────────────────────────────────────

fn check_bot_l(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{22A5}L expects 0 premises, got {}", premises.len()),
        });
    }

    if !formula::contains_formula(&seq.antecedents, &Formula::Bot) {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{22A5}L requires \u{22A5} in the antecedent".to_string(),
        });
    }

    diags
}

// ── ⊤R: Γ ⇒ ⊤ ──────────────────────────────────────────────────────

fn check_top_r(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if !premises.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{22A4}R expects 0 premises, got {}", premises.len()),
        });
    }

    if seq.succedent != Formula::Top {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{22A4}R requires the succedent to be \u{22A4}".to_string(),
        });
    }

    diags
}

// ── →R: from A, Γ ⇒ B conclude Γ ⇒ A → B ──────────────────────────

fn check_imp_r(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2192}R expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    let (ant, con) = match &seq.succedent {
        Formula::Imp(a, b) => (a.as_ref(), b.as_ref()),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2192}R requires the succedent to be an implication A \u{2192} B".to_string(),
            });
            return diags;
        }
    };

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        // Premise succedent should be B
        if prem.succedent != *con {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "Premise succedent should be {}, got {}",
                    con, prem.succedent
                ),
            });
        }
        // Premise antecedent should be {A} ∪ Γ
        if !formula::multiset_add_one(&seq.antecedents, &prem.antecedents, ant) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "Premise antecedent should be {}, {} but got {}",
                    ant,
                    formula::format_formula_list(&seq.antecedents),
                    formula::format_formula_list(&prem.antecedents),
                ),
            });
        }
    }

    diags
}

// ── →L: from A→B, Γ ⇒ A and B, Γ ⇒ C conclude A→B, Γ ⇒ C ────────

fn check_imp_l(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2192}L expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    // Find an implication in the antecedent
    let imp_candidates: Vec<(usize, &Formula, &Formula)> = seq
        .antecedents
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            if let Formula::Imp(a, b) = f {
                Some((i, a.as_ref(), b.as_ref()))
            } else {
                None
            }
        })
        .collect();

    if imp_candidates.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{2192}L requires an implication in the antecedent".to_string(),
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

    // Try each implication candidate
    let mut found = false;
    for (_idx, a, b) in &imp_candidates {
        let imp_formula = Formula::Imp(Box::new((*a).clone()), Box::new((*b).clone()));

        // First premise: A→B, Γ ⇒ A (same antecedent, succedent = A)
        let p0_ant_ok = formula::multiset_eq(&prem0.antecedents, &seq.antecedents);
        let p0_suc_ok = prem0.succedent == **a;

        // Second premise: B, Γ' ⇒ C where Γ' = Γ\{A→B} ∪ {B}
        // Actually: B, Γ ⇒ C means antecedent = (conclusion antecedent with A→B replaced by B)
        let p1_ant_ok =
            formula::multiset_replace_one(&seq.antecedents, &prem1.antecedents, &imp_formula, b);
        let p1_suc_ok = prem1.succedent == seq.succedent;

        if p0_ant_ok && p0_suc_ok && p1_ant_ok && p1_suc_ok {
            found = true;
            break;
        }
    }

    if !found {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{2192}L: premises don't match. First premise should prove A (the antecedent of the implication), second should prove the original succedent with B replacing A\u{2192}B in the context.".to_string(),
        });
    }

    diags
}

// ── ∧R: from Γ ⇒ A and Γ ⇒ B conclude Γ ⇒ A ∧ B ──────────────────

fn check_and_r(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2227}R expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    let (left, right) = match &seq.succedent {
        Formula::And(a, b) => (a.as_ref(), b.as_ref()),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2227}R requires the succedent to be A \u{2227} B".to_string(),
            });
            return diags;
        }
    };

    for (i, (expected_suc, label)) in [(left, "A"), (right, "B")].iter().enumerate() {
        if let Ok(prem) = parse_premise_seq(premises[i]) {
            if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!("Premise {} antecedent should match conclusion antecedent", label),
                });
            }
            if prem.succedent != **expected_suc {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: format!(
                        "Premise {} succedent should be {}, got {}",
                        label, expected_suc, prem.succedent
                    ),
                });
            }
        }
    }

    diags
}

// ── ∧L: from A, B, Γ ⇒ C conclude A ∧ B, Γ ⇒ C ───────────────────

fn check_and_l(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2227}L expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    // Find a conjunction in the antecedent
    let conj_candidates: Vec<(usize, &Formula, &Formula)> = seq
        .antecedents
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            if let Formula::And(a, b) = f {
                Some((i, a.as_ref(), b.as_ref()))
            } else {
                None
            }
        })
        .collect();

    if conj_candidates.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{2227}L requires a conjunction in the antecedent".to_string(),
        });
        return diags;
    }

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        // Check succedent matches
        if prem.succedent != seq.succedent {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!(
                    "Premise succedent should be {}, got {}",
                    seq.succedent, prem.succedent
                ),
            });
        }

        // Try each conjunction: premise antecedent = (Γ \ {A∧B}) ∪ {A, B}
        let mut found = false;
        for (_idx, a, b) in &conj_candidates {
            let conj = Formula::And(Box::new((*a).clone()), Box::new((*b).clone()));
            let mut expected = seq.antecedents.clone();
            if let Some(pos) = expected.iter().position(|f| *f == conj) {
                expected.remove(pos);
                expected.push((*a).clone());
                expected.push((*b).clone());
                if formula::multiset_eq(&expected, &prem.antecedents) {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "\u{2227}L: premise antecedent should replace A \u{2227} B with A and B".to_string(),
            });
        }
    }

    diags
}

// ── ∨R₁: from Γ ⇒ A conclude Γ ⇒ A ∨ B ────────────────────────────

fn check_or_r1(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2228}R\u{2081} expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    let left = match &seq.succedent {
        Formula::Or(a, _) => a.as_ref(),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2228}R\u{2081} requires the succedent to be A \u{2228} B".to_string(),
            });
            return diags;
        }
    };

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise antecedent should match conclusion antecedent".to_string(),
            });
        }
        if prem.succedent != *left {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise succedent should be {}, got {}", left, prem.succedent),
            });
        }
    }

    diags
}

// ── ∨R₂: from Γ ⇒ B conclude Γ ⇒ A ∨ B ────────────────────────────

fn check_or_r2(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if premises.len() != 1 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2228}R\u{2082} expects 1 premise, got {}", premises.len()),
        });
        return diags;
    }

    let right = match &seq.succedent {
        Formula::Or(_, b) => b.as_ref(),
        _ => {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "\u{2228}R\u{2082} requires the succedent to be A \u{2228} B".to_string(),
            });
            return diags;
        }
    };

    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: "Premise antecedent should match conclusion antecedent".to_string(),
            });
        }
        if prem.succedent != *right {
            diags.push(Diagnostic {
                level: Level::Error,
                path: vec![0],
                message: format!("Premise succedent should be {}, got {}", right, prem.succedent),
            });
        }
    }

    diags
}

// ── ∨L: from A, Γ ⇒ C and B, Γ ⇒ C conclude A ∨ B, Γ ⇒ C ────────

fn check_or_l(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    if premises.len() != 2 {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: format!("\u{2228}L expects 2 premises, got {}", premises.len()),
        });
        return diags;
    }

    // Find a disjunction in the antecedent
    let disj_candidates: Vec<(usize, &Formula, &Formula)> = seq
        .antecedents
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            if let Formula::Or(a, b) = f {
                Some((i, a.as_ref(), b.as_ref()))
            } else {
                None
            }
        })
        .collect();

    if disj_candidates.is_empty() {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{2228}L requires a disjunction in the antecedent".to_string(),
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

    let mut found = false;
    for (_idx, a, b) in &disj_candidates {
        let disj = Formula::Or(Box::new((*a).clone()), Box::new((*b).clone()));

        // First premise: A, Γ' ⇒ C where Γ' = Γ \ {A∨B}
        let p0_ant_ok =
            formula::multiset_replace_one(&seq.antecedents, &prem0.antecedents, &disj, a);
        let p0_suc_ok = prem0.succedent == seq.succedent;

        // Second premise: B, Γ' ⇒ C
        let p1_ant_ok =
            formula::multiset_replace_one(&seq.antecedents, &prem1.antecedents, &disj, b);
        let p1_suc_ok = prem1.succedent == seq.succedent;

        if p0_ant_ok && p0_suc_ok && p1_ant_ok && p1_suc_ok {
            found = true;
            break;
        }
    }

    if !found {
        diags.push(Diagnostic {
            level: Level::Error,
            path: vec![],
            message: "\u{2228}L: premises don't match. First/second premise should prove the succedent with A/B (respectively) replacing A\u{2228}B in the context.".to_string(),
        });
    }

    diags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Theory;

    #[test]
    fn test_gen_and_r() {
        let prems = G3ipTheory.generate_premises("\u{2227}R", "P, Q \u{21D2} P \u{2227} Q").unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("P"));
        assert!(prems[1].contains("Q"));
    }

    #[test]
    fn test_gen_and_l() {
        let prems = G3ipTheory.generate_premises("\u{2227}L", "P \u{2227} Q \u{21D2} R").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("P"));
        assert!(prems[0].contains("Q"));
        assert!(prems[0].contains("R"));
    }

    #[test]
    fn test_gen_or_r1() {
        let prems = G3ipTheory.generate_premises("OrR1", "P \u{21D2} P \u{2228} Q").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("P"));
    }

    #[test]
    fn test_gen_or_l() {
        let prems = G3ipTheory.generate_premises("OrL", "P \u{2228} Q \u{21D2} R").unwrap();
        assert_eq!(prems.len(), 2);
        assert!(prems[0].contains("P"));
        assert!(prems[1].contains("Q"));
    }

    #[test]
    fn test_gen_imp_r() {
        let prems = G3ipTheory.generate_premises("ImpR", "\u{21D2} P \u{2192} Q").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("P"));
        assert!(prems[0].contains("Q"));
    }

    #[test]
    fn test_gen_imp_l() {
        let prems = G3ipTheory.generate_premises("ImpL", "P \u{2192} Q \u{21D2} R").unwrap();
        assert_eq!(prems.len(), 2);
    }

    #[test]
    fn test_gen_ax_zero_premises() {
        let prems = G3ipTheory.generate_premises("Ax", "P \u{21D2} P").unwrap();
        assert_eq!(prems.len(), 0);
    }

    #[test]
    fn test_gen_and_r_not_conjunction() {
        let result = G3ipTheory.generate_premises("\u{2227}R", "P \u{21D2} P \u{2192} Q");
        assert!(result.is_err());
    }
}
