use crate::tree::ProofNode;
use serde::{Deserialize, Serialize};

/// Severity level for a diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Error,
    Incomplete,
    Valid,
}

/// A single diagnostic message attached to a proof tree node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: Level,
    pub path: Vec<usize>,
    pub message: String,
}

/// The result of checking an entire proof tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub valid: bool,
    pub complete: bool,
    pub diagnostics: Vec<Diagnostic>,
}

/// Trait that each theory (logic) must implement.
/// This is the extension point: implement this for propositional logic,
/// first-order logic, type systems, big-step semantics, etc.
pub trait Theory {
    /// Human-readable name of this theory.
    fn name(&self) -> &str;

    /// List of known rule names in this theory.
    fn known_rules(&self) -> Vec<&str>;

    /// Check a single node with a rule applied.
    /// Called only when the node has a non-empty conclusion and a rule name.
    /// Should return diagnostics for this node (not recursive — the framework recurses).
    fn check_rule(
        &self,
        rule_name: &str,
        conclusion: &str,
        premises: &[&ProofNode],
    ) -> Vec<Diagnostic>;

    /// Returns true if the given string looks like a judgement in this theory
    /// (as opposed to a side condition). Used to determine if a bare leaf needs a rule.
    fn is_judgement(&self, s: &str) -> bool;
}

/// Check an entire proof tree against a theory.
pub fn check_tree(root: &ProofNode, theory: &dyn Theory) -> CheckResult {
    let mut diagnostics = Vec::new();
    check_node(root, theory, &mut diagnostics, &mut vec![]);

    let has_error = diagnostics.iter().any(|d| d.level == Level::Error);
    let has_incomplete = diagnostics.iter().any(|d| d.level == Level::Incomplete);

    CheckResult {
        valid: !has_error && !has_incomplete,
        complete: !has_incomplete,
        diagnostics,
    }
}

fn check_node(
    node: &ProofNode,
    theory: &dyn Theory,
    diagnostics: &mut Vec<Diagnostic>,
    path: &mut Vec<usize>,
) {
    let current_path = path.clone();

    // 1. Empty conclusion
    if node.conclusion.trim().is_empty() {
        diagnostics.push(Diagnostic {
            level: Level::Incomplete,
            path: current_path,
            message: "Not yet filled in".to_string(),
        });
        return;
    }

    // 2. Leaf with no rule
    if node.rule_name.is_none() && node.premises.is_empty() {
        if theory.is_judgement(&node.conclusion) {
            diagnostics.push(Diagnostic {
                level: Level::Incomplete,
                path: current_path,
                message: "This judgement needs a rule applied".to_string(),
            });
        } else {
            // Side condition — parent will validate it
            diagnostics.push(Diagnostic {
                level: Level::Valid,
                path: current_path,
                message: "Side condition".to_string(),
            });
        }
        return;
    }

    // 3. Has premises but no rule name
    if node.rule_name.is_none() && !node.premises.is_empty() {
        diagnostics.push(Diagnostic {
            level: Level::Incomplete,
            path: current_path.clone(),
            message: "This node needs a rule name".to_string(),
        });
        // Still recurse into premises
        for (i, premise) in node.premises.iter().enumerate() {
            path.push(i);
            check_node(premise, theory, diagnostics, path);
            path.pop();
        }
        return;
    }

    // 4. Has rule name — check if empty
    let rule_name = node.rule_name.as_deref().unwrap();
    if rule_name.trim().is_empty() {
        diagnostics.push(Diagnostic {
            level: Level::Incomplete,
            path: current_path.clone(),
            message: "Name the rule".to_string(),
        });
        for (i, premise) in node.premises.iter().enumerate() {
            path.push(i);
            check_node(premise, theory, diagnostics, path);
            path.pop();
        }
        return;
    }

    // 5. Unknown rule
    let known = theory.known_rules();
    if !known.iter().any(|r| *r == rule_name) {
        diagnostics.push(Diagnostic {
            level: Level::Error,
            path: current_path.clone(),
            message: format!(
                "Unknown rule '{}'. Valid rules: {}",
                rule_name,
                known.join(", ")
            ),
        });
        for (i, premise) in node.premises.iter().enumerate() {
            path.push(i);
            check_node(premise, theory, diagnostics, path);
            path.pop();
        }
        return;
    }

    // 6. Delegate to theory for rule-specific checking
    let premise_refs: Vec<&ProofNode> = node.premises.iter().collect();
    let mut rule_diags = theory.check_rule(rule_name, &node.conclusion, &premise_refs);

    // Fix up paths: the theory returns diagnostics with empty paths,
    // we need to prefix them with the current path
    for d in &mut rule_diags {
        if d.path.is_empty() {
            d.path = current_path.clone();
        } else {
            let mut full_path = current_path.clone();
            full_path.extend_from_slice(&d.path);
            d.path = full_path;
        }
    }
    diagnostics.extend(rule_diags);

    // 7. Recurse into premises
    for (i, premise) in node.premises.iter().enumerate() {
        path.push(i);
        check_node(premise, theory, diagnostics, path);
        path.pop();
    }
}
