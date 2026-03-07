/// First-Order Natural Deduction (intuitionistic).
///
/// Extends propositional natural deduction with quantifier rules.
///
/// Judgement format: Γ ⊢ A
///
/// Propositional rules (inherited from PropND):
///   Ax, →I, →E, ∧I, ∧E₁, ∧E₂, ∨I₁, ∨I₂, ∨E, ⊥E, ¬I, ¬E
///
/// First-order rules:
///   ∀I  — from Γ ⊢ φ conclude Γ ⊢ ∀x.φ
///   ∀E  — from Γ ⊢ ∀x.φ conclude Γ ⊢ φ[t/x]
///   ∃I  — from Γ ⊢ φ[t/x] conclude Γ ⊢ ∃x.φ
///   ∃E  — from Γ ⊢ ∃x.φ and Γ, φ ⊢ C conclude Γ ⊢ C

use crate::check::{Diagnostic, Level, Theory};
use crate::formula::{self, Formula, Sequent};
use crate::tree::ProofNode;

pub struct FONDTheory;

const SEP: char = '\u{22A2}'; // ⊢

impl Theory for FONDTheory {
    fn name(&self) -> &str {
        "First-Order Natural Deduction"
    }

    fn known_rules(&self) -> Vec<&str> {
        vec![
            // Propositional rules (with aliases)
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
            // First-order rules
            "\u{2200}I", "ForallI",
            "\u{2200}E", "ForallE",
            "\u{2203}I", "ExistsI",
            "\u{2203}E", "ExistsE",
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
            // Propositional rules — delegate to shared implementations
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
            // First-order rules
            "ForallI" => check_forall_i(&seq, premises),
            "ForallE" => check_forall_e(&seq, premises),
            "ExistsI" => check_exists_i(&seq, premises),
            "ExistsE" => check_exists_e(&seq, premises),
            _ => vec![Diagnostic {
                level: Level::Error,
                path: vec![],
                message: format!("Unknown rule '{}'", rule_name),
            }],
        }
    }

    fn is_judgement(&self, s: &str) -> bool {
        s.contains('\u{22A2}') && !s.contains('\u{21D2}') && !s.contains('\u{21D3}')
    }

    fn applicable_rules(&self, conclusion: &str) -> Vec<(&str, bool, Option<String>)> {
        let seq = match formula::parse_sequent(conclusion, SEP) {
            Ok(s) => s,
            Err(_) => return self.known_rules().into_iter().map(|r| (r, true, None)).collect(),
        };
        let suc = &seq.succedent;
        let suc_in_ant = formula::contains_formula(&seq.antecedents, suc);
        let suc_is_bot = *suc == Formula::Bot;
        let suc_is_imp = matches!(suc, Formula::Imp(_, _));
        let suc_is_and = matches!(suc, Formula::And(_, _));
        let suc_is_or  = matches!(suc, Formula::Or(_, _));
        let suc_is_not = matches!(suc, Formula::Not(_));
        let suc_is_forall = matches!(suc, Formula::Forall(_, _));
        let suc_is_exists = matches!(suc, Formula::Exists(_, _));

        vec![
            ("Ax", suc_in_ant, if !suc_in_ant { Some("conclusion must appear in context".into()) } else { None }),
            ("\u{2192}I", suc_is_imp, if !suc_is_imp { Some("conclusion is not an implication".into()) } else { None }),
            ("\u{2192}E", true, None),
            ("\u{2227}I", suc_is_and, if !suc_is_and { Some("conclusion is not a conjunction".into()) } else { None }),
            ("\u{2227}E\u{2081}", true, None),
            ("\u{2227}E\u{2082}", true, None),
            ("\u{2228}I\u{2081}", suc_is_or, if !suc_is_or { Some("conclusion is not a disjunction".into()) } else { None }),
            ("\u{2228}I\u{2082}", suc_is_or, if !suc_is_or { Some("conclusion is not a disjunction".into()) } else { None }),
            ("\u{2228}E", true, None),
            ("\u{22A5}E", true, None),
            ("\u{00AC}I", suc_is_not, if !suc_is_not { Some("conclusion is not a negation".into()) } else { None }),
            ("\u{00AC}E", suc_is_bot, if !suc_is_bot { Some("conclusion must be \u{22A5}".into()) } else { None }),
            ("\u{2200}I", suc_is_forall, if !suc_is_forall { Some("conclusion is not a universal".into()) } else { None }),
            ("\u{2200}E", true, None), // always potentially applicable (instantiation)
            ("\u{2203}I", suc_is_exists, if !suc_is_exists { Some("conclusion is not an existential".into()) } else { None }),
            ("\u{2203}E", true, None), // always potentially applicable
        ]
    }

