use std::collections::HashSet;

/// Core lambda calculus term representation.
/// Designed for extensibility to typed lambda calculi.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathStep {
    Left,
    Right,
    Body,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Strategy {
    Normal,
    Applicative,
    CallByValue,
    CallByName,
}

impl Term {
    pub fn var(name: &str) -> Term {
        Term::Var(name.to_string())
    }

    pub fn abs(param: &str, body: Term) -> Term {
        Term::Abs(param.to_string(), Box::new(body))
    }

    pub fn app(func: Term, arg: Term) -> Term {
        Term::App(Box::new(func), Box::new(arg))
    }

    /// Set of free variables
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Term::Var(x) => {
                let mut s = HashSet::new();
                s.insert(x.clone());
                s
            }
            Term::Abs(x, body) => {
                let mut fv = body.free_vars();
                fv.remove(x);
                fv
            }
            Term::App(f, a) => {
                let mut fv = f.free_vars();
                fv.extend(a.free_vars());
                fv
            }
        }
    }

    /// All variable names mentioned (free and bound)
    pub fn all_vars(&self) -> HashSet<String> {
        match self {
            Term::Var(x) => {
                let mut s = HashSet::new();
                s.insert(x.clone());
                s
            }
            Term::Abs(x, body) => {
                let mut s = body.all_vars();
                s.insert(x.clone());
                s
            }
            Term::App(f, a) => {
                let mut s = f.all_vars();
                s.extend(a.all_vars());
                s
            }
        }
    }

    /// Generate a fresh variable name not in `avoid`
    pub fn fresh_var(base: &str, avoid: &HashSet<String>) -> String {
        if !avoid.contains(base) {
            return base.to_string();
        }
        let base_stripped = base.trim_end_matches(|c: char| c.is_ascii_digit());
        let mut i = 0u64;
        loop {
            let candidate = format!("{}{}", base_stripped, i);
            if !avoid.contains(&candidate) {
                return candidate;
            }
            i += 1;
        }
    }

    /// Capture-avoiding substitution: self[var := replacement]
    pub fn substitute(&self, var: &str, replacement: &Term) -> Term {
        match self {
            Term::Var(x) => {
                if x == var {
                    replacement.clone()
                } else {
                    self.clone()
                }
            }
            Term::App(f, a) => Term::App(
                Box::new(f.substitute(var, replacement)),
                Box::new(a.substitute(var, replacement)),
            ),
            Term::Abs(x, body) => {
                if x == var {
                    // x shadows var; no substitution inside
                    self.clone()
                } else if replacement.free_vars().contains(x) {
                    // Must alpha-rename to avoid capture
                    let mut avoid = self.all_vars();
                    avoid.extend(replacement.all_vars());
                    avoid.insert(var.to_string());
                    let fresh = Term::fresh_var(x, &avoid);
                    let renamed_body = body.substitute(x, &Term::Var(fresh.clone()));
                    Term::Abs(fresh, Box::new(renamed_body.substitute(var, replacement)))
                } else {
                    Term::Abs(x.clone(), Box::new(body.substitute(var, replacement)))
                }
            }
        }
    }

    /// Is this term a beta-redex? (λx.M) N
    pub fn is_redex(&self) -> bool {
        matches!(self, Term::App(f, _) if matches!(f.as_ref(), Term::Abs(_, _)))
    }

    /// Perform beta reduction: (λx.body) arg → body[x := arg]
    pub fn beta_reduce(&self) -> Option<Term> {
        if let Term::App(f, a) = self {
            if let Term::Abs(x, body) = f.as_ref() {
                return Some(body.substitute(x, a));
            }
        }
        None
    }

    /// Is this term eta-convertible? λx.(M x) where x ∉ FV(M)
    pub fn is_eta_convertible(&self) -> bool {
        if let Term::Abs(x, body) = self {
            if let Term::App(m, arg) = body.as_ref() {
                if let Term::Var(y) = arg.as_ref() {
                    return x == y && !m.free_vars().contains(x);
                }
            }
        }
        false
    }

    /// Perform eta conversion: λx.(M x) → M
    pub fn eta_convert(&self) -> Option<Term> {
        if self.is_eta_convertible() {
            if let Term::Abs(_, body) = self {
                if let Term::App(m, _) = body.as_ref() {
                    return Some(m.as_ref().clone());
                }
            }
        }
        None
    }

    /// Alpha-convert: rename bound variable in a lambda abstraction
    pub fn alpha_convert(&self, new_name: &str) -> Result<Term, String> {
        if let Term::Abs(x, body) = self {
            if x == new_name {
                return Ok(self.clone());
            }
            // new_name must not be free in body (would capture)
            if body.free_vars().contains(new_name) {
                return Err(format!(
                    "Cannot rename {} to {}: the variable {} already appears free in the body, so renaming would capture it (changing the meaning of the term).",
                    x, new_name, new_name
                ));
            }
            let new_body = body.substitute(x, &Term::Var(new_name.to_string()));
            Ok(Term::Abs(new_name.to_string(), Box::new(new_body)))
        } else {
            Err("α-conversion can only be applied to a λ-abstraction (a term of the form (λ (x) body)).".to_string())
        }
    }

    /// Is this a value? (for CBV reduction)
    pub fn is_value(&self) -> bool {
        matches!(self, Term::Abs(_, _) | Term::Var(_))
    }

    /// Count subterms
    pub fn size(&self) -> usize {
        match self {
            Term::Var(_) => 1,
            Term::Abs(_, body) => 1 + body.size(),
            Term::App(f, a) => 1 + f.size() + a.size(),
        }
    }

    /// Get the subterm at a given path
    pub fn at_path(&self, path: &[PathStep]) -> Option<&Term> {
        if path.is_empty() {
            return Some(self);
        }
        match (&path[0], self) {
            (PathStep::Body, Term::Abs(_, body)) => body.at_path(&path[1..]),
            (PathStep::Left, Term::App(f, _)) => f.at_path(&path[1..]),
            (PathStep::Right, Term::App(_, a)) => a.at_path(&path[1..]),
            _ => None,
        }
    }

    /// Apply a transformation at a given path, returning the modified whole term
    pub fn map_at_path<F>(&self, path: &[PathStep], f: F) -> Option<Term>
    where
        F: FnOnce(&Term) -> Option<Term>,
    {
        if path.is_empty() {
            return f(self);
        }
        match (&path[0], self) {
            (PathStep::Body, Term::Abs(x, body)) => {
                let new_body = body.map_at_path(&path[1..], f)?;
                Some(Term::Abs(x.clone(), Box::new(new_body)))
            }
            (PathStep::Left, Term::App(func, arg)) => {
                let new_func = func.map_at_path(&path[1..], f)?;
                Some(Term::App(Box::new(new_func), arg.clone()))
            }
            (PathStep::Right, Term::App(func, arg)) => {
                let new_arg = arg.map_at_path(&path[1..], f)?;
                Some(Term::App(func.clone(), Box::new(new_arg)))
            }
            _ => None,
        }
    }

    /// Find the path to the next redex according to the given strategy
    pub fn next_redex(&self, strategy: &Strategy) -> Option<Vec<PathStep>> {
        match strategy {
            Strategy::Normal => self.next_redex_normal(),
            Strategy::Applicative => self.next_redex_applicative(),
            Strategy::CallByValue => self.next_redex_cbv(),
            Strategy::CallByName => self.next_redex_cbn(),
        }
    }

    /// Normal order: leftmost-outermost redex
    fn next_redex_normal(&self) -> Option<Vec<PathStep>> {
        match self {
            Term::Var(_) => None,
            Term::Abs(_, body) => {
                body.next_redex_normal()
                    .map(|mut p| { p.insert(0, PathStep::Body); p })
            }
            Term::App(f, a) => {
                // If this is a redex, reduce it (outermost first)
                if self.is_redex() {
                    return Some(vec![]);
                }
                // Otherwise try left then right
                f.next_redex_normal()
                    .map(|mut p| { p.insert(0, PathStep::Left); p })
                    .or_else(|| {
                        a.next_redex_normal()
                            .map(|mut p| { p.insert(0, PathStep::Right); p })
                    })
            }
        }
    }

    /// Applicative order: leftmost-innermost redex
    fn next_redex_applicative(&self) -> Option<Vec<PathStep>> {
        match self {
            Term::Var(_) => None,
            Term::Abs(_, body) => {
                body.next_redex_applicative()
                    .map(|mut p| { p.insert(0, PathStep::Body); p })
            }
            Term::App(f, a) => {
                // Try inside func first, then arg, then self
                if let Some(mut p) = f.next_redex_applicative() {
                    p.insert(0, PathStep::Left);
                    return Some(p);
                }
                if let Some(mut p) = a.next_redex_applicative() {
                    p.insert(0, PathStep::Right);
                    return Some(p);
                }
                if self.is_redex() {
                    return Some(vec![]);
                }
                None
            }
        }
    }

    /// Call-by-value: reduce arguments to values before beta-reducing
    fn next_redex_cbv(&self) -> Option<Vec<PathStep>> {
        match self {
            Term::Var(_) => None,
            Term::Abs(_, _) => None, // Don't reduce under lambda
            Term::App(f, a) => {
                // First reduce func to a value
                if !f.is_value() {
                    return f.next_redex_cbv()
                        .map(|mut p| { p.insert(0, PathStep::Left); p });
                }
                // Then reduce arg to a value
                if !a.is_value() {
                    return a.next_redex_cbv()
                        .map(|mut p| { p.insert(0, PathStep::Right); p });
                }
                // Both are values; if this is a redex, reduce
                if self.is_redex() {
                    return Some(vec![]);
                }
                None
            }
        }
    }

    /// Call-by-name: leftmost-outermost, no reductions under lambda
    fn next_redex_cbn(&self) -> Option<Vec<PathStep>> {
        match self {
            Term::Var(_) => None,
            Term::Abs(_, _) => None, // Don't reduce under lambda
            Term::App(f, a) => {
                if self.is_redex() {
                    return Some(vec![]);
                }
                f.next_redex_cbn()
                    .map(|mut p| { p.insert(0, PathStep::Left); p })
                    .or_else(|| {
                        a.next_redex_cbn()
                            .map(|mut p| { p.insert(0, PathStep::Right); p })
                    })
            }
        }
    }

    /// Reduce one step according to the given strategy
    pub fn reduce_step(&self, strategy: &Strategy) -> Option<Term> {
        let path = self.next_redex(strategy)?;
        self.map_at_path(&path, |t| t.beta_reduce())
    }

    /// Is this term in normal form under the given strategy?
    pub fn is_normal_form(&self, strategy: &Strategy) -> bool {
        self.next_redex(strategy).is_none()
    }

    /// Display in CIS352 syntax
    pub fn display_string(&self) -> String {
        match self {
            Term::Var(x) => x.clone(),
            Term::Abs(x, body) => format!("(\u{03bb} ({}) {})", x, body.display_string()),
            Term::App(f, a) => format!("({} {})", f.display_string(), a.display_string()),
        }
    }

    /// Collect all redex paths in the term
    pub fn all_redex_paths(&self, prefix: &[PathStep]) -> Vec<Vec<PathStep>> {
        let mut paths = Vec::new();
        if self.is_redex() {
            paths.push(prefix.to_vec());
        }
        match self {
            Term::Var(_) => {}
            Term::Abs(_, body) => {
                let mut p = prefix.to_vec();
                p.push(PathStep::Body);
                paths.extend(body.all_redex_paths(&p));
            }
            Term::App(f, a) => {
                let mut pl = prefix.to_vec();
                pl.push(PathStep::Left);
                paths.extend(f.all_redex_paths(&pl));

                let mut pr = prefix.to_vec();
                pr.push(PathStep::Right);
                paths.extend(a.all_redex_paths(&pr));
            }
        }
        paths
    }
}

