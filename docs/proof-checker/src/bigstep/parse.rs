use super::ast::*;
use std::collections::BTreeMap;

/// Parse a judgement string: "ρ ⊢ e ⇓ v" or "{x ↦ 3} ⊢ (- x) ⇓ -3"
pub fn parse_judgement(s: &str) -> Result<Judgement, String> {
    let s = s.trim();

    // Split on ⊢ (first occurrence)
    let turnstile_pos = s
        .find('⊢')
        .ok_or_else(|| "Missing ⊢ in judgement".to_string())?;
    let env_str = s[..turnstile_pos].trim();
    let rest = s[turnstile_pos + '⊢'.len_utf8()..].trim();

    // Split rest on ⇓ (last occurrence)
    let downarrow_pos = rest
        .rfind('⇓')
        .ok_or_else(|| "Missing ⇓ in judgement".to_string())?;
    let expr_str = rest[..downarrow_pos].trim();
    let value_str = rest[downarrow_pos + '⇓'.len_utf8()..].trim();

    let env = parse_env(env_str)?;
    let expr = parse_expr(expr_str)?;
    let value = parse_value(value_str)?;

    Ok(Judgement { env, expr, value })
}

/// Parse an environment string: "{}", "{x ↦ 3}", "{x ↦ 5, y ↦ 3}"
pub fn parse_env(s: &str) -> Result<Env, String> {
    let s = s.trim();
    if s == "{}" {
        return Ok(BTreeMap::new());
    }

    // Strip outer braces
    if !s.starts_with('{') || !s.ends_with('}') {
        return Err(format!("Environment must be enclosed in {{...}}, got: {}", s));
    }
    let inner = &s[1..s.len() - 1];

    // Split on ", " at top nesting level (respecting {}, ⟨⟩, ())
    let bindings = split_top_level(inner, ',');

    let mut env = BTreeMap::new();
    for binding in bindings {
        let binding = binding.trim();
        if binding.is_empty() {
            continue;
        }
        // Split on ↦ (first occurrence)
        let arrow_pos = binding
            .find('↦')
            .ok_or_else(|| format!("Missing ↦ in binding: {}", binding))?;
        let var_name = binding[..arrow_pos].trim().to_string();
        let val_str = binding[arrow_pos + '↦'.len_utf8()..].trim();
        let value = parse_value(val_str)?;
        env.insert(var_name, value);
    }

    Ok(env)
}

/// Parse a value string: integer or closure
pub fn parse_value(s: &str) -> Result<Value, String> {
    let s = s.trim();

    // Try as integer
    if let Ok(i) = s.parse::<i64>() {
        return Ok(Value::Int(i));
    }

    // Try as negative number with Unicode minus
    if s.starts_with('−') || s.starts_with('-') {
        let rest = if s.starts_with('−') {
            &s['−'.len_utf8()..]
        } else {
            &s[1..]
        };
        if let Ok(i) = rest.parse::<i64>() {
            return Ok(Value::Int(-i));
        }
    }

    // Try as closure: ⟨λ (x) e , ρ⟩
    if s.starts_with('⟨') && s.ends_with('⟩') {
        return parse_closure(s);
    }

    Err(format!("Can't parse value: {}", s))
}

/// Parse a closure: ⟨λ (x) e , ρ⟩
fn parse_closure(s: &str) -> Result<Value, String> {
    // Strip ⟨ and ⟩
    let inner = &s['⟨'.len_utf8()..s.len() - '⟩'.len_utf8()];
    let inner = inner.trim();

    // Find "λ" at start
    if !inner.starts_with('λ') {
        return Err(format!("Closure must start with λ, got: {}", s));
    }
    let after_lam = inner['λ'.len_utf8()..].trim();

    // Find the parameter: (x)
    if !after_lam.starts_with('(') {
        return Err(format!("Expected (param) after λ in closure: {}", s));
    }
    let paren_close = after_lam
        .find(')')
        .ok_or_else(|| format!("Missing ) in lambda param: {}", s))?;
    let param = after_lam[1..paren_close].trim().to_string();
    let rest = after_lam[paren_close + 1..].trim();

    // Split rest on last top-level " , " before a "{"
    // The body and env are separated by " , " at the top level
    let (body_str, env_str) = split_closure_body_env(rest)?;

    let body = parse_expr(body_str.trim())?;
    let env = parse_env(env_str.trim())?;

    Ok(Value::Closure { param, body, env })
}

