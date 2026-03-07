/// Propositional formula AST, parser, and display.
/// Shared by G3ip (sequent calculus) and PropND (natural deduction).

use std::fmt;

/// Propositional formula.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Formula {
    Atom(String),
    Bot,
    Top,
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Imp(Box<Formula>, Box<Formula>),
    Forall(String, Box<Formula>),
    Exists(String, Box<Formula>),
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Atom(s) => write!(f, "{}", s),
            Formula::Bot => write!(f, "\u{22A5}"),
            Formula::Top => write!(f, "\u{22A4}"),
            Formula::Not(a) => {
                match a.as_ref() {
                    Formula::Atom(_) | Formula::Bot | Formula::Top => write!(f, "\u{00AC}{}", a),
                    _ => write!(f, "\u{00AC}({})", a),
                }
            }
            Formula::And(a, b) => {
                let ls = paren_if_lower(a, 3);
                let rs = paren_if_lower(b, 3);
                write!(f, "{} \u{2227} {}", ls, rs)
            }
            Formula::Or(a, b) => {
                let ls = paren_if_lower(a, 2);
                let rs = paren_if_lower(b, 2);
                write!(f, "{} \u{2228} {}", ls, rs)
            }
            Formula::Imp(a, b) => {
                let ls = paren_if_lower(a, 2);
                write!(f, "{} \u{2192} {}", ls, b)
            }
            Formula::Forall(x, body) => write!(f, "\u{2200}{}.{}", x, body),
            Formula::Exists(x, body) => write!(f, "\u{2203}{}.{}", x, body),
        }
    }
}

fn precedence(f: &Formula) -> u8 {
    match f {
        Formula::Forall(_, _) | Formula::Exists(_, _) => 0,
        Formula::Imp(_, _) => 1,
        Formula::Or(_, _) => 2,
        Formula::And(_, _) => 3,
        Formula::Not(_) => 4,
        _ => 5,
    }
}

fn paren_if_lower(f: &Formula, min_prec: u8) -> String {
    if precedence(f) < min_prec {
        format!("({})", f)
    } else {
        format!("{}", f)
    }
}

/// A sequent: antecedents => succedent.
#[derive(Debug, Clone)]
pub struct Sequent {
    pub antecedents: Vec<Formula>,
    pub succedent: Formula,
}