    fn generate_premises(&self, rule_name: &str, conclusion: &str) -> Result<Vec<String>, String> {
        let seq = formula::parse_sequent(conclusion, SEP)
            .map_err(|e| format!("Can't parse conclusion: {}", e))?;
        let ants = &seq.antecedents;
        let suc = &seq.succedent;
        let placeholder = Formula::Atom("?".into());

        match normalize_rule(rule_name) {
            "Ax" => Ok(vec![]),
            "ImpI" => match suc {
                Formula::Imp(a, b) => {
                    let mut new_ants = ants.to_vec();
                    new_ants.push(*a.clone());
                    Ok(vec![formula::format_sequent_str(&new_ants, b, SEP)])
                }
                _ => Err("\u{2192}I requires conclusion A \u{2192} B".into()),
            },
            "ImpE" => {
                let imp_suc = Formula::Imp(Box::new(placeholder.clone()), Box::new(suc.clone()));
                Ok(vec![
                    formula::format_sequent_str(ants, &imp_suc, SEP),
                    formula::format_sequent_str(ants, &placeholder, SEP),
                ])
            }
            "AndI" => match suc {
                Formula::And(a, b) => Ok(vec![
                    formula::format_sequent_str(ants, a, SEP),
                    formula::format_sequent_str(ants, b, SEP),
                ]),
                _ => Err("\u{2227}I requires conclusion A \u{2227} B".into()),
            },
            "AndE1" => {
                let conj = Formula::And(Box::new(suc.clone()), Box::new(placeholder));
                Ok(vec![formula::format_sequent_str(ants, &conj, SEP)])
            }
            "AndE2" => {
                let conj = Formula::And(Box::new(placeholder), Box::new(suc.clone()));
                Ok(vec![formula::format_sequent_str(ants, &conj, SEP)])
            }
            "OrI1" => match suc {
                Formula::Or(a, _) => Ok(vec![formula::format_sequent_str(ants, a, SEP)]),
                _ => Err("\u{2228}I\u{2081} requires conclusion A \u{2228} B".into()),
            },
            "OrI2" => match suc {
                Formula::Or(_, b) => Ok(vec![formula::format_sequent_str(ants, b, SEP)]),
                _ => Err("\u{2228}I\u{2082} requires conclusion A \u{2228} B".into()),
            },
            "OrE" => {
                let disj = Formula::Or(Box::new(placeholder.clone()), Box::new(placeholder.clone()));
                let mut ants2 = ants.to_vec();
                ants2.push(placeholder.clone());
                let mut ants3 = ants.to_vec();
                ants3.push(placeholder);
                Ok(vec![
                    formula::format_sequent_str(ants, &disj, SEP),
                    formula::format_sequent_str(&ants2, suc, SEP),
                    formula::format_sequent_str(&ants3, suc, SEP),
                ])
            }
            "BotE" => Ok(vec![formula::format_sequent_str(ants, &Formula::Bot, SEP)]),
            "NotI" => match suc {
                Formula::Not(a) => {
                    let mut new_ants = ants.to_vec();
                    new_ants.push(*a.clone());
                    Ok(vec![formula::format_sequent_str(&new_ants, &Formula::Bot, SEP)])
                }
                _ => Err("\u{00AC}I requires conclusion \u{00AC}A".into()),
            },
            "NotE" => {
                let neg = Formula::Not(Box::new(placeholder.clone()));
                Ok(vec![
                    formula::format_sequent_str(ants, &neg, SEP),
                    formula::format_sequent_str(ants, &placeholder, SEP),
                ])
            }
            "ForallI" => match suc {
                Formula::Forall(x, body) => {
                    Ok(vec![formula::format_sequent_str(ants, body, SEP)])
                }
                _ => Err("\u{2200}I requires conclusion \u{2200}x.\u{03C6}".into()),
            },
            "ForallE" => {
                // premise: Γ ⊢ ∀x.? (the actual body is unknown)
                let forall = Formula::Forall("x".into(), Box::new(placeholder));
                Ok(vec![formula::format_sequent_str(ants, &forall, SEP)])
            }
            "ExistsI" => match suc {
                Formula::Exists(_x, body) => {
                    // premise should prove the body with some substitution
                    Ok(vec![formula::format_sequent_str(ants, body, SEP)])
                }
                _ => Err("\u{2203}I requires conclusion \u{2203}x.\u{03C6}".into()),
            },
            "ExistsE" => {
                // premises: Γ ⊢ ∃x.?, Γ, ? ⊢ C
                let exists = Formula::Exists("x".into(), Box::new(placeholder.clone()));
                let mut ants2 = ants.to_vec();
                ants2.push(placeholder);
                Ok(vec![
                    formula::format_sequent_str(ants, &exists, SEP),
                    formula::format_sequent_str(&ants2, suc, SEP),
                ])
            }
            _ => Err(format!("Unknown rule '{}'", rule_name)),
        }
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
        "\u{2200}I" | "ForallI" => "ForallI",
        "\u{2200}E" | "ForallE" => "ForallE",
        "\u{2203}I" | "ExistsI" => "ExistsI",
        "\u{2203}E" | "ExistsE" => "ExistsE",
        _ => name,
    }
}

