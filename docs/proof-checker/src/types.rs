/// Type AST and parser for STLC and System F.

use std::fmt;

/// Types in the simply-typed lambda calculus and System F.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Int,
    Bool,
    TyVar(String),
    Arrow(Box<Ty>, Box<Ty>),
    Forall(String, Box<Ty>),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Int => write!(f, "int"),
            Ty::Bool => write!(f, "bool"),
            Ty::TyVar(s) => write!(f, "{}", s),
            Ty::Arrow(a, b) => {
                match a.as_ref() {
                    Ty::Arrow(_, _) | Ty::Forall(_, _) => write!(f, "({}) \u{2192} {}", a, b),
                    _ => write!(f, "{} \u{2192} {}", a, b),
                }
            }
            Ty::Forall(v, t) => write!(f, "\u{2200}{}. {}", v, t),
        }
    }
}

/// A typing judgement: context ⊢ expr : type.
#[derive(Debug, Clone)]
pub struct TypingJudgement {
    pub context: Vec<(String, Ty)>,
    pub expr_str: String,
    pub ty: Ty,
}

// ── Tokenizer ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum TyToken {
    Int,
    Bool,
    Ident(String),
    Arrow,
    Forall,
    Dot,
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Result<Vec<TyToken>, String> {
    let chars: Vec<char> = s.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        match chars[i] {
            '(' => { tokens.push(TyToken::LParen); i += 1; }
            ')' => { tokens.push(TyToken::RParen); i += 1; }
            '.' => { tokens.push(TyToken::Dot); i += 1; }
            '\u{2192}' => { tokens.push(TyToken::Arrow); i += 1; } // →
            '-' if i + 1 < chars.len() && chars[i + 1] == '>' => {
                tokens.push(TyToken::Arrow);
                i += 2;
            }
            '\u{2200}' => { tokens.push(TyToken::Forall); i += 1; } // ∀
            c if c.is_alphabetic() || c == '_' => {
                let mut name = String::new();
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_'
                    || ('\u{2080}'..='\u{2089}').contains(&chars[i])
                    || chars[i] == '\'')
                {
                    name.push(chars[i]);
                    i += 1;
                }
                match name.as_str() {
                    "int" => tokens.push(TyToken::Int),
                    "bool" => tokens.push(TyToken::Bool),
                    "forall" => tokens.push(TyToken::Forall),
                    _ => tokens.push(TyToken::Ident(name)),
                }
            }
            c => return Err(format!("Unexpected character '{}' in type", c)),
        }
    }

    Ok(tokens)
}

// ── Parser ───────────────────────────────────────────────────────────

/// Parse a type string.
pub fn parse_type(s: &str) -> Result<Ty, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("Empty type".to_string());
    }
    let tokens = tokenize(s)?;
    let mut pos = 0;
    let t = parse_arrow(&tokens, &mut pos)?;
    if pos < tokens.len() {
        return Err(format!("Unexpected token {:?} after type", tokens[pos]));
    }
    Ok(t)
}

// → is right-associative
fn parse_arrow(tokens: &[TyToken], pos: &mut usize) -> Result<Ty, String> {
    // Check for ∀
    if *pos < tokens.len() && tokens[*pos] == TyToken::Forall {
        *pos += 1;
        if *pos >= tokens.len() {
            return Err("Expected type variable after \u{2200}".to_string());
        }
        let var = match &tokens[*pos] {
            TyToken::Ident(s) => s.clone(),
            _ => return Err("Expected type variable after \u{2200}".to_string()),
        };
        *pos += 1;
        if *pos >= tokens.len() || tokens[*pos] != TyToken::Dot {
            return Err("Expected '.' after type variable in \u{2200}".to_string());
        }
        *pos += 1;
        let body = parse_arrow(tokens, pos)?;
        return Ok(Ty::Forall(var, Box::new(body)));
    }

    let left = parse_primary_ty(tokens, pos)?;
    if *pos < tokens.len() && tokens[*pos] == TyToken::Arrow {
        *pos += 1;
        let right = parse_arrow(tokens, pos)?;
        Ok(Ty::Arrow(Box::new(left), Box::new(right)))
    } else {
        Ok(left)
    }
}

fn parse_primary_ty(tokens: &[TyToken], pos: &mut usize) -> Result<Ty, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of type".to_string());
    }
    match &tokens[*pos] {
        TyToken::Int => { *pos += 1; Ok(Ty::Int) }
        TyToken::Bool => { *pos += 1; Ok(Ty::Bool) }
        TyToken::Ident(s) => {
            let t = Ty::TyVar(s.clone());
            *pos += 1;
            Ok(t)
        }
        TyToken::LParen => {
            *pos += 1;
            let t = parse_arrow(tokens, pos)?;
            if *pos >= tokens.len() || tokens[*pos] != TyToken::RParen {
                return Err("Missing closing parenthesis in type".to_string());
            }
            *pos += 1;
            Ok(t)
        }
        other => Err(format!("Expected type, got {:?}", other)),
    }
}