/// Parse a path string like "LBR" into PathSteps
pub fn parse_path(s: &str) -> Result<Vec<PathStep>, String> {
    if s.is_empty() {
        return Ok(vec![]);
    }
    s.chars()
        .map(|c| match c {
            'L' => Ok(PathStep::Left),
            'R' => Ok(PathStep::Right),
            'B' => Ok(PathStep::Body),
            _ => Err(format!("Invalid path character: {}", c)),
        })
        .collect()
}

/// Encode a path as a string
pub fn encode_path(path: &[PathStep]) -> String {
    path.iter()
        .map(|s| match s {
            PathStep::Left => 'L',
            PathStep::Right => 'R',
            PathStep::Body => 'B',
        })
        .collect()
}

impl Strategy {
    pub fn from_str(s: &str) -> Option<Strategy> {
        match s {
            "normal" => Some(Strategy::Normal),
            "applicative" => Some(Strategy::Applicative),
            "cbv" => Some(Strategy::CallByValue),
            "cbn" => Some(Strategy::CallByName),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Strategy::Normal => "Normal Order",
            Strategy::Applicative => "Applicative Order",
            Strategy::CallByValue => "Call-by-Value",
            Strategy::CallByName => "Call-by-Name",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helpers ──────────────────────────────────────────────────

    /// Reduce term to normal form under a strategy, returning each step.
    /// Panics if it doesn't reach normal form within `limit` steps.
    fn reduce_to_nf(term: &Term, strategy: &Strategy, limit: usize) -> (Term, Vec<Term>) {
        let mut current = term.clone();
        let mut steps = vec![current.clone()];
        for _ in 0..limit {
            match current.reduce_step(strategy) {
                Some(next) => { current = next; steps.push(current.clone()); }
                None => return (current, steps),
            }
        }
        panic!(
            "Did not reach normal form within {} steps. Last term: {}",
            limit, current.display_string()
        );
    }

    /// Parse a CIS352-syntax term (convenience for integration-style tests).
    fn p(input: &str) -> Term {
        crate::parser::parse(input).unwrap_or_else(|e| panic!("Parse error on {:?}: {}", input, e))
    }

    // ── Free Variables ──────────────────────────────────────────

    #[test]
    fn free_vars_bound() {
        let t = Term::abs("x", Term::app(Term::var("x"), Term::var("y")));
        let fv = t.free_vars();
        assert!(fv.contains("y"));
        assert!(!fv.contains("x"));
    }

    #[test]
    fn free_vars_closed_term() {
        // (λx. x) has no free variables
        assert!(Term::abs("x", Term::var("x")).free_vars().is_empty());
    }

    #[test]
    fn free_vars_nested_shadow() {
        // (λx. (λx. x)) — inner x shadows outer, no free vars
        let t = Term::abs("x", Term::abs("x", Term::var("x")));
        assert!(t.free_vars().is_empty());
    }

    #[test]
    fn free_vars_multiple() {
        // (x (y z)) — all three are free
        let t = Term::app(Term::var("x"), Term::app(Term::var("y"), Term::var("z")));
        let fv = t.free_vars();
        assert_eq!(fv.len(), 3);
        assert!(fv.contains("x") && fv.contains("y") && fv.contains("z"));
    }

    // ── Substitution ────────────────────────────────────────────

    #[test]
    fn substitute_simple() {
        // x[x := y] → y
        assert_eq!(Term::var("x").substitute("x", &Term::var("y")), Term::var("y"));
    }

    #[test]
    fn substitute_different_var() {
        // z[x := y] → z
        assert_eq!(Term::var("z").substitute("x", &Term::var("y")), Term::var("z"));
    }

    #[test]
    fn substitute_under_lambda_shadowed() {
        // (λx. x)[x := y] → (λx. x) — x is shadowed
        let t = Term::abs("x", Term::var("x"));
        assert_eq!(t.substitute("x", &Term::var("y")), t);
    }

    #[test]
    fn substitute_under_lambda_not_shadowed() {
        // (λz. x)[x := y] → (λz. y)
        let t = Term::abs("z", Term::var("x"));
        let result = t.substitute("x", &Term::var("y"));
        assert_eq!(result, Term::abs("z", Term::var("y")));
    }

    #[test]
    fn substitute_capture_avoiding() {
        // (λy. x)[x := y] should alpha-rename to avoid capture
        let t = Term::abs("y", Term::var("x"));
        let result = t.substitute("x", &Term::var("y"));
        if let Term::Abs(param, body) = &result {
            assert_ne!(param, "y", "must rename to avoid capture");
            assert_eq!(body.as_ref(), &Term::var("y"), "substituted value is y");
        } else {
            panic!("Expected Abs, got {:?}", result);
        }
    }

    #[test]
    fn substitute_in_app() {
        // (x x)[x := y] → (y y)
        let t = Term::app(Term::var("x"), Term::var("x"));
        let result = t.substitute("x", &Term::var("y"));
        assert_eq!(result, Term::app(Term::var("y"), Term::var("y")));
    }

    // ── Beta Reduction ──────────────────────────────────────────

    #[test]
    fn beta_identity() {
        // (λx. x) y → y
        let t = Term::app(Term::abs("x", Term::var("x")), Term::var("y"));
        assert_eq!(t.beta_reduce().unwrap(), Term::var("y"));
    }

    #[test]
    fn beta_constant() {
        // (λx. z) y → z
        let t = Term::app(Term::abs("x", Term::var("z")), Term::var("y"));
        assert_eq!(t.beta_reduce().unwrap(), Term::var("z"));
    }

    #[test]
    fn beta_self_application() {
        // (λx. (x x)) y → (y y)
        let t = Term::app(
            Term::abs("x", Term::app(Term::var("x"), Term::var("x"))),
            Term::var("y"),
        );
        assert_eq!(
            t.beta_reduce().unwrap(),
            Term::app(Term::var("y"), Term::var("y"))
        );
    }

    #[test]
    fn beta_not_redex() {
        // (x y) is not a redex
        assert!(Term::app(Term::var("x"), Term::var("y")).beta_reduce().is_none());
    }

    #[test]
    fn beta_capture_avoiding() {
        // (λx. (λy. x)) y → (λy'. y), NOT (λy. y)
        let t = Term::app(
            Term::abs("x", Term::abs("y", Term::var("x"))),
            Term::var("y"),
        );
        let result = t.beta_reduce().unwrap();
        if let Term::Abs(param, body) = &result {
            assert_ne!(param, "y", "must rename to avoid capture");
            assert_eq!(body.as_ref(), &Term::var("y"));
        } else {
            panic!("Expected Abs, got {:?}", result);
        }
    }

    // ── Eta Conversion ──────────────────────────────────────────

    #[test]
    fn eta_simple() {
        // λx.(f x) → f
        let t = Term::abs("x", Term::app(Term::var("f"), Term::var("x")));
        assert_eq!(t.eta_convert().unwrap(), Term::var("f"));
    }

    #[test]
    fn eta_not_convertible_free_var() {
        // λx.(x x) — x ∈ FV(x), cannot eta-convert
        let t = Term::abs("x", Term::app(Term::var("x"), Term::var("x")));
        assert!(t.eta_convert().is_none());
        assert!(!t.is_eta_convertible());
    }

    #[test]
    fn eta_not_convertible_wrong_arg() {
        // λx.(f y) — argument is y not x
        let t = Term::abs("x", Term::app(Term::var("f"), Term::var("y")));
        assert!(t.eta_convert().is_none());
    }

    #[test]
    fn eta_not_abs() {
        // (f x) is not an abstraction
        assert!(Term::app(Term::var("f"), Term::var("x")).eta_convert().is_none());
    }

    // ── Alpha Conversion ────────────────────────────────────────

    #[test]
    fn alpha_simple() {
        let t = Term::abs("x", Term::var("x"));
        assert_eq!(t.alpha_convert("z").unwrap(), Term::abs("z", Term::var("z")));
    }

    #[test]
    fn alpha_same_name() {
        let t = Term::abs("x", Term::var("x"));
        assert_eq!(t.alpha_convert("x").unwrap(), t);
    }

    #[test]
    fn alpha_capture_rejected() {
        // (λy. (x y)) — renaming y to x would capture the free x
        let t = Term::abs("y", Term::app(Term::var("x"), Term::var("y")));
        let err = t.alpha_convert("x").unwrap_err();
        assert!(err.contains("capture"), "Error should mention capture: {}", err);
    }

    #[test]
    fn alpha_on_non_abs() {
        let err = Term::var("x").alpha_convert("y").unwrap_err();
        assert!(err.contains("λ-abstraction"), "Error: {}", err);
    }

    #[test]
    fn alpha_in_nested_body() {
        // (λx. (λy. (x y))) — rename x to z → (λz. (λy. (z y)))
        let t = Term::abs("x", Term::abs("y", Term::app(Term::var("x"), Term::var("y"))));
        let result = t.alpha_convert("z").unwrap();
        assert_eq!(
            result,
            Term::abs("z", Term::abs("y", Term::app(Term::var("z"), Term::var("y"))))
        );
    }

    // ── Strategies: next_redex ───────────────────────────────────

    // Test term: (λx.x) ((λy.y) z)   — two redexes: root and Right
    fn two_redex_term() -> Term {
        Term::app(
            Term::abs("x", Term::var("x")),
            Term::app(Term::abs("y", Term::var("y")), Term::var("z")),
        )
    }

    #[test]
    fn normal_picks_outermost() {
        let path = two_redex_term().next_redex(&Strategy::Normal).unwrap();
        assert_eq!(path, vec![], "Normal order: root (outermost) redex");
    }

    #[test]
    fn applicative_picks_innermost() {
        let path = two_redex_term().next_redex(&Strategy::Applicative).unwrap();
        assert_eq!(path, vec![PathStep::Right], "Applicative: inner redex first");
    }

    #[test]
    fn cbv_picks_argument_first() {
        // (λx.x) ((λy.y) z): func is value (Abs), arg is not value (App/redex)
        let path = two_redex_term().next_redex(&Strategy::CallByValue).unwrap();
        assert_eq!(path, vec![PathStep::Right], "CBV: reduce arg to value first");
    }

    #[test]
    fn cbn_picks_outermost_no_under_lambda() {
        let path = two_redex_term().next_redex(&Strategy::CallByName).unwrap();
        assert_eq!(path, vec![], "CBN: outermost redex (like normal, but no under-lambda)");
    }

    #[test]
    fn normal_under_lambda() {
        // λx. ((λy.y) z) — redex is inside a lambda body
        let t = Term::abs("x", Term::app(Term::abs("y", Term::var("y")), Term::var("z")));
        let path = t.next_redex(&Strategy::Normal).unwrap();
        assert_eq!(path, vec![PathStep::Body]);
    }

    #[test]
    fn cbv_no_under_lambda() {
        // λx. ((λy.y) z) — CBV does NOT reduce under lambda
        let t = Term::abs("x", Term::app(Term::abs("y", Term::var("y")), Term::var("z")));
        assert!(t.next_redex(&Strategy::CallByValue).is_none());
    }

    #[test]
    fn cbn_no_under_lambda() {
        let t = Term::abs("x", Term::app(Term::abs("y", Term::var("y")), Term::var("z")));
        assert!(t.next_redex(&Strategy::CallByName).is_none());
    }

    // ── Root-level redex (the empty-string-is-falsy bug) ────────

    #[test]
    fn root_redex_returns_empty_path() {
        // ((λy. body) w) — the whole term is a redex, path = []
        let t = Term::app(
            Term::abs("y", Term::app(Term::abs("a", Term::var("a")), Term::var("z"))),
            Term::var("w"),
        );
        let path = t.next_redex(&Strategy::Normal).unwrap();
        assert_eq!(path, vec![], "Root-level redex path must be empty vec");
        assert_eq!(encode_path(&path), "", "Encoded root path is empty string");
    }

    #[test]
    fn root_redex_is_not_normal_form() {
        // A root-level redex is NOT in normal form
        let t = Term::app(
            Term::abs("y", Term::var("z")),
            Term::var("w"),
        );
        assert!(!t.is_normal_form(&Strategy::Normal));
        assert!(!t.is_normal_form(&Strategy::Applicative));
        assert!(!t.is_normal_form(&Strategy::CallByValue));
        assert!(!t.is_normal_form(&Strategy::CallByName));
    }

    // ── Multi-step reduction to normal form ─────────────────────

    #[test]
    fn reduce_identity_app() {
        // (λx.x) y →* y
        let t = p("((λ (x) x) y)");
        let (nf, steps) = reduce_to_nf(&t, &Strategy::Normal, 10);
        assert_eq!(nf, Term::var("y"));
        assert_eq!(steps.len(), 2); // original + 1 step
    }

    #[test]
    fn reduce_skk_to_identity() {
        // S K K →* λz. z  (S = λxyz.xz(yz), K = λxy.x)
        let t = p("(((\u{03bb} (x y z) ((x z) (y z))) (\u{03bb} (x y) x)) (\u{03bb} (x y) x))");
        let (nf, _) = reduce_to_nf(&t, &Strategy::Normal, 20);
        // The result should be an identity function: λz. z (parameter name may vary)
        if let Term::Abs(param, body) = &nf {
            assert_eq!(body.as_ref(), &Term::Var(param.clone()), "S K K should reduce to I");
        } else {
            panic!("S K K should reduce to an abstraction, got: {}", nf.display_string());
        }
    }

    #[test]
    fn reduce_church_rosser_demo_term() {
        // This is the exact term used in the Church-Rosser tutorial demo.
        // It triggered the empty-string-path bug because the second step
        // has a root-level redex (path "").
        let t = p("((\u{03bb} (x y) x) ((\u{03bb} (a) a) z) w)");

        // Normal order
        let (nf_normal, steps_n) = reduce_to_nf(&t, &Strategy::Normal, 10);
        assert_eq!(nf_normal.display_string(), "z", "Normal order should reach z");
        assert!(steps_n.len() > 2, "Should take more than 1 step (was the bug)");

        // Applicative order
        let (nf_app, _) = reduce_to_nf(&t, &Strategy::Applicative, 10);
        assert_eq!(nf_app.display_string(), "z", "Applicative should also reach z");

        // Church-Rosser: both strategies reach the same normal form
        assert_eq!(nf_normal, nf_app, "Church-Rosser: normal form must be the same");
    }

    #[test]
    fn church_rosser_property() {
        // Test Church-Rosser on several terms with all four strategies.
        let terms = vec![
            "((λ (x) x) y)",
            "((λ (x) (x x)) y)",
            "((λ (x y) x) a b)",
            "((λ (f x) (f (f x))) (λ (y) y) z)",
        ];
        for input in &terms {
            let t = p(input);
            let results: Vec<_> = [
                Strategy::Normal, Strategy::Applicative,
                Strategy::CallByValue, Strategy::CallByName,
            ].iter().filter_map(|s| {
                // Some strategies may not reach NF (e.g., stuck terms in CBV/CBN)
                let mut cur = t.clone();
                for _ in 0..50 {
                    match cur.reduce_step(s) {
                        Some(next) => cur = next,
                        None => return Some(cur.display_string()),
                    }
                }
                None // didn't terminate
            }).collect();

            // All strategies that terminated should agree
            if results.len() > 1 {
                for r in &results[1..] {
                    assert_eq!(
                        r, &results[0],
                        "Church-Rosser violated for {:?}: got {:?} and {:?}",
                        input, results[0], r
                    );
                }
            }
        }
    }

    #[test]
    fn omega_diverges() {
        // Ω = (λx. x x)(λx. x x) reduces to itself
        let t = p("((\u{03bb} (x) (x x)) (\u{03bb} (x) (x x)))");
        let next = t.reduce_step(&Strategy::Normal).unwrap();
        assert_eq!(t, next, "Omega must reduce to itself");
    }

    #[test]
    fn omega_root_redex() {
        let t = p("((\u{03bb} (x) (x x)) (\u{03bb} (x) (x x)))");
        let path = t.next_redex(&Strategy::Normal).unwrap();
        assert_eq!(path, vec![], "Omega's redex is at the root");
    }

    // ── Capture-avoiding substitution through full reduction ────

    #[test]
    fn capture_avoiding_through_beta() {
        // ((λx. (λy. x)) y) → (λy'. y)  NOT (λy. y)
        let t = p("((\u{03bb} (x) (\u{03bb} (y) x)) y)");
        let (nf, _) = reduce_to_nf(&t, &Strategy::Normal, 10);
        // The result should be λ<fresh>. y, where <fresh> ≠ y
        if let Term::Abs(param, body) = &nf {
            assert_ne!(param, "y", "Must alpha-rename to avoid capture");
            assert_eq!(body.as_ref(), &Term::var("y"), "Body should be free y");
        } else {
            panic!("Expected Abs, got: {}", nf.display_string());
        }
    }

    // ── Normal form detection ───────────────────────────────────

    #[test]
    fn variable_is_normal_form() {
        let t = Term::var("x");
        assert!(t.is_normal_form(&Strategy::Normal));
        assert!(t.is_normal_form(&Strategy::Applicative));
        assert!(t.is_normal_form(&Strategy::CallByValue));
        assert!(t.is_normal_form(&Strategy::CallByName));
    }

    #[test]
    fn lambda_with_no_redex_is_nf() {
        assert!(Term::abs("x", Term::var("x")).is_normal_form(&Strategy::Normal));
    }

    #[test]
    fn redex_is_not_nf() {
        let t = Term::app(Term::abs("x", Term::var("x")), Term::var("y"));
        assert!(!t.is_normal_form(&Strategy::Normal));
    }

    #[test]
    fn cbv_normal_form_with_redex_under_lambda() {
        // λx. ((λy.y) z) — has a redex under λ, but CBV won't reduce it
        let t = Term::abs("x", Term::app(Term::abs("y", Term::var("y")), Term::var("z")));
        assert!(!t.is_normal_form(&Strategy::Normal), "Not NF in normal order");
        assert!(t.is_normal_form(&Strategy::CallByValue), "IS NF in CBV");
        assert!(t.is_normal_form(&Strategy::CallByName), "IS NF in CBN");
    }

    // ── Path encoding / decoding ────────────────────────────────

    #[test]
    fn path_roundtrip() {
        let path = vec![PathStep::Left, PathStep::Body, PathStep::Right];
        assert_eq!(encode_path(&path), "LBR");
        assert_eq!(parse_path("LBR").unwrap(), path);
    }

    #[test]
    fn empty_path_roundtrip() {
        assert_eq!(encode_path(&[]), "");
        assert_eq!(parse_path("").unwrap(), vec![]);
    }

    #[test]
    fn parse_path_invalid() {
        assert!(parse_path("X").is_err());
    }

    // ── map_at_path ─────────────────────────────────────────────

    #[test]
    fn map_at_path_root() {
        let t = Term::var("x");
        let result = t.map_at_path(&[], |_| Some(Term::var("y")));
        assert_eq!(result, Some(Term::var("y")));
    }

    #[test]
    fn map_at_path_nested() {
        // (f x) — replace x (Right child) with z
        let t = Term::app(Term::var("f"), Term::var("x"));
        let result = t.map_at_path(&[PathStep::Right], |_| Some(Term::var("z")));
        assert_eq!(result, Some(Term::app(Term::var("f"), Term::var("z"))));
    }

    #[test]
    fn map_at_path_invalid() {
        // Try to go Left on a Var — should return None
        assert!(Term::var("x").map_at_path(&[PathStep::Left], |_| Some(Term::var("y"))).is_none());
    }

    // ── size ────────────────────────────────────────────────────

    #[test]
    fn size_var() { assert_eq!(Term::var("x").size(), 1); }

    #[test]
    fn size_abs() {
        assert_eq!(Term::abs("x", Term::var("x")).size(), 2);
    }

    #[test]
    fn size_app() {
        assert_eq!(Term::app(Term::var("f"), Term::var("x")).size(), 3);
    }

    // ── display_string ──────────────────────────────────────────

    #[test]
    fn display_var() {
        assert_eq!(Term::var("x").display_string(), "x");
    }

    #[test]
    fn display_abs() {
        assert_eq!(
            Term::abs("x", Term::var("x")).display_string(),
            "(\u{03bb} (x) x)"
        );
    }

    #[test]
    fn display_app() {
        assert_eq!(
            Term::app(Term::var("f"), Term::var("x")).display_string(),
            "(f x)"
        );
    }

    // ── all_redex_paths ─────────────────────────────────────────

    #[test]
    fn all_redex_paths_two_redexes() {
        let t = two_redex_term();
        let paths = t.all_redex_paths(&[]);
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&vec![]));             // root
        assert!(paths.contains(&vec![PathStep::Right])); // inner
    }

    #[test]
    fn all_redex_paths_no_redex() {
        assert!(Term::var("x").all_redex_paths(&[]).is_empty());
    }

    // ── Strategy::from_str ──────────────────────────────────────

    #[test]
    fn strategy_from_str() {
        assert_eq!(Strategy::from_str("normal"), Some(Strategy::Normal));
        assert_eq!(Strategy::from_str("applicative"), Some(Strategy::Applicative));
        assert_eq!(Strategy::from_str("cbv"), Some(Strategy::CallByValue));
        assert_eq!(Strategy::from_str("cbn"), Some(Strategy::CallByName));
        assert_eq!(Strategy::from_str("bogus"), None);
    }

    // ── is_value (for CBV) ──────────────────────────────────────

    #[test]
    fn is_value_var() { assert!(Term::var("x").is_value()); }

    #[test]
    fn is_value_abs() { assert!(Term::abs("x", Term::var("x")).is_value()); }

    #[test]
    fn is_value_app() { assert!(!Term::app(Term::var("f"), Term::var("x")).is_value()); }
}