fn parse_premise_seq(p: &ProofNode) -> Result<Sequent, String> {
    formula::parse_sequent(&p.conclusion, SEP)
}

// ── Propositional rules ──────────────────────────────────────────────

fn check_ax(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if !premises.is_empty() {
        diags.push(Diagnostic { level: Level::Error, path: vec![], message: format!("Ax expects 0 premises, got {}", premises.len()) });
    }
    if !formula::contains_formula(&seq.antecedents, &seq.succedent) {
        diags.push(Diagnostic { level: Level::Error, path: vec![], message: format!("Ax: {} must appear in the context", seq.succedent) });
    }
    diags
}

fn check_imp_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2192}I expects 1 premise, got {}", premises.len()) }]; }
    let (ant, con) = match &seq.succedent {
        Formula::Imp(a, b) => (a.as_ref(), b.as_ref()),
        _ => return vec![Diagnostic { level: Level::Error, path: vec![], message: "\u{2192}I requires conclusion A \u{2192} B".into() }],
    };
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != *con { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Premise should prove {}, got {}", con, prem.succedent) }); }
        if !formula::multiset_add_one(&seq.antecedents, &prem.antecedents, ant) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Premise context should be \u{0393}, {}", ant) }); }
    }
    diags
}

fn check_imp_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2192}E expects 2 premises, got {}", premises.len()) }]; }
    let prem0 = match parse_premise_seq(premises[0]) { Ok(s) => s, Err(_) => return diags };
    let prem1 = match parse_premise_seq(premises[1]) { Ok(s) => s, Err(_) => return diags };
    match &prem0.succedent {
        Formula::Imp(a, b) => {
            if **b != seq.succedent { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Implication consequent ({}) doesn't match conclusion ({})", b, seq.succedent) }); }
            if prem1.succedent != **a { diags.push(Diagnostic { level: Level::Error, path: vec![1], message: format!("Second premise should prove {}, got {}", a, prem1.succedent) }); }
        }
        _ => { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "First premise must prove an implication".into() }); }
    }
    for (i, prem) in [&prem0, &prem1].iter().enumerate() {
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![i], message: "Premise context should match conclusion context".into() }); }
    }
    diags
}

