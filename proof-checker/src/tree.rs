use serde::{Deserialize, Serialize};

/// A node in a proof tree. This is the generic structure shared by all theories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofNode {
    pub conclusion: String,
    pub rule_name: Option<String>,
    pub premises: Vec<ProofNode>,
}

impl ProofNode {
    pub fn is_leaf(&self) -> bool {
        self.premises.is_empty() && self.rule_name.is_none()
    }
}
