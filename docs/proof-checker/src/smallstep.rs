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

    fn applicable_rules(&self, conclusion: &str) -> Vec<(&str, bool, Option<String>)> {
        if !is_reduction(conclusion) {
            return self.known_rules().into_iter().map(|r| (r, true, None)).collect();
        }
        // Extract the LHS expression (before ⟶)
        let lhs = conclusion.split('\u{27F6}').next()
            .or_else(|| conclusion.split("-->").next())
            .unwrap_or("").trim();

        let parts = crate::types::split_sexpr(lhs);
        let head = parts.as_ref().and_then(|p| p.first().map(|s| s.as_str()));

        let is_app = parts.as_ref().map_or(false, |p| p.len() == 2 && !crate::stlc::is_keyword(Some(&p[0])));
        let is_add = head == Some("+");
        let is_neg = head == Some("-");
        let is_if0 = head == Some("if0");
        let is_let = head == Some("let");

        vec![
            ("Beta",      is_app,  if !is_app  { Some("expression is not an application".into()) } else { None }),
            ("App-L",     is_app,  if !is_app  { Some("expression is not an application".into()) } else { None }),
            ("App-R",     is_app,  if !is_app  { Some("expression is not an application".into()) } else { None }),
            ("Add-L",     is_add,  if !is_add  { Some("expression is not (+ ...)".into()) } else { None }),
            ("Add-R",     is_add,  if !is_add  { Some("expression is not (+ ...)".into()) } else { None }),
            ("Add",       is_add,  if !is_add  { Some("expression is not (+ ...)".into()) } else { None }),
            ("Neg-Step",  is_neg,  if !is_neg  { Some("expression is not (- ...)".into()) } else { None }),
            ("Neg",       is_neg,  if !is_neg  { Some("expression is not (- ...)".into()) } else { None }),
            ("If0-Step",  is_if0,  if !is_if0  { Some("expression is not (if0 ...)".into()) } else { None }),
            ("If0-True",  is_if0,  if !is_if0  { Some("expression is not (if0 ...)".into()) } else { None }),
            ("If0-False", is_if0,  if !is_if0  { Some("expression is not (if0 ...)".into()) } else { None }),
            ("Let-Step",  is_let,  if !is_let  { Some("expression is not (let ...)".into()) } else { None }),
            ("Let",       is_let,  if !is_let  { Some("expression is not (let ...)".into()) } else { None }),
        ]
    }

    fn generate_premises(&self, rule_name: &str, conclusion: &str) -> Result<Vec<String>, String> {
        let _ = conclusion; // not needed for structure
        let base = "? \u{27F6} ?".to_string();

        match normalize_rule(rule_name) {
            "Beta" | "Add" | "Neg" | "If0-True" | "If0-False" | "Let" => Ok(vec![]),
            "App-L" | "App-R" | "Add-L" | "Add-R" | "Neg-Step" | "If0-Step" | "Let-Step" => {
                Ok(vec![base])
            }
            _ => Err(format!("Unknown small-step rule '{}'", rule_name)),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Theory;

    #[test]
    fn test_gen_beta() {
        let prems = SmallStepTheory.generate_premises("Beta", "e \u{27F6} e'").unwrap();
        assert_eq!(prems.len(), 0);
    }

    #[test]
    fn test_gen_app_l() {
        let prems = SmallStepTheory.generate_premises("App-L", "e \u{27F6} e'").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("\u{27F6}"));
    }

    #[test]
    fn test_gen_add_step() {
        let prems = SmallStepTheory.generate_premises("Add-L", "e \u{27F6} e'").unwrap();
        assert_eq!(prems.len(), 1);
    }

    #[test]
    fn test_gen_unknown() {
        let result = SmallStepTheory.generate_premises("Unknown", "e \u{27F6} e'");
        assert!(result.is_err());
    }
}