// ── Tokenizer ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum FToken {
    Atom(String),
    Var(String),
    Bot,
    Top,
    Not,
    And,
    Or,
    Imp,
    Forall,
    Exists,
    Dot,
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Result<Vec<FToken>, String> {
    let chars: Vec<char> = s.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        match chars[i] {
            '(' => { tokens.push(FToken::LParen); i += 1; }
            ')' => { tokens.push(FToken::RParen); i += 1; }
            '\u{22A5}' => { tokens.push(FToken::Bot); i += 1; } // ⊥
            '\u{22A4}' => { tokens.push(FToken::Top); i += 1; } // ⊤
            '\u{00AC}' => { tokens.push(FToken::Not); i += 1; } // ¬
            '~' => { tokens.push(FToken::Not); i += 1; }
            '\u{2227}' => { tokens.push(FToken::And); i += 1; } // ∧
            '\u{2228}' => { tokens.push(FToken::Or); i += 1; }  // ∨
            '\u{2192}' => { tokens.push(FToken::Imp); i += 1; } // →
            '\u{2200}' => { tokens.push(FToken::Forall); i += 1; } // ∀
            '\u{2203}' => { tokens.push(FToken::Exists); i += 1; } // ∃
            '.' => { tokens.push(FToken::Dot); i += 1; }
            '-' if i + 1 < chars.len() && chars[i + 1] == '>' => {
                tokens.push(FToken::Imp);
                i += 2;
            }
            '/' if i + 1 < chars.len() && chars[i + 1] == '\\' => {
                tokens.push(FToken::And);
                i += 2;
            }
            '\\' if i + 1 < chars.len() && chars[i + 1] == '/' => {
                tokens.push(FToken::Or);
                i += 2;
            }
            c if c.is_alphabetic() && c.is_uppercase() => {
                let mut name = String::new();
                name.push(c);
                i += 1;
                // Allow subscript digits and primes
                while i < chars.len() {
                    let nc = chars[i];
                    if nc.is_alphanumeric()
                        || nc == '\''
                        || nc == '_'
                        || ('\u{2080}'..='\u{2089}').contains(&nc)
                        || ('\u{2090}'..='\u{209C}').contains(&nc)
                    {
                        name.push(nc);
                        i += 1;
                    } else {
                        break;
                    }
                }
                // Allow parenthesized arguments: P(x), Q(x, y)
                if i < chars.len() && chars[i] == '(' {
                    name.push('(');
                    i += 1;
                    let mut depth = 1;
                    while i < chars.len() && depth > 0 {
                        if chars[i] == '(' { depth += 1; }
                        if chars[i] == ')' { depth -= 1; }
                        name.push(chars[i]);
                        i += 1;
                    }
                }
                tokens.push(FToken::Atom(name));
            }
            c if c.is_alphabetic() && c.is_lowercase() => {
                let mut name = String::new();
                name.push(c);
                i += 1;
                while i < chars.len() {
                    let nc = chars[i];
                    if nc.is_alphanumeric() || nc == '\'' || nc == '_' {
                        name.push(nc);
                        i += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(FToken::Var(name));
            }
            c => {
                return Err(format!("Unexpected character '{}' in formula", c));
            }
        }
    }

    Ok(tokens)
}

// ── Parser (recursive descent) ───────────────────────────────────────

/// Parse a formula string.
pub fn parse_formula(s: &str) -> Result<Formula, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("Empty formula".to_string());
    }
    let tokens = tokenize(s)?;
    let mut pos = 0;
    let f = parse_quantifier_or_imp(&tokens, &mut pos)?;
    if pos < tokens.len() {
        return Err(format!("Unexpected token {:?} after formula", tokens[pos]));
    }
    Ok(f)
}

// Quantifiers bind the entire remaining formula (lowest precedence)
fn parse_quantifier_or_imp(tokens: &[FToken], pos: &mut usize) -> Result<Formula, String> {
    if *pos < tokens.len() {
        match &tokens[*pos] {
            FToken::Forall => {
                *pos += 1;
                let var = match tokens.get(*pos) {
                    Some(FToken::Var(v)) => { let v = v.clone(); *pos += 1; v }
                    Some(FToken::Atom(v)) => { let v = v.clone(); *pos += 1; v }
                    _ => return Err("Expected variable after \u{2200}".to_string()),
                };
                if tokens.get(*pos) == Some(&FToken::Dot) { *pos += 1; }
                let body = parse_quantifier_or_imp(tokens, pos)?;
                Ok(Formula::Forall(var, Box::new(body)))
            }
            FToken::Exists => {
                *pos += 1;
                let var = match tokens.get(*pos) {
                    Some(FToken::Var(v)) => { let v = v.clone(); *pos += 1; v }
                    Some(FToken::Atom(v)) => { let v = v.clone(); *pos += 1; v }
                    _ => return Err("Expected variable after \u{2203}".to_string()),
                };
                if tokens.get(*pos) == Some(&FToken::Dot) { *pos += 1; }
                let body = parse_quantifier_or_imp(tokens, pos)?;
                Ok(Formula::Exists(var, Box::new(body)))
            }
            _ => parse_imp(tokens, pos),
        }
    } else {
        parse_imp(tokens, pos)
    }
}