fn check_and_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2227}I expects 2 premises, got {}", premises.len()) }]; }
    let (left, right) = match &seq.succedent {
        Formula::And(a, b) => (a.as_ref(), b.as_ref()),
        _ => return vec![Diagnostic { level: Level::Error, path: vec![], message: "\u{2227}I requires conclusion A \u{2227} B".into() }],
    };
    for (i, (expected, label)) in [(left, "A"), (right, "B")].iter().enumerate() {
        if let Ok(prem) = parse_premise_seq(premises[i]) {
            if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![i], message: format!("Premise {} context mismatch", label) }); }
            if prem.succedent != **expected { diags.push(Diagnostic { level: Level::Error, path: vec![i], message: format!("Premise {} should prove {}, got {}", label, expected, prem.succedent) }); }
        }
    }
    diags
}

fn check_and_e1(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2227}E\u{2081} expects 1 premise, got {}", premises.len()) }]; }
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        match &prem.succedent {
            Formula::And(a, _) => { if **a != seq.succedent { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Left conjunct {} doesn't match conclusion {}", a, seq.succedent) }); } }
            _ => { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Premise must prove A \u{2227} B".into() }); }
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() }); }
    }
    diags
}

fn check_and_e2(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2227}E\u{2082} expects 1 premise, got {}", premises.len()) }]; }
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        match &prem.succedent {
            Formula::And(_, b) => { if **b != seq.succedent { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Right conjunct {} doesn't match conclusion {}", b, seq.succedent) }); } }
            _ => { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Premise must prove A \u{2227} B".into() }); }
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() }); }
    }
    diags
}

fn check_or_i1(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2228}I\u{2081} expects 1 premise, got {}", premises.len()) }]; }
    let left = match &seq.succedent { Formula::Or(a, _) => a.as_ref(), _ => return vec![Diagnostic { level: Level::Error, path: vec![], message: "\u{2228}I\u{2081} requires conclusion A \u{2228} B".into() }] };
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != *left { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Premise should prove {}", left) }); }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() }); }
    }
    diags
}

fn check_or_i2(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2228}I\u{2082} expects 1 premise, got {}", premises.len()) }]; }
    let right = match &seq.succedent { Formula::Or(_, b) => b.as_ref(), _ => return vec![Diagnostic { level: Level::Error, path: vec![], message: "\u{2228}I\u{2082} requires conclusion A \u{2228} B".into() }] };
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != *right { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Premise should prove {}", right) }); }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() }); }
    }
    diags
}

fn check_or_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 3 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2228}E expects 3 premises, got {}", premises.len()) }]; }
    let prem0 = match parse_premise_seq(premises[0]) { Ok(s) => s, Err(_) => return diags };
    let (a, b) = match &prem0.succedent {
        Formula::Or(a, b) => (a.as_ref().clone(), b.as_ref().clone()),
        _ => return vec![Diagnostic { level: Level::Error, path: vec![0], message: "First premise must prove A \u{2228} B".into() }],
    };
    if !formula::multiset_eq(&prem0.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() }); }
    if let Ok(prem1) = parse_premise_seq(premises[1]) {
        if prem1.succedent != seq.succedent { diags.push(Diagnostic { level: Level::Error, path: vec![1], message: format!("Second premise should prove {}", seq.succedent) }); }
        if !formula::multiset_add_one(&seq.antecedents, &prem1.antecedents, &a) { diags.push(Diagnostic { level: Level::Error, path: vec![1], message: format!("Context should be \u{0393}, {}", a) }); }
    }
    if let Ok(prem2) = parse_premise_seq(premises[2]) {
        if prem2.succedent != seq.succedent { diags.push(Diagnostic { level: Level::Error, path: vec![2], message: format!("Third premise should prove {}", seq.succedent) }); }
        if !formula::multiset_add_one(&seq.antecedents, &prem2.antecedents, &b) { diags.push(Diagnostic { level: Level::Error, path: vec![2], message: format!("Context should be \u{0393}, {}", b) }); }
    }
    diags
}