/// Split "body , {env}" — find the last top-level comma that separates body from env
fn split_closure_body_env(s: &str) -> Result<(&str, &str), String> {
    // Find the last " , " at nesting depth 0
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0i32;
    let mut last_comma_byte = None;

    let mut byte_pos = 0;
    for &ch in &chars {
        match ch {
            '(' | '[' | '{' | '⟨' => depth += 1,
            ')' | ']' | '}' | '⟩' => depth -= 1,
            ',' if depth == 0 => {
                last_comma_byte = Some(byte_pos);
            }
            _ => {}
        }
        byte_pos += ch.len_utf8();
    }

    match last_comma_byte {
        Some(pos) => {
            let body = s[..pos].trim();
            let env = s[pos + 1..].trim();
            Ok((body, env))
        }
        None => Err(format!(
            "Can't find ',' separating body and env in closure: {}",
            s
        )),
    }
}

/// Split a string on a delimiter at the top nesting level
fn split_top_level(s: &str, delim: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    let mut byte_pos = 0;

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' | '⟨' => depth += 1,
            ')' | ']' | '}' | '⟩' => depth -= 1,
            c if c == delim && depth == 0 => {
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

/// Parse an expression from the object language.
/// Handles S-expression syntax: x, i, (- e), (+ e₁ e₂), (if0 eg et ef),
/// (let ([x e]) eb), (λ (x) e), (e₁ e₂)
pub fn parse_expr(s: &str) -> Result<Expr, String> {
    let s = s.trim();

    if s.is_empty() {
        return Err("Empty expression".to_string());
    }

    // Integer literal
    if let Ok(i) = s.parse::<i64>() {
        return Ok(Expr::Int(i));
    }

    // Negative integer with various minus signs
    if (s.starts_with('−') || s.starts_with('-')) && s.len() > 1 {
        let rest = if s.starts_with('−') {
            &s['−'.len_utf8()..]
        } else {
            &s[1..]
        };
        if let Ok(i) = rest.parse::<i64>() {
            return Ok(Expr::Int(-i));
        }
    }

    // Parenthesized expression
    if (s.starts_with('(') && s.ends_with(')')) || (s.starts_with('[') && s.ends_with(']')) {
        let inner = &s[1..s.len() - 1].trim();
        return parse_paren_expr(inner);
    }

    // Must be a variable (bare symbol)
    if is_valid_symbol(s) {
        return Ok(Expr::Var(s.to_string()));
    }

    Err(format!("Can't parse expression: {}", s))
}

fn is_valid_symbol(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // A symbol is anything that's not a number and doesn't start with parens
    let first = s.chars().next().unwrap();
    !first.is_ascii_digit() && first != '(' && first != ')' && first != '[' && first != ']'
}

/// Parse the interior of a parenthesized expression
fn parse_paren_expr(s: &str) -> Result<Expr, String> {
    let parts = tokenize_sexp(s)?;

    if parts.is_empty() {
        return Err("Empty parenthesized expression".to_string());
    }

    // (- e)
    if parts[0] == "-" && parts.len() == 2 {
        let e = parse_expr(&parts[1])?;
        return Ok(Expr::Neg(Box::new(e)));
    }

    // (+ e₁ e₂)
    if parts[0] == "+" && parts.len() == 3 {
        let e1 = parse_expr(&parts[1])?;
        let e2 = parse_expr(&parts[2])?;
        return Ok(Expr::Add(Box::new(e1), Box::new(e2)));
    }

    // (if0 eg et ef)
    if parts[0] == "if0" && parts.len() == 4 {
        let eg = parse_expr(&parts[1])?;
        let et = parse_expr(&parts[2])?;
        let ef = parse_expr(&parts[3])?;
        return Ok(Expr::If0(Box::new(eg), Box::new(et), Box::new(ef)));
    }

    // (let ([x e]) eb)
    if parts[0] == "let" && parts.len() == 3 {
        let binding = parts[1].trim();
        // binding looks like ([x e]) — strip outer parens/brackets
        let binding_inner = strip_parens(binding)
            .ok_or_else(|| format!("Invalid let binding: {}", binding))?;
        // binding_inner looks like [x e] or (x e) — strip again
        let binding_inner2 = strip_parens(binding_inner.trim())
            .ok_or_else(|| format!("Invalid let binding inner: {}", binding_inner))?;
        let bind_parts = tokenize_sexp(binding_inner2.trim())?;
        if bind_parts.len() != 2 {
            return Err(format!("Let binding must have exactly [x e], got: {}", binding));
        }
        let var = bind_parts[0].clone();
        let e = parse_expr(&bind_parts[1])?;
        let eb = parse_expr(&parts[2])?;
        return Ok(Expr::Let(var, Box::new(e), Box::new(eb)));
    }

    // (λ (x) e) — lambda
    if (parts[0] == "λ" || parts[0] == "lambda" || parts[0] == "\\lambda") && parts.len() == 3 {
        let param_str = parts[1].trim();
        let param_inner = strip_parens(param_str)
            .ok_or_else(|| format!("Expected (param) in lambda: {}", param_str))?;
        let param = param_inner.trim().to_string();
        let body = parse_expr(&parts[2])?;
        return Ok(Expr::Lam(param, Box::new(body)));
    }

    // (e₁ e₂) — application
    if parts.len() == 2 {
        let e1 = parse_expr(&parts[0])?;
        let e2 = parse_expr(&parts[1])?;
        return Ok(Expr::App(Box::new(e1), Box::new(e2)));
    }

    Err(format!(
        "Can't parse parenthesized expression with {} parts: {}",
        parts.len(),
        s
    ))
}

fn strip_parens(s: &str) -> Option<&str> {
    let s = s.trim();
    if (s.starts_with('(') && s.ends_with(')')) || (s.starts_with('[') && s.ends_with(']')) {
        Some(&s[1..s.len() - 1])
    } else {
        None
    }
}

/// Tokenize an S-expression interior into top-level parts.
/// Preserves nested parenthesized groups as single tokens.
fn tokenize_sexp(s: &str) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut chars = s.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' | '[' => {
                depth += 1;
                current.push(ch);
                chars.next();
            }
            ')' | ']' => {
                depth -= 1;
                current.push(ch);
                chars.next();
                // If back to depth 0, end this token
                if depth == 0 && !current.trim().is_empty() {
                    parts.push(current.trim().to_string());
                    current.clear();
                }
            }
            c if c.is_whitespace() && depth == 0 => {
                if !current.trim().is_empty() {
                    parts.push(current.trim().to_string());
                    current.clear();
                }
                chars.next();
            }
            _ => {
                current.push(ch);
                chars.next();
            }
        }
    }

    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }

    Ok(parts)
}