// → is right-associative
fn parse_imp(tokens: &[FToken], pos: &mut usize) -> Result<Formula, String> {
    let left = parse_or(tokens, pos)?;
    if *pos < tokens.len() && tokens[*pos] == FToken::Imp {
        *pos += 1;
        // Allow quantifiers on the RHS of →: P → ∀x.Q(x)
        let right = parse_quantifier_or_imp(tokens, pos)?;
        Ok(Formula::Imp(Box::new(left), Box::new(right)))
    } else {
        Ok(left)
    }
}

// ∨ is left-associative
fn parse_or(tokens: &[FToken], pos: &mut usize) -> Result<Formula, String> {
    let mut left = parse_and(tokens, pos)?;
    while *pos < tokens.len() && tokens[*pos] == FToken::Or {
        *pos += 1;
        let right = parse_and(tokens, pos)?;
        left = Formula::Or(Box::new(left), Box::new(right));
    }
    Ok(left)
}

// ∧ is left-associative
fn parse_and(tokens: &[FToken], pos: &mut usize) -> Result<Formula, String> {
    let mut left = parse_unary(tokens, pos)?;
    while *pos < tokens.len() && tokens[*pos] == FToken::And {
        *pos += 1;
        let right = parse_unary(tokens, pos)?;
        left = Formula::And(Box::new(left), Box::new(right));
    }
    Ok(left)
}

// ¬ prefix
fn parse_unary(tokens: &[FToken], pos: &mut usize) -> Result<Formula, String> {
    if *pos < tokens.len() && tokens[*pos] == FToken::Not {
        *pos += 1;
        let inner = parse_unary(tokens, pos)?;
        Ok(Formula::Not(Box::new(inner)))
    } else {
        parse_primary(tokens, pos)
    }
}

// Atoms, variables, ⊥, ⊤, parenthesized formulas
fn parse_primary(tokens: &[FToken], pos: &mut usize) -> Result<Formula, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of formula".to_string());
    }
    match &tokens[*pos] {
        FToken::Atom(s) => {
            let f = Formula::Atom(s.clone());
            *pos += 1;
            Ok(f)
        }
        FToken::Var(s) => {
            // Standalone variable in formula position — treat as atom
            let f = Formula::Atom(s.clone());
            *pos += 1;
            Ok(f)
        }
        FToken::Bot => { *pos += 1; Ok(Formula::Bot) }
        FToken::Top => { *pos += 1; Ok(Formula::Top) }
        FToken::LParen => {
            *pos += 1;
            let f = parse_quantifier_or_imp(tokens, pos)?;
            if *pos >= tokens.len() || tokens[*pos] != FToken::RParen {
                return Err("Missing closing parenthesis".to_string());
            }
            *pos += 1;
            Ok(f)
        }
        other => Err(format!("Expected formula, got {:?}", other)),
    }
}

// ── Sequent parsing ──────────────────────────────────────────────────

/// Parse a sequent of the form "A, B, C ⇒ D" or "⇒ D".
/// `sep` is the separator character (⇒ for sequent calculus, ⊢ for ND).
pub fn parse_sequent(s: &str, sep: char) -> Result<Sequent, String> {
    let s = s.trim();
    let sep_pos = s
        .find(sep)
        .ok_or_else(|| format!("Missing '{}' in sequent/judgement", sep))?;
    let left = s[..sep_pos].trim();
    let right = s[sep_pos + sep.len_utf8()..].trim();

    let antecedents = if left.is_empty() {
        vec![]
    } else {
        parse_formula_list(left)?
    };

    let succedent = parse_formula(right)?;

    Ok(Sequent { antecedents, succedent })
}

/// Parse a comma-separated list of formulas.
pub fn parse_formula_list(s: &str) -> Result<Vec<Formula>, String> {
    let parts = split_top_level_commas(s);
    let mut formulas = Vec::new();
    for part in &parts {
        let part = part.trim();
        if !part.is_empty() {
            formulas.push(parse_formula(part)?);
        }
    }
    Ok(formulas)
}