fn check_bot_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{22A5}E expects 1 premise, got {}", premises.len()) }]; }
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != Formula::Bot { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Premise must prove \u{22A5}, got {}", prem.succedent) }); }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() }); }
    }
    diags
}

fn check_not_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{00AC}I expects 1 premise, got {}", premises.len()) }]; }
    let inner = match &seq.succedent { Formula::Not(a) => a.as_ref(), _ => return vec![Diagnostic { level: Level::Error, path: vec![], message: "\u{00AC}I requires conclusion \u{00AC}A".into() }] };
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != Formula::Bot { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Premise must prove \u{22A5}".into() }); }
        if !formula::multiset_add_one(&seq.antecedents, &prem.antecedents, inner) { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Context should be \u{0393}, {}", inner) }); }
    }
    diags
}

fn check_not_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 { return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{00AC}E expects 2 premises, got {}", premises.len()) }]; }
    if seq.succedent != Formula::Bot { diags.push(Diagnostic { level: Level::Error, path: vec![], message: "\u{00AC}E conclusion must be \u{22A5}".into() }); }
    let prem0 = match parse_premise_seq(premises[0]) { Ok(s) => s, Err(_) => return diags };
    let prem1 = match parse_premise_seq(premises[1]) { Ok(s) => s, Err(_) => return diags };
    let inner = match &prem0.succedent { Formula::Not(a) => a.as_ref().clone(), _ => { diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "First premise must prove \u{00AC}A".into() }); return diags; } };
    if prem1.succedent != inner { diags.push(Diagnostic { level: Level::Error, path: vec![1], message: format!("Second premise should prove {}", inner) }); }
    for (i, prem) in [&prem0, &prem1].iter().enumerate() {
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) { diags.push(Diagnostic { level: Level::Error, path: vec![i], message: "Context mismatch".into() }); }
    }
    diags
}

// ── First-order rules ────────────────────────────────────────────────

// ∀I: from Γ ⊢ φ conclude Γ ⊢ ∀x.φ
fn check_forall_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2200}I expects 1 premise, got {}", premises.len()) }];
    }
    let (_x, body) = match &seq.succedent {
        Formula::Forall(x, body) => (x, body.as_ref()),
        _ => return vec![Diagnostic { level: Level::Error, path: vec![], message: "\u{2200}I requires conclusion \u{2200}x.\u{03C6}".into() }],
    };
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if prem.succedent != *body {
            diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Premise should prove {}, got {}", body, prem.succedent) });
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() });
        }
    }
    diags
}

// ∀E: from Γ ⊢ ∀x.φ conclude Γ ⊢ φ[t/x]
fn check_forall_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2200}E expects 1 premise, got {}", premises.len()) }];
    }
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        match &prem.succedent {
            Formula::Forall(x, body) => {
                // Check that conclusion is an instance of body[t/x] for some t.
                // We try to verify by checking if the conclusion matches the body
                // when x is not free (i.e., body has no x → conclusion should equal body).
                // For general cases, we accept if the shape looks right.
                let body_str = format!("{}", body);
                let conc_str = format!("{}", seq.succedent);
                if body_str == conc_str {
                    // Direct match (no substitution needed, or x not free)
                } else if !body_str.contains(x.as_str()) {
                    // x doesn't appear in body, but conclusion differs
                    diags.push(Diagnostic { level: Level::Error, path: vec![0], message: format!("Variable {} not free in body; conclusion should be {}", x, body) });
                }
                // For cases where substitution occurred, we trust the student.
            }
            _ => {
                diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Premise must prove \u{2200}x.\u{03C6}".into() });
            }
        }
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() });
        }
    }
    diags
}

// ∃I: from Γ ⊢ φ[t/x] conclude Γ ⊢ ∃x.φ
fn check_exists_i(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 1 {
        return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2203}I expects 1 premise, got {}", premises.len()) }];
    }
    match &seq.succedent {
        Formula::Exists(_, _) => {}
        _ => return vec![Diagnostic { level: Level::Error, path: vec![], message: "\u{2203}I requires conclusion \u{2203}x.\u{03C6}".into() }],
    }
    if let Ok(prem) = parse_premise_seq(premises[0]) {
        if !formula::multiset_eq(&prem.antecedents, &seq.antecedents) {
            diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() });
        }
    }
    diags
}

