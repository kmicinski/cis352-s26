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
        // A judgement contains ⊢ and ⇓ (or their ASCII equivalents |- and ==>)
        (s.contains('⊢') || s.contains("|-")) && (s.contains('⇓') || s.contains("==>"))
    }

    fn applicable_rules(&self, conclusion: &str) -> Vec<(&str, bool, Option<String>)> {
        let j = match parse::parse_judgement(conclusion) {
            Ok(j) => j,
            Err(_) => {
                return self.known_rules().into_iter().map(|r| (r, true, None)).collect();
            }
        };
        use ast::Expr;
        vec![
            ("Var", matches!(&j.expr, Expr::Var(_)),
             if !matches!(&j.expr, Expr::Var(_)) { Some(format!("expression is {}, not a variable", j.expr.form_name())) } else { None }),
            ("Int", matches!(&j.expr, Expr::Int(_)),
             if !matches!(&j.expr, Expr::Int(_)) { Some(format!("expression is {}, not an integer", j.expr.form_name())) } else { None }),
            ("Neg", matches!(&j.expr, Expr::Neg(_)),
             if !matches!(&j.expr, Expr::Neg(_)) { Some(format!("expression is {}, not (- e)", j.expr.form_name())) } else { None }),
            ("Add", matches!(&j.expr, Expr::Add(_, _)),
             if !matches!(&j.expr, Expr::Add(_, _)) { Some(format!("expression is {}, not (+ e₁ e₂)", j.expr.form_name())) } else { None }),
            ("If0-True", matches!(&j.expr, Expr::If0(_, _, _)),
             if !matches!(&j.expr, Expr::If0(_, _, _)) { Some(format!("expression is {}, not (if0 ...)", j.expr.form_name())) } else { None }),
            ("If0-False", matches!(&j.expr, Expr::If0(_, _, _)),
             if !matches!(&j.expr, Expr::If0(_, _, _)) { Some(format!("expression is {}, not (if0 ...)", j.expr.form_name())) } else { None }),
            ("Let", matches!(&j.expr, Expr::Let(_, _, _)),
             if !matches!(&j.expr, Expr::Let(_, _, _)) { Some(format!("expression is {}, not (let ...)", j.expr.form_name())) } else { None }),
            ("Lam", matches!(&j.expr, Expr::Lam(_, _)),
             if !matches!(&j.expr, Expr::Lam(_, _)) { Some(format!("expression is {}, not (λ ...)", j.expr.form_name())) } else { None }),
            ("App", matches!(&j.expr, Expr::App(_, _)),
             if !matches!(&j.expr, Expr::App(_, _)) { Some(format!("expression is {}, not (e₁ e₂)", j.expr.form_name())) } else { None }),
        ]
    }

    fn generate_premises(&self, rule_name: &str, conclusion: &str) -> Result<Vec<String>, String> {
        use ast::{Expr, format_env};

        let j = parse::parse_judgement(conclusion).ok();
        let env_str = j.as_ref().map(|j| format_env(&j.env)).unwrap_or_else(|| {
            conclusion.split('\u{22A2}').next().unwrap_or("{}").trim().to_string()
        });
        let base = format!("{} \u{22A2} ? \u{21D3} ?", env_str);

        match rule_name {
            "Int" | "Lam" => Ok(vec![]),
            "Var" => {
                if let Some(ref j) = j {
                    // Side condition: ρ(x) = v
                    if let Expr::Var(x) = &j.expr {
                        Ok(vec![format!("{}({}) = {}", env_str, x, j.value)])
                    } else {
                        Ok(vec!["".to_string()])
                    }
                } else {
                    Ok(vec!["".to_string()])
                }
            }
            "Neg" => {
                if let Some(ref j) = j {
                    if let Expr::Neg(e) = &j.expr {
                        Ok(vec![
                            format!("{} \u{22A2} {} \u{21D3} ?", env_str, e),
                            format!("{} = -?", j.value),
                        ])
                    } else {
                        Ok(vec![base.clone(), "".to_string()])
                    }
                } else {
                    Ok(vec![base.clone(), "".to_string()])
                }
            }
            "Add" => {
                if let Some(ref j) = j {
                    if let Expr::Add(e1, e2) = &j.expr {
                        Ok(vec![
                            format!("{} \u{22A2} {} \u{21D3} ?", env_str, e1),
                            format!("{} \u{22A2} {} \u{21D3} ?", env_str, e2),
                            format!("{} = ? + ?", j.value),
                        ])
                    } else {
                        Ok(vec![base.clone(), base.clone(), "".to_string()])
                    }
                } else {
                    Ok(vec![base.clone(), base.clone(), "".to_string()])
                }
            }
            "If0-True" => {
                if let Some(ref j) = j {
                    if let Expr::If0(eg, et, _ef) = &j.expr {
                        Ok(vec![
                            format!("{} \u{22A2} {} \u{21D3} 0", env_str, eg),
                            format!("{} \u{22A2} {} \u{21D3} {}", env_str, et, j.value),
                        ])
                    } else {
                        Ok(vec![base.clone(), base.clone()])
                    }
                } else {
                    Ok(vec![base.clone(), base.clone()])
                }
            }
            "If0-False" => {
                if let Some(ref j) = j {
                    if let Expr::If0(eg, _et, ef) = &j.expr {
                        Ok(vec![
                            format!("{} \u{22A2} {} \u{21D3} ?", env_str, eg),
                            "? \u{2260} 0".to_string(),
                            format!("{} \u{22A2} {} \u{21D3} {}", env_str, ef, j.value),
                        ])
                    } else {
                        Ok(vec![base.clone(), "? \u{2260} 0".to_string(), base.clone()])
                    }
                } else {
                    Ok(vec![base.clone(), "? \u{2260} 0".to_string(), base.clone()])
                }
            }
            "Let" => {
                if let Some(ref j) = j {
                    if let Expr::Let(x, e1, e2) = &j.expr {
                        Ok(vec![
                            format!("{} \u{22A2} {} \u{21D3} ?", env_str, e1),
                            format!("{{..., {} \u{21A6} ?}} \u{22A2} {} \u{21D3} {}", x, e2, j.value),
                        ])
                    } else {
                        Ok(vec![base.clone(), base.clone()])
                    }
                } else {
                    Ok(vec![base.clone(), base.clone()])
                }
            }
            "App" => {
                if let Some(ref j) = j {
                    if let Expr::App(e1, e2) = &j.expr {
                        Ok(vec![
                            format!("{} \u{22A2} {} \u{21D3} ?", env_str, e1),
                            format!("{} \u{22A2} {} \u{21D3} ?", env_str, e2),
                            format!("? \u{22A2} ? \u{21D3} {}", j.value),
                        ])
                    } else {
                        Ok(vec![base.clone(), base.clone(), base.clone()])
                    }
                } else {
                    Ok(vec![base.clone(), base.clone(), base.clone()])
                }
            }
            _ => Err(format!("Unknown big-step rule '{}'", rule_name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Theory;

    #[test]
    fn test_gen_int() {
        let prems = BigStepTheory.generate_premises("Int", "{} \u{22A2} 42 \u{21D3} 42").unwrap();
        assert_eq!(prems.len(), 0);
    }

    #[test]
    fn test_gen_add() {
        let prems = BigStepTheory.generate_premises("Add", "{} \u{22A2} (+ 1 2) \u{21D3} 3").unwrap();
        assert_eq!(prems.len(), 3);
        assert_eq!(prems[0], "{} \u{22A2} 1 \u{21D3} ?");
        assert_eq!(prems[1], "{} \u{22A2} 2 \u{21D3} ?");
        assert_eq!(prems[2], "3 = ? + ?");
    }

    #[test]
    fn test_gen_neg() {
        let prems = BigStepTheory.generate_premises("Neg", "{} \u{22A2} (- x) \u{21D3} -3").unwrap();
        assert_eq!(prems.len(), 2);
        assert_eq!(prems[0], "{} \u{22A2} x \u{21D3} ?");
        assert_eq!(prems[1], "-3 = -?");
    }

    #[test]
    fn test_gen_var() {
        let prems = BigStepTheory.generate_premises("Var", "{x \u{21A6} 3} \u{22A2} x \u{21D3} 3").unwrap();
        assert_eq!(prems.len(), 1);
        assert!(prems[0].contains("(x)"));
        assert!(prems[0].contains("= 3"));
    }

    #[test]
    fn test_gen_if0_true() {
        let prems = BigStepTheory.generate_premises("If0-True", "{} \u{22A2} (if0 0 1 2) \u{21D3} 1").unwrap();
        assert_eq!(prems.len(), 2);
        assert_eq!(prems[0], "{} \u{22A2} 0 \u{21D3} 0");
        assert_eq!(prems[1], "{} \u{22A2} 1 \u{21D3} 1");
    }

    #[test]
    fn test_gen_if0_false() {
        let prems = BigStepTheory.generate_premises("If0-False", "{} \u{22A2} (if0 1 2 3) \u{21D3} 3").unwrap();
        assert_eq!(prems.len(), 3);
        assert_eq!(prems[0], "{} \u{22A2} 1 \u{21D3} ?");
        assert_eq!(prems[1], "? \u{2260} 0");
        assert_eq!(prems[2], "{} \u{22A2} 3 \u{21D3} 3");
    }

    #[test]
    fn test_gen_let() {
        let prems = BigStepTheory.generate_premises("Let", "{} \u{22A2} (let ([x 5]) (+ x x)) \u{21D3} 10").unwrap();
        assert_eq!(prems.len(), 2);
        assert_eq!(prems[0], "{} \u{22A2} 5 \u{21D3} ?");
        assert!(prems[1].contains("x \u{21A6} ?"));
        assert!(prems[1].contains("(+ x x)"));
        assert!(prems[1].contains("10"));
    }

    #[test]
    fn test_gen_app() {
        let prems = BigStepTheory.generate_premises("App", "{} \u{22A2} (f 5) \u{21D3} 42").unwrap();
        assert_eq!(prems.len(), 3);
        assert_eq!(prems[0], "{} \u{22A2} f \u{21D3} ?");
        assert_eq!(prems[1], "{} \u{22A2} 5 \u{21D3} ?");
        assert_eq!(prems[2], "? \u{22A2} ? \u{21D3} 42");
    }

    #[test]
    fn test_gen_app_unparseable_value() {
        // When value can't be parsed, falls back to placeholder
        let prems = BigStepTheory.generate_premises("App", "{} \u{22A2} (f x) \u{21D3} v").unwrap();
        assert_eq!(prems.len(), 3);
        assert!(prems[0].contains("\u{22A2}"));
    }

    #[test]
    fn test_gen_lam() {
        let prems = BigStepTheory.generate_premises("Lam", "{} \u{22A2} (\u{03BB} (x) x) \u{21D3} \u{27E8}\u{03BB} (x) x , {}\u{27E9}").unwrap();
        assert_eq!(prems.len(), 0);
    }

    #[test]
    fn test_gen_unknown() {
        let result = BigStepTheory.generate_premises("Unknown", "{} \u{22A2} e \u{21D3} v");
        assert!(result.is_err());
    }
}