/// Split on commas at nesting depth 0 (respecting parentheses).
fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    let mut byte_pos = 0;

    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&s[start..byte_pos]);
                start = byte_pos + ch.len_utf8();
            }
            _ => {}
        }
        byte_pos += ch.len_utf8();
    }
    parts.push(&s[start..]);
    parts
}

// ── Multiset utilities ───────────────────────────────────────────────

/// Check if two multisets of formulas are equal (order-independent).
pub fn multiset_eq(a: &[Formula], b: &[Formula]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.sort();
    b.sort();
    a == b
}

/// Check if `big` equals `small` union {extra}.
pub fn multiset_add_one(small: &[Formula], big: &[Formula], extra: &Formula) -> bool {
    let mut expected = small.to_vec();
    expected.push(extra.clone());
    multiset_eq(&expected, big)
}

/// Check if `result` equals `original` with one copy of `old` replaced by `new_f`.
pub fn multiset_replace_one(
    original: &[Formula],
    result: &[Formula],
    old: &Formula,
    new_f: &Formula,
) -> bool {
    let mut modified = original.to_vec();
    if let Some(idx) = modified.iter().position(|f| f == old) {
        modified.remove(idx);
        modified.push(new_f.clone());
        multiset_eq(&modified, result)
    } else {
        false
    }
}

/// Check if `result` equals `original` with one copy of `old` removed.
pub fn multiset_remove_one(original: &[Formula], result: &[Formula], removed: &Formula) -> bool {
    let mut modified = original.to_vec();
    if let Some(idx) = modified.iter().position(|f| f == removed) {
        modified.remove(idx);
        multiset_eq(&modified, result)
    } else {
        false
    }
}

/// Check if `formula` is a member of the list.
pub fn contains_formula(list: &[Formula], f: &Formula) -> bool {
    list.iter().any(|x| x == f)
}