// ── Side condition parsers ───────────────────────────────────────────

/// Parsed side condition types
#[derive(Debug, Clone)]
pub enum SideCondition {
    /// ρ(x) = v — variable lookup
    Lookup {
        env_str: String,
        var: String,
        val_str: String,
    },
    /// v = -i — negation result
    NegResult { result_str: String, operand_str: String },
    /// v = i₁ + i₂ — addition result
    AddResult {
        result_str: String,
        left_str: String,
        right_str: String,
    },
    /// i ≠ 0 — nonzero guard
    Nonzero { val_str: String },
}

/// Try to parse a side condition string
pub fn parse_side_condition(s: &str) -> Option<SideCondition> {
    let s = s.trim();

    // i ≠ 0
    if s.contains('≠') {
        let parts: Vec<&str> = s.split('≠').collect();
        if parts.len() == 2 {
            return Some(SideCondition::Nonzero {
                val_str: parts[0].trim().to_string(),
            });
        }
    }

    // ρ(x) = v — env lookup: pattern is "{...}(x) = v"
    if s.contains('(') && s.contains(") =") {
        // Find the "} (" or similar pattern
        if s.find(")(").is_some() {
            // Not an env lookup
        } else if let Some(paren_start) = find_lookup_paren(s) {
            let env_str = s[..paren_start].trim();
            let rest = &s[paren_start..];
            if let Some(eq_pos) = rest.find(") =") {
                let var = rest[1..eq_pos].trim().to_string();
                let val_str = rest[eq_pos + 3..].trim().to_string();
                return Some(SideCondition::Lookup {
                    env_str: env_str.to_string(),
                    var,
                    val_str,
                });
            }
        }
    }

    // v = ... patterns
    if let Some(eq_pos) = s.find(" = ") {
        // Skip if this looks like a lookup (env before the =)
        let lhs = s[..eq_pos].trim();
        let rhs = s[eq_pos + 3..].trim();

        // v = -i (negation)
        if rhs.starts_with('-') || rhs.starts_with('−') {
            let neg_part = if rhs.starts_with('−') {
                &rhs['−'.len_utf8()..]
            } else {
                &rhs[1..]
            };
            if neg_part.trim().parse::<i64>().is_ok() || is_paren_wrapped_neg(neg_part) {
                return Some(SideCondition::NegResult {
                    result_str: lhs.to_string(),
                    operand_str: rhs.to_string(),
                });
            }
        }

        // v = i₁ + i₂ (addition) — find + at top level in rhs
        if let Some((left, right)) = split_addition(rhs) {
            return Some(SideCondition::AddResult {
                result_str: lhs.to_string(),
                left_str: left.to_string(),
                right_str: right.to_string(),
            });
        }

        // Also handle negation result: "v = -i" where i could be negative like "v = -(-3)" or "v = -3"
        // Catch-all negation
        if rhs.starts_with('-') || rhs.starts_with('−') {
            return Some(SideCondition::NegResult {
                result_str: lhs.to_string(),
                operand_str: rhs.to_string(),
            });
        }
    }

    None
}

