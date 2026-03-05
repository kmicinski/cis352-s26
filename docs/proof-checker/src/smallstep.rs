/// Small-step operational semantics.
///
/// Judgement format: e ⟶ e' (using ⟶ U+27F6, or --> as ASCII)
///
/// Rules:
///   Beta      — ((λ (x) e) v) ⟶ e[x := v] (0 premises)
///   App-L     — from e₁ ⟶ e₁' conclude (e₁ e₂) ⟶ (e₁' e₂) (1 premise)
///   App-R     — from e₂ ⟶ e₂' conclude (v e₂) ⟶ (v e₂') (1 premise)
///   Add-L     — from e₁ ⟶ e₁' conclude (+ e₁ e₂) ⟶ (+ e₁' e₂) (1 premise)
///   Add-R     — from e₂ ⟶ e₂' conclude (+ v e₂) ⟶ (+ v e₂') (1 premise)
///   Add       — (+ i₁ i₂) ⟶ i₃ where i₃ = i₁ + i₂ (0 premises, side condition)
///   Neg-Step  — from e ⟶ e' conclude (- e) ⟶ (- e') (1 premise)
///   Neg       — (- i) ⟶ -i (0 premises, side condition)
///   If0-Step  — from e ⟶ e' conclude (if0 e et ef) ⟶ (if0 e' et ef) (1 premise)
///   If0-True  — (if0 0 et ef) ⟶ et (0 premises)
///   If0-False — (if0 i et ef) ⟶ ef where i ≠ 0 (0 premises, side condition)
///   Let-Step  — from e ⟶ e' conclude (let ([x e]) eb) ⟶ (let ([x e']) eb) (1 premise)
///   Let       — (let ([x v]) eb) ⟶ eb[x := v] (0 premises)

use crate::check::{Diagnostic, Level, Theory};
use crate::tree::ProofNode;

pub struct SmallStepTheory;

impl Theory for SmallStepTheory {
    fn name(&self) -> &str {
        "Small-Step Operational Semantics"
    }

    fn known_rules(&self) -> Vec<&str> {
        vec![
            "Beta", "\u{03B2}", "beta",
            "App-L", "App-R",
            "Add-L", "Add-R", "Add",
            "Neg-Step", "Neg",
            "If0-Step", "If0-True", "If0-False",
            "Let-Step", "Let",
        ]
    }

    fn check_rule(
        &self,
        rule_name: &str,
        conclusion: &str,
        premises: &[&ProofNode],
    ) -> Vec<Diagnostic> {
        // Validate judgement format
        if !is_reduction(conclusion) {
            return vec![Diagnostic {
                level: Level::Error,
                path: vec![],
                message: "Conclusion must be a reduction judgement: e \u{27F6} e'".to_string(),
            }];
        }

        let normalized = normalize_rule(rule_name);
        let expected = expected_premises(normalized);

        let mut diags = Vec::new();

        if let Some(exp) = expected {
            if premises.len() != exp {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: format!(
                        "{} expects {} premise(s), got {}",
                        rule_name, exp, premises.len()
                    ),
                });
                return diags;
            }
        }

        // For rules with a premise, check that the premise is also a reduction
        for (i, p) in premises.iter().enumerate() {
            let conc = p.conclusion.trim();
            if !conc.is_empty() && !is_reduction(conc) && !is_side_condition(conc) {
                diags.push(Diagnostic {
                    level: Level::Error,
                    path: vec![i],
                    message: "Premise should be a reduction judgement (e \u{27F6} e') or side condition".to_string(),
                });
            }
        }

        diags
    }

    fn is_judgement(&self, s: &str) -> bool {
        is_reduction(s)
    }
}

fn normalize_rule(name: &str) -> &str {
    match name {
        "Beta" | "\u{03B2}" | "beta" => "Beta",
        _ => name,
    }
}

fn expected_premises(rule: &str) -> Option<usize> {
    match rule {
        "Beta" => Some(0),
        "App-L" | "App-R" => Some(1),
        "Add-L" | "Add-R" => Some(1),
        "Add" => None, // 0 or 1 (side condition)
        "Neg-Step" => Some(1),
        "Neg" => None,
        "If0-Step" => Some(1),
        "If0-True" => Some(0),
        "If0-False" => None,
        "Let-Step" => Some(1),
        "Let" => Some(0),
        _ => None,
    }
}

fn is_reduction(s: &str) -> bool {
    s.contains('\u{27F6}') || s.contains("-->")
}

fn is_side_condition(s: &str) -> bool {
    let s = s.trim();
    // Things like "i ≠ 0" or "3 + 5 = 8"
    s.contains('=') || s.contains('\u{2260}')
}