/// Format a list of formulas as a comma-separated string.
pub fn format_formula_list(formulas: &[Formula]) -> String {
    if formulas.is_empty() {
        String::new()
    } else {
        formulas
            .iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Substitute a variable name within a formula (for first-order quantifier rules).
/// Replaces free occurrences of `var` in atom strings and descends under connectives.
/// Stops at quantifiers that bind the same variable.
pub fn subst(formula: &Formula, var: &str, replacement: &str) -> Formula {
    match formula {
        Formula::Atom(s) => {
            // Replace whole-word occurrences of var inside the atom string
            Formula::Atom(subst_in_atom(s, var, replacement))
        }
        Formula::Bot => Formula::Bot,
        Formula::Top => Formula::Top,
        Formula::Not(a) => Formula::Not(Box::new(subst(a, var, replacement))),
        Formula::And(a, b) => Formula::And(
            Box::new(subst(a, var, replacement)),
            Box::new(subst(b, var, replacement)),
        ),
        Formula::Or(a, b) => Formula::Or(
            Box::new(subst(a, var, replacement)),
            Box::new(subst(b, var, replacement)),
        ),
        Formula::Imp(a, b) => Formula::Imp(
            Box::new(subst(a, var, replacement)),
            Box::new(subst(b, var, replacement)),
        ),
        Formula::Forall(x, body) => {
            if x == var { formula.clone() } // bound — don't substitute
            else { Formula::Forall(x.clone(), Box::new(subst(body, var, replacement))) }
        }
        Formula::Exists(x, body) => {
            if x == var { formula.clone() }
            else { Formula::Exists(x.clone(), Box::new(subst(body, var, replacement))) }
        }
    }
}

/// Replace whole-word occurrences of `var` in an atom string.
fn subst_in_atom(atom: &str, var: &str, replacement: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = atom.chars().collect();
    let var_chars: Vec<char> = var.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + var_chars.len() <= chars.len()
            && chars[i..i + var_chars.len()] == var_chars[..]
        {
            let before_ok = i == 0 || !chars[i - 1].is_alphanumeric();
            let after_ok = i + var_chars.len() >= chars.len()
                || !chars[i + var_chars.len()].is_alphanumeric();
            if before_ok && after_ok {
                result.push_str(replacement);
                i += var_chars.len();
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

/// Format a sequent (antecedent list + succedent) as a string.
pub fn format_sequent_str(antecedents: &[Formula], succedent: &Formula, sep: char) -> String {
    let ant_str = format_formula_list(antecedents);
    if ant_str.is_empty() {
        format!("{} {}", sep, succedent)
    } else {
        format!("{} {} {}", ant_str, sep, succedent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom() {
        assert_eq!(parse_formula("P").unwrap(), Formula::Atom("P".into()));
    }

    #[test]
    fn test_parse_imp() {
        let f = parse_formula("P \u{2192} Q").unwrap();
        assert_eq!(
            f,
            Formula::Imp(
                Box::new(Formula::Atom("P".into())),
                Box::new(Formula::Atom("Q".into())),
            )
        );
    }

    #[test]
    fn test_parse_and() {
        let f = parse_formula("P \u{2227} Q").unwrap();
        assert_eq!(
            f,
            Formula::And(
                Box::new(Formula::Atom("P".into())),
                Box::new(Formula::Atom("Q".into())),
            )
        );
    }

    #[test]
    fn test_parse_not() {
        let f = parse_formula("\u{00AC}P").unwrap();
        assert_eq!(f, Formula::Not(Box::new(Formula::Atom("P".into()))));
    }

    #[test]
    fn test_precedence() {
        // ¬P ∧ Q → R should parse as ((¬P) ∧ Q) → R
        let f = parse_formula("\u{00AC}P \u{2227} Q \u{2192} R").unwrap();
        assert_eq!(
            f,
            Formula::Imp(
                Box::new(Formula::And(
                    Box::new(Formula::Not(Box::new(Formula::Atom("P".into())))),
                    Box::new(Formula::Atom("Q".into())),
                )),
                Box::new(Formula::Atom("R".into())),
            )
        );
    }

    #[test]
    fn test_right_associative_imp() {
        let f = parse_formula("P \u{2192} Q \u{2192} R").unwrap();
        assert_eq!(
            f,
            Formula::Imp(
                Box::new(Formula::Atom("P".into())),
                Box::new(Formula::Imp(
                    Box::new(Formula::Atom("Q".into())),
                    Box::new(Formula::Atom("R".into())),
                )),
            )
        );
    }

    #[test]
    fn test_parse_sequent() {
        let s = parse_sequent("P, Q \u{21D2} P \u{2227} Q", '\u{21D2}').unwrap();
        assert_eq!(s.antecedents.len(), 2);
        assert_eq!(s.antecedents[0], Formula::Atom("P".into()));
        assert_eq!(s.antecedents[1], Formula::Atom("Q".into()));
    }

    #[test]
    fn test_parse_empty_antecedent() {
        let s = parse_sequent("\u{21D2} P \u{2192} P", '\u{21D2}').unwrap();
        assert_eq!(s.antecedents.len(), 0);
    }

    #[test]
    fn test_multiset_eq() {
        let a = vec![Formula::Atom("P".into()), Formula::Atom("Q".into())];
        let b = vec![Formula::Atom("Q".into()), Formula::Atom("P".into())];
        assert!(multiset_eq(&a, &b));
    }

    #[test]
    fn test_ascii_alternatives() {
        let f = parse_formula("P -> Q").unwrap();
        assert_eq!(
            f,
            Formula::Imp(
                Box::new(Formula::Atom("P".into())),
                Box::new(Formula::Atom("Q".into())),
            )
        );
        let f2 = parse_formula("P /\\ Q").unwrap();
        assert_eq!(
            f2,
            Formula::And(
                Box::new(Formula::Atom("P".into())),
                Box::new(Formula::Atom("Q".into())),
            )
        );
    }
}