/// Find the opening paren of a lookup like "{x ↦ 3}(x)"
fn find_lookup_paren(s: &str) -> Option<usize> {
    // Find the "(" that comes after a "}"
    let mut found_brace = false;
    let mut byte_pos = 0;
    for ch in s.chars() {
        if ch == '}' {
            found_brace = true;
        } else if ch == '(' && found_brace {
            return Some(byte_pos);
        } else if !ch.is_whitespace() && found_brace {
            found_brace = false;
        }
        byte_pos += ch.len_utf8();
    }
    None
}

fn is_paren_wrapped_neg(s: &str) -> bool {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') {
        let inner = &s[1..s.len() - 1];
        if inner.starts_with('-') || inner.starts_with('−') {
            return true;
        }
    }
    false
}

/// Split an addition expression: "3 + 5" → ("3", "5")
/// Handles parenthesized negatives: "1 + (-1)"
fn split_addition(s: &str) -> Option<(String, String)> {
    let s = s.trim();
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0i32;
    let mut byte_pos = 0;
    let mut last_plus = None;

    // Find the last top-level '+' that's surrounded by spaces
    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '(' | '[' | '⟨' => depth += 1,
            ')' | ']' | '⟩' => depth -= 1,
            '+' if depth == 0 => {
                // Check it's surrounded by spaces (not part of a number)
                if i > 0 && chars[i - 1] == ' ' {
                    last_plus = Some(byte_pos);
                }
            }
            _ => {}
        }
        byte_pos += ch.len_utf8();
    }

    last_plus.map(|pos| {
        let left = s[..pos].trim().to_string();
        let right = s[pos + 1..].trim().to_string();
        (left, right)
    })
}