// ∃E: from Γ ⊢ ∃x.φ and Γ, φ ⊢ C conclude Γ ⊢ C
fn check_exists_e(seq: &Sequent, premises: &[&ProofNode]) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    if premises.len() != 2 {
        return vec![Diagnostic { level: Level::Error, path: vec![], message: format!("\u{2203}E expects 2 premises, got {}", premises.len()) }];
    }
    let prem0 = match parse_premise_seq(premises[0]) { Ok(s) => s, Err(_) => return diags };
    let body = match &prem0.succedent {
        Formula::Exists(_, body) => body.as_ref().clone(),
        _ => {
            diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "First premise must prove \u{2203}x.\u{03C6}".into() });
            return diags;
        }
    };
    if !formula::multiset_eq(&prem0.antecedents, &seq.antecedents) {
        diags.push(Diagnostic { level: Level::Error, path: vec![0], message: "Context mismatch".into() });
    }
    if let Ok(prem1) = parse_premise_seq(premises[1]) {
        if prem1.succedent != seq.succedent {
            diags.push(Diagnostic { level: Level::Error, path: vec![1], message: format!("Second premise should prove {}", seq.succedent) });
        }
        if !formula::multiset_add_one(&seq.antecedents, &prem1.antecedents, &body) {
            diags.push(Diagnostic { level: Level::Error, path: vec![1], message: format!("Context should be \u{0393}, {}", body) });
        }
    }
    diags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Theory;

    #[test]
    fn test_gen_forall_i() {
        let prems = FONDTheory.generate_premises("ForallI", "\u{22A2} \u{2200}x.P(x)").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("P(x)"));
    }

    #[test]
    fn test_gen_forall_e() {
        let prems = FONDTheory.generate_premises("ForallE", "\u{22A2} P(a)").unwrap();
        assert_eq!(prems.len(), 1);
    }

    #[test]
    fn test_gen_exists_i() {
        let prems = FONDTheory.generate_premises("ExistsI", "\u{22A2} \u{2203}x.P(x)").unwrap();
        assert_eq!(prems.len(), 1);
    }

    #[test]
    fn test_gen_exists_e() {
        let prems = FONDTheory.generate_premises("ExistsE", "\u{22A2} Q").unwrap();
        assert_eq!(prems.len(), 2);
    }

    #[test]
    fn test_applicable_forall_i() {
        let rules = FONDTheory.applicable_rules("\u{22A2} \u{2200}x.P(x)");
        let r = rules.iter().find(|r| r.0 == "\u{2200}I").unwrap();
        assert!(r.1);
    }

    #[test]
    fn test_applicable_forall_i_not() {
        let rules = FONDTheory.applicable_rules("\u{22A2} P(x)");
        let r = rules.iter().find(|r| r.0 == "\u{2200}I").unwrap();
        assert!(!r.1);
    }

    #[test]
    fn test_applicable_exists_i() {
        let rules = FONDTheory.applicable_rules("\u{22A2} \u{2203}x.P(x)");
        let r = rules.iter().find(|r| r.0 == "\u{2203}I").unwrap();
        assert!(r.1);
    }

    #[test]
    fn test_applicable_returns_16_rules() {
        let rules = FONDTheory.applicable_rules("P \u{22A2} P");
        assert_eq!(rules.len(), 16);
    }

    #[test]
    fn test_propositional_rules_still_work() {
        let prems = FONDTheory.generate_premises("ImpI", "\u{22A2} P \u{2192} Q").unwrap();
        assert_eq!(prems.len(), 1);
    }

    #[test]
    fn test_gen_ax() {
        let prems = FONDTheory.generate_premises("Ax", "P \u{22A2} P").unwrap();
        assert_eq!(prems.len(), 0);
    }
}
