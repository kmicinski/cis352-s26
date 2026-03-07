# Proof Checker

A Rust-to-WASM proof tree verifier for CIS352. Students build interactive proof trees in the browser and the checker validates each inference step.

## Architecture

```
proof-checker/          Rust crate, compiled to WASM
  src/
    lib.rs              WASM entry points (check_proof, generate_premises, applicable_rules, etc.)
    sexp.rs             S-expression parser: text ↔ ProofNode tree
    tree.rs             ProofNode data structure
    check.rs            Theory trait + generic tree walker (check_tree)
    formula.rs          Propositional/first-order formula parser & utilities
    types.rs            Type parser for STLC/System F judgements
    bigstep.rs          Big-step operational semantics theory
    smallstep.rs        Small-step operational semantics theory
    g3ip.rs             G3ip intuitionistic sequent calculus theory
    propnd.rs           Propositional natural deduction theory
    fond.rs             First-order natural deduction theory (extends propnd with quantifiers)
    stlc.rs             Simply-typed lambda calculus theory
    systemf.rs          System F theory (extends stlc with type abstraction/application)
  tests/
    integration_tests.rs  85 tests: one per playground example + larger proofs + error cases + WASM API
```

## The Theory Trait (`check.rs`)

Every proof system implements `Theory`:

```rust
pub trait Theory {
    fn name(&self) -> &str;
    fn known_rules(&self) -> &[&str];
    fn is_judgement(&self, s: &str) -> bool;
    fn check_rule(&self, rule: &str, conclusion: &str, side_conditions: &[&str],
                  premises: &[&ProofNode]) -> Vec<Diagnostic>;
    fn generate_premises(&self, rule: &str, conclusion: &str) -> Result<Vec<String>, String>;
    fn applicable_rules(&self, conclusion: &str) -> Vec<(&str, bool, Option<String>)>;
}
```

`check_tree` walks the proof tree top-down, calling `check_rule` at each node. It collects diagnostics (errors, valid markers, incomplete markers) with paths indicating which node produced them.

## Data Flow

```
Browser (proof-tree.js)
  │
  │  getSexp() → S-expression string
  │
  ▼
proof-checker-glue.js
  │
  │  calls WASM: check_proof(sexp, theory) → JSON
  │  calls WASM: generate_premises(conclusion, rule, theory) → JSON
  │  calls WASM: applicable_rules(conclusion, theory) → JSON
  │
  ▼
lib.rs (WASM entry points)
  │
  │  parses S-expression → ProofNode tree
  │  dispatches to appropriate Theory implementation
  │
  ▼
check_tree(&node, &theory) → CheckResult { valid, complete, diagnostics }
```

## Key Types

- **`ProofNode`** — tree node with `rule: Option<String>`, `conclusion: String`, `side_conditions: Vec<String>`, `children: Vec<ProofNode>`
- **`CheckResult`** — `{ valid: bool, complete: bool, diagnostics: Vec<Diagnostic> }`
- **`Diagnostic`** — `{ level: Level, path: Vec<usize>, message: String }` where path identifies the tree node

## Theories

| ID | Module | Judgement Form | Rules |
|----|--------|---------------|-------|
| `big-step` | `bigstep.rs` | `ρ ⊢ e ⇓ v` | Int, Var, Lam, Add, Neg, App, If0-True, If0-False, Let |
| `small-step` | `smallstep.rs` | `e ⟶ e'` | Add, Neg, Beta, Add-L, Add-R, Neg-Step, App-L, App-R, If0-True, If0-False, If0-Step, Let-Step, Let |
| `g3ip` | `g3ip.rs` | `Γ ⇒ C` | Ax, ⊥L, ⊤R, ∧R, ∧L, ∨R₁, ∨R₂, ∨L, →R, →L |
| `propnd` | `propnd.rs` | `Γ ⊢ A` | Ax, →I, →E, ∧I, ∧E₁, ∧E₂, ∨I₁, ∨I₂, ∨E, ⊥E, ¬I, ¬E |
| `fond` | `fond.rs` | `Γ ⊢ A` | (all propnd rules) + ∀I, ∀E, ∃I, ∃E |
| `stlc` | `stlc.rs` | `Γ ⊢ e : τ` | T-Var, T-Int, T-Bool, T-Lam, T-App, T-Add, T-Neg, T-If, T-Let |
| `systemf` | `systemf.rs` | `Γ ⊢ e : τ` | (all stlc rules) + T-TyLam, T-TyApp |

## Building

```bash
# Run tests
cargo test

# Build WASM (output to ../assets/wasm/)
wasm-pack build --target web --out-dir ../assets/wasm
```

## Browser Integration

Three JS files work together:

1. **`proof-tree.js`** — Editor widget: renders proof trees, handles user interaction, produces S-expressions
2. **`proof-checker-glue.js`** — Loads WASM, provides `ProofChecker.check()`, `ProofChecker.theoryConfig()`, etc.
3. **`proof-tree.css`** — Styling for proof trees, check annotations, viewport controls

Usage in a page:

```html
<script src="proof-tree.js"></script>
<script src="proof-checker-glue.js"></script>
<script>
  var editor = ProofTree.createEditor(container,
    Object.assign({}, ProofChecker.theoryConfig('big-step'), {
      initialSexp: '(--- "")'
    })
  );
  ProofChecker.createCheckButton(container, editor, 'big-step');
</script>
```

## Testing

- **145 unit tests** across all modules (parsers, rule checkers, premise generators)
- **85 integration tests** covering every playground example for all 7 theories, complex proofs, error cases, and WASM API endpoints
- Every example shown in the playground has a corresponding regression test
