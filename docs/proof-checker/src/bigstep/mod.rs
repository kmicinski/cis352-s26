pub mod ast;
pub mod parse;
pub mod rules;

use crate::check::{Diagnostic, Level, Theory};
use crate::tree::ProofNode;

/// Big-step operational semantics theory.
/// Handles the 9 inference rules: Var, Int, Neg, Add, If0-True, If0-False, Let, Lam, App.
pub struct BigStepTheory;

impl Theory for BigStepTheory {
    fn name(&self) -> &str {
        "Big-Step Operational Semantics"
    }

    fn known_rules(&self) -> Vec<&str> {
        vec![
            "Var", "Int", "Neg", "Add", "If0-True", "If0-False", "Let", "Lam", "App",
        ]
    }

    fn check_rule(
        &self,
        rule_name: &str,
        conclusion: &str,
        premises: &[&ProofNode],
    ) -> Vec<Diagnostic> {
        // Parse the conclusion as a judgement
        let judgement = match parse::parse_judgement(conclusion) {
            Ok(j) => j,
            Err(e) => {
                return vec![Diagnostic {
                    level: Level::Error,
                    path: vec![],
                    message: format!("Can't parse conclusion as ρ ⊢ e ⇓ v: {}", e),
                }];
            }
        };

        rules::check_rule(rule_name, &judgement, premises)
    }

    fn is_judgement(&self, s: &str) -> bool {
        // A judgement contains ⊢ and ⇓
        s.contains('⊢') && s.contains('⇓')
    }
}
