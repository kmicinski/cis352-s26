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
    pub fn alpha_convert(&self, new_name: &str) -> Option<Term> {
        if let Term::Abs(x, body) = self {
            if x == new_name {
                return Some(self.clone());
            }
            // new_name must not be free in body (would capture)
            if body.free_vars().contains(new_name) {
                return None;
            }
            let new_body = body.substitute(x, &Term::Var(new_name.to_string()));
            Some(Term::Abs(new_name.to_string(), Box::new(new_body)))
        } else {
            None
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

    #[test]
    fn test_free_vars() {
        let t = Term::abs("x", Term::app(Term::var("x"), Term::var("y")));
        let fv = t.free_vars();
        assert!(fv.contains("y"));
        assert!(!fv.contains("x"));
    }

    #[test]
    fn test_substitute_no_capture() {
        // (λy. x)[x := y] should alpha-rename to avoid capture
        let t = Term::abs("y", Term::var("x"));
        let result = t.substitute("x", &Term::var("y"));
        // Result should be λy0. y (or similar fresh name), NOT λy. y
        if let Term::Abs(param, body) = &result {
            assert_ne!(param, "y"); // must have been renamed
            if let Term::Var(v) = body.as_ref() {
                assert_eq!(v, "y"); // the substituted value
            } else {
                panic!("Expected Var");
            }
        } else {
            panic!("Expected Abs");
        }
    }

    #[test]
    fn test_beta_reduce() {
        // (λx.x) y → y
        let t = Term::app(Term::abs("x", Term::var("x")), Term::var("y"));
        let result = t.beta_reduce().unwrap();
        assert_eq!(result, Term::var("y"));
    }

    #[test]
    fn test_eta_convert() {
        // λx.(f x) → f  (when x ∉ FV(f))
        let t = Term::abs("x", Term::app(Term::var("f"), Term::var("x")));
        let result = t.eta_convert().unwrap();
        assert_eq!(result, Term::var("f"));
    }

    #[test]
    fn test_eta_not_convertible() {
        // λx.(x x) — NOT eta-convertible (x ∈ FV(x))
        let t = Term::abs("x", Term::app(Term::var("x"), Term::var("x")));
        assert!(!t.is_eta_convertible());
    }

    #[test]
    fn test_alpha_convert() {
        let t = Term::abs("x", Term::var("x"));
        let result = t.alpha_convert("z").unwrap();
        assert_eq!(result, Term::abs("z", Term::var("z")));
    }

    #[test]
    fn test_normal_order() {
        // (λx.x) ((λy.y) z) — should reduce outermost first
        let inner = Term::app(Term::abs("y", Term::var("y")), Term::var("z"));
        let t = Term::app(Term::abs("x", Term::var("x")), inner);
        let path = t.next_redex(&Strategy::Normal).unwrap();
        assert_eq!(path, vec![]); // root is the redex
    }

    #[test]
    fn test_applicative_order() {
        // (λx.x) ((λy.y) z) — should reduce innermost first
        let inner = Term::app(Term::abs("y", Term::var("y")), Term::var("z"));
        let t = Term::app(Term::abs("x", Term::var("x")), inner);
        let path = t.next_redex(&Strategy::Applicative).unwrap();
        assert_eq!(path, vec![PathStep::Right]); // inner redex first
    }
}