// ── Typing judgement parsing ─────────────────────────────────────────

/// Parse a typing judgement: "Γ ⊢ e : τ"
/// Returns (context, expression_string, type).
pub fn parse_typing_judgement(s: &str) -> Result<TypingJudgement, String> {
    let s = s.trim();

    // Split on ⊢
    let turnstile_pos = s
        .find('\u{22A2}')
        .ok_or_else(|| "Missing \u{22A2} in typing judgement".to_string())?;
    let ctx_str = s[..turnstile_pos].trim();
    let rest = s[turnstile_pos + '\u{22A2}'.len_utf8()..].trim();

    // Find the last ':' at top level in rest (separating expression from type)
    let colon_pos = find_last_top_level_colon(rest)
        .ok_or_else(|| "Missing ':' separating expression and type".to_string())?;

    let expr_str = rest[..colon_pos].trim().to_string();
    let ty_str = rest[colon_pos + 1..].trim();

    let context = if ctx_str.is_empty() {
        vec![]
    } else {
        parse_typing_context(ctx_str)?
    };

    let ty = parse_type(ty_str)?;

    Ok(TypingJudgement { context, expr_str, ty })
}

/// Parse a typing context: "x : int, y : bool"
fn parse_typing_context(s: &str) -> Result<Vec<(String, Ty)>, String> {
    let parts = split_top_level_commas(s);
    let mut ctx = Vec::new();
    for part in &parts {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        // Split on the first ':'
        let colon_pos = part
            .find(':')
            .ok_or_else(|| format!("Missing ':' in context entry: {}", part))?;
        let var = part[..colon_pos].trim().to_string();
        let ty_str = part[colon_pos + 1..].trim();
        let ty = parse_type(ty_str)?;
        ctx.push((var, ty));
    }
    Ok(ctx)
}

fn find_last_top_level_colon(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    let mut last_colon = None;
    let mut byte_pos = 0;

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' | '\u{27E8}' => depth += 1,
            ')' | ']' | '}' | '\u{27E9}' => depth -= 1,
            ':' if depth == 0 => {
                last_colon = Some(byte_pos);
            }
            _ => {}
        }
        byte_pos += ch.len_utf8();
    }

    last_colon
}

fn split_top_level_commas(s: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    let mut byte_pos = 0;

    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' | '\u{27E8}' => depth += 1,
            ')' | ']' | '}' | '\u{27E9}' => depth -= 1,
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

/// Check if two typing contexts are equal as sets.
pub fn contexts_eq(a: &[(String, Ty)], b: &[(String, Ty)]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.sort_by(|x, y| x.0.cmp(&y.0));
    b.sort_by(|x, y| x.0.cmp(&y.0));
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_type("int").unwrap(), Ty::Int);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_type("bool").unwrap(), Ty::Bool);
    }

    #[test]
    fn test_parse_arrow() {
        let t = parse_type("int \u{2192} bool").unwrap();
        assert_eq!(t, Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool)));
    }

    #[test]
    fn test_parse_arrow_right_assoc() {
        let t = parse_type("int \u{2192} int \u{2192} bool").unwrap();
        assert_eq!(
            t,
            Ty::Arrow(
                Box::new(Ty::Int),
                Box::new(Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool)))
            )
        );
    }

    #[test]
    fn test_parse_forall() {
        let t = parse_type("\u{2200}\u{03B1}. \u{03B1} \u{2192} \u{03B1}").unwrap();
        assert_eq!(
            t,
            Ty::Forall(
                "\u{03B1}".into(),
                Box::new(Ty::Arrow(
                    Box::new(Ty::TyVar("\u{03B1}".into())),
                    Box::new(Ty::TyVar("\u{03B1}".into())),
                ))
            )
        );
    }

    #[test]
    fn test_parse_typing_judgement() {
        let j = parse_typing_judgement("\u{22A2} 42 : int").unwrap();
        assert!(j.context.is_empty());
        assert_eq!(j.expr_str, "42");
        assert_eq!(j.ty, Ty::Int);
    }

    #[test]
    fn test_parse_typing_judgement_with_ctx() {
        let j = parse_typing_judgement("x : int \u{22A2} x : int").unwrap();
        assert_eq!(j.context.len(), 1);
        assert_eq!(j.context[0], ("x".into(), Ty::Int));
        assert_eq!(j.expr_str, "x");
        assert_eq!(j.ty, Ty::Int);
    }

    #[test]
    fn test_parse_ascii_arrow() {
        let t = parse_type("int -> bool").unwrap();
        assert_eq!(t, Ty::Arrow(Box::new(Ty::Int), Box::new(Ty::Bool)));
    }
}