/// Parse a numeric value from a side condition (handles parens, minus signs)
pub fn parse_side_int(s: &str) -> Result<i64, String> {
    let s = s.trim();

    // Direct integer
    if let Ok(i) = s.parse::<i64>() {
        return Ok(i);
    }

    // Parenthesized: (-3) or (−3)
    if s.starts_with('(') && s.ends_with(')') {
        let inner = s[1..s.len() - 1].trim();
        return parse_side_int(inner);
    }

    // Unicode minus
    if s.starts_with('−') {
        let rest = s['−'.len_utf8()..].trim();
        if let Ok(i) = rest.parse::<i64>() {
            return Ok(-i);
        }
        // Could be −(something)
        return parse_side_int(rest).map(|i| -i);
    }

    // ASCII minus not followed by digit directly
    if s.starts_with('-') {
        let rest = s[1..].trim();
        return parse_side_int(rest).map(|i| -i);
    }

    Err(format!("Can't parse integer from side condition: {}", s))
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_env() {
        let env = parse_env("{}").unwrap();
        assert!(env.is_empty());
    }

    #[test]
    fn test_parse_single_binding() {
        let env = parse_env("{x ↦ 3}").unwrap();
        assert_eq!(env.get("x"), Some(&Value::Int(3)));
    }

    #[test]
    fn test_parse_multiple_bindings() {
        let env = parse_env("{x ↦ 5, y ↦ 3}").unwrap();
        assert_eq!(env.get("x"), Some(&Value::Int(5)));
        assert_eq!(env.get("y"), Some(&Value::Int(3)));
    }

    #[test]
    fn test_parse_int_expr() {
        let e = parse_expr("3").unwrap();
        assert_eq!(e, Expr::Int(3));
    }

    #[test]
    fn test_parse_neg_int_expr() {
        let e = parse_expr("-3").unwrap();
        assert_eq!(e, Expr::Int(-3));
    }

    #[test]
    fn test_parse_var_expr() {
        let e = parse_expr("x").unwrap();
        assert_eq!(e, Expr::Var("x".to_string()));
    }

    #[test]
    fn test_parse_neg_expr() {
        let e = parse_expr("(- x)").unwrap();
        assert_eq!(e, Expr::Neg(Box::new(Expr::Var("x".to_string()))));
    }

    #[test]
    fn test_parse_add_expr() {
        let e = parse_expr("(+ x 1)").unwrap();
        assert_eq!(
            e,
            Expr::Add(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Int(1)),
            )
        );
    }

    #[test]
    fn test_parse_if0_expr() {
        let e = parse_expr("(if0 x 1 2)").unwrap();
        assert_eq!(
            e,
            Expr::If0(
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2)),
            )
        );
    }

    #[test]
    fn test_parse_let_expr() {
        let e = parse_expr("(let ([x 5]) (+ x x))").unwrap();
        assert_eq!(
            e,
            Expr::Let(
                "x".to_string(),
                Box::new(Expr::Int(5)),
                Box::new(Expr::Add(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Var("x".to_string())),
                )),
            )
        );
    }

    #[test]
    fn test_parse_lambda_expr() {
        let e = parse_expr("(λ (x) (+ x 1))").unwrap();
        assert_eq!(
            e,
            Expr::Lam(
                "x".to_string(),
                Box::new(Expr::Add(
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Int(1)),
                )),
            )
        );
    }

    #[test]
    fn test_parse_app_expr() {
        let e = parse_expr("((λ (x) x) 5)").unwrap();
        assert_eq!(
            e,
            Expr::App(
                Box::new(Expr::Lam("x".to_string(), Box::new(Expr::Var("x".to_string())))),
                Box::new(Expr::Int(5)),
            )
        );
    }

    #[test]
    fn test_parse_closure_value() {
        let v = parse_value("⟨λ (x) (+ x 1) , {}⟩").unwrap();
        match v {
            Value::Closure { param, body, env } => {
                assert_eq!(param, "x");
                assert_eq!(
                    body,
                    Expr::Add(
                        Box::new(Expr::Var("x".to_string())),
                        Box::new(Expr::Int(1)),
                    )
                );
                assert!(env.is_empty());
            }
            _ => panic!("Expected closure"),
        }
    }

    #[test]
    fn test_parse_closure_with_env() {
        let v = parse_value("⟨λ (y) (+ x y) , {x ↦ 10}⟩").unwrap();
        match v {
            Value::Closure { param, body, env } => {
                assert_eq!(param, "y");
                assert_eq!(env.get("x"), Some(&Value::Int(10)));
                assert_eq!(
                    body,
                    Expr::Add(
                        Box::new(Expr::Var("x".to_string())),
                        Box::new(Expr::Var("y".to_string())),
                    )
                );
            }
            _ => panic!("Expected closure"),
        }
    }

    #[test]
    fn test_parse_judgement_simple() {
        let j = parse_judgement("{} ⊢ 3 ⇓ 3").unwrap();
        assert!(j.env.is_empty());
        assert_eq!(j.expr, Expr::Int(3));
        assert_eq!(j.value, Value::Int(3));
    }

    #[test]
    fn test_parse_judgement_neg() {
        let j = parse_judgement("{x ↦ 3} ⊢ (- x) ⇓ -3").unwrap();
        assert_eq!(j.env.get("x"), Some(&Value::Int(3)));
        assert_eq!(j.expr, Expr::Neg(Box::new(Expr::Var("x".to_string()))));
        assert_eq!(j.value, Value::Int(-3));
    }

    #[test]
    fn test_parse_judgement_closure_result() {
        let j =
            parse_judgement("{} ⊢ (λ (x) (+ x 1)) ⇓ ⟨λ (x) (+ x 1) , {}⟩").unwrap();
        assert!(j.env.is_empty());
        match j.value {
            Value::Closure { param, .. } => assert_eq!(param, "x"),
            _ => panic!("Expected closure value"),
        }
    }

    #[test]
    fn test_side_condition_lookup() {
        let sc = parse_side_condition("{x ↦ 3}(x) = 3").unwrap();
        match sc {
            SideCondition::Lookup { var, val_str, .. } => {
                assert_eq!(var, "x");
                assert_eq!(val_str, "3");
            }
            _ => panic!("Expected Lookup"),
        }
    }

    #[test]
    fn test_side_condition_neg() {
        let sc = parse_side_condition("v = -3").unwrap();
        match sc {
            SideCondition::NegResult {
                result_str,
                operand_str,
            } => {
                assert_eq!(result_str, "v");
                assert_eq!(operand_str, "-3");
            }
            _ => panic!("Expected NegResult"),
        }
    }

    #[test]
    fn test_side_condition_add() {
        let sc = parse_side_condition("v = 3 + 5").unwrap();
        match sc {
            SideCondition::AddResult {
                result_str,
                left_str,
                right_str,
            } => {
                assert_eq!(result_str, "v");
                assert_eq!(left_str, "3");
                assert_eq!(right_str, "5");
            }
            _ => panic!("Expected AddResult"),
        }
    }

    #[test]
    fn test_side_condition_add_negative() {
        let sc = parse_side_condition("v = 1 + (-1)").unwrap();
        match sc {
            SideCondition::AddResult {
                left_str,
                right_str,
                ..
            } => {
                assert_eq!(left_str, "1");
                assert_eq!(right_str, "(-1)");
            }
            _ => panic!("Expected AddResult"),
        }
    }

    #[test]
    fn test_side_condition_nonzero() {
        let sc = parse_side_condition("3 ≠ 0").unwrap();
        match sc {
            SideCondition::Nonzero { val_str } => {
                assert_eq!(val_str, "3");
            }
            _ => panic!("Expected Nonzero"),
        }
    }

    #[test]
    fn test_parse_side_int() {
        assert_eq!(parse_side_int("3").unwrap(), 3);
        assert_eq!(parse_side_int("-3").unwrap(), -3);
        assert_eq!(parse_side_int("(-1)").unwrap(), -1);
        assert_eq!(parse_side_int("−3").unwrap(), -3);
    }
}
