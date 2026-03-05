use crate::tree::ProofNode;

/// Parse the S-expression format produced by proof-tree.js `toSexp()`.
///
/// Format:
///   - Leaf: `"text"` or `bare-word`
///   - Node: `((RuleName :right) premise1 premise2 ... --- "conclusion")`
///   - Label specs `(name :right)` and `(name :left)` appear before premises
pub fn parse_proof_sexp(input: &str) -> Result<ProofNode, String> {
    let tokens = tokenize(input)?;
    let mut pos = 0;
    let node = parse_node(&tokens, &mut pos)?;
    if pos < tokens.len() {
        return Err(format!(
            "Unexpected tokens after main expression at position {}",
            pos
        ));
    }
    Ok(node)
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    Sep,        // ---
    Keyword(String), // :right, :left
    Str(String),     // quoted string
    Symbol(String),  // bare word
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        // Skip whitespace
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        if chars[i] == '(' {
            tokens.push(Token::LParen);
            i += 1;
            continue;
        }
        if chars[i] == ')' {
            tokens.push(Token::RParen);
            i += 1;
            continue;
        }

        // Quoted string
        if chars[i] == '"' || chars[i] == '\'' {
            let quote = chars[i];
            i += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    s.push(chars[i + 1]);
                    i += 2;
                } else {
                    s.push(chars[i]);
                    i += 1;
                }
            }
            if i < chars.len() {
                i += 1; // skip closing quote
            }
            tokens.push(Token::Str(s));
            continue;
        }

        // --- separator
        if i + 2 < chars.len()
            && chars[i] == '-'
            && chars[i + 1] == '-'
            && chars[i + 2] == '-'
            && (i + 3 >= chars.len() || chars[i + 3].is_whitespace() || chars[i + 3] == '(' || chars[i + 3] == ')')
        {
            tokens.push(Token::Sep);
            i += 3;
            continue;
        }

        // :keyword
        if chars[i] == ':' {
            i += 1;
            let mut kw = String::new();
            while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '(' && chars[i] != ')' {
                kw.push(chars[i]);
                i += 1;
            }
            tokens.push(Token::Keyword(kw));
            continue;
        }

        // Bare word/symbol
        let mut word = String::new();
        while i < chars.len()
            && !chars[i].is_whitespace()
            && chars[i] != '('
            && chars[i] != ')'
            && chars[i] != '"'
            && chars[i] != '\''
        {
            word.push(chars[i]);
            i += 1;
        }
        if !word.is_empty() {
            tokens.push(Token::Symbol(word));
        }
    }

    Ok(tokens)
}

fn is_label_spec(tokens: &[Token], pos: usize) -> bool {
    // Check for (name :right) or (name :left) pattern
    if pos + 3 >= tokens.len() {
        return false;
    }
    matches!(
        (&tokens[pos], &tokens[pos + 1], &tokens[pos + 2], &tokens[pos + 3]),
        (
            Token::LParen,
            Token::Symbol(_) | Token::Str(_),
            Token::Keyword(kw),
            Token::RParen,
        ) if kw == "right" || kw == "left"
    )
}

fn parse_node(tokens: &[Token], pos: &mut usize) -> Result<ProofNode, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of input".to_string());
    }

    match &tokens[*pos] {
        Token::LParen => {
            // Check if this is a label spec at top level (error)
            if is_label_spec(tokens, *pos) {
                return Err("Unexpected label spec outside rule".to_string());
            }

            *pos += 1; // consume (

            // Parse leading label specs
            let mut rule_name = None;
            while *pos < tokens.len() && is_label_spec(tokens, *pos) {
                *pos += 1; // consume inner (
                let label_value = match &tokens[*pos] {
                    Token::Symbol(s) | Token::Str(s) => s.clone(),
                    _ => return Err("Expected label value".to_string()),
                };
                *pos += 1;
                let side = match &tokens[*pos] {
                    Token::Keyword(kw) => kw.clone(),
                    _ => return Err("Expected :right or :left".to_string()),
                };
                *pos += 1;
                if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                    return Err("Expected ) after label spec".to_string());
                }
                *pos += 1; // consume inner )

                if side == "right" {
                    rule_name = Some(label_value);
                }
                // We ignore :left labels for checking purposes
            }

            // Parse premises (everything before ---)
            let mut premises = Vec::new();
            while *pos < tokens.len()
                && tokens[*pos] != Token::Sep
                && tokens[*pos] != Token::RParen
            {
                premises.push(parse_node(tokens, pos)?);
            }

            // Expect ---
            if *pos >= tokens.len() || tokens[*pos] != Token::Sep {
                return Err("Expected --- separator".to_string());
            }
            *pos += 1; // consume ---

            // Conclusion: collect tokens until ) or :keyword
            let mut conclusion_parts = Vec::new();
            while *pos < tokens.len()
                && tokens[*pos] != Token::RParen
                && !matches!(&tokens[*pos], Token::Keyword(_))
            {
                match &tokens[*pos] {
                    Token::Str(s) => conclusion_parts.push(s.clone()),
                    Token::Symbol(s) => conclusion_parts.push(s.clone()),
                    _ => return Err(format!("Unexpected token in conclusion: {:?}", tokens[*pos])),
                }
                *pos += 1;
            }

            if conclusion_parts.is_empty() {
                return Err("Expected conclusion after ---".to_string());
            }
            let conclusion = conclusion_parts.join(" ");

            // Expect )
            if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                return Err("Expected )".to_string());
            }
            *pos += 1; // consume )

            Ok(ProofNode {
                conclusion,
                rule_name,
                premises,
            })
        }
        Token::Str(s) => {
            let conclusion = s.clone();
            *pos += 1;
            Ok(ProofNode {
                conclusion,
                rule_name: None,
                premises: vec![],
            })
        }
        Token::Symbol(s) => {
            let conclusion = s.clone();
            *pos += 1;
            Ok(ProofNode {
                conclusion,
                rule_name: None,
                premises: vec![],
            })
        }
        other => Err(format!("Unexpected token: {:?}", other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf() {
        let node = parse_proof_sexp(r#""hello world""#).unwrap();
        assert_eq!(node.conclusion, "hello world");
        assert!(node.rule_name.is_none());
        assert!(node.premises.is_empty());
    }

    #[test]
    fn test_simple_rule() {
        let node = parse_proof_sexp(r#"((Int :right) --- "{} ⊢ 3 ⇓ 3")"#).unwrap();
        assert_eq!(node.conclusion, "{} ⊢ 3 ⇓ 3");
        assert_eq!(node.rule_name.as_deref(), Some("Int"));
        assert!(node.premises.is_empty());
    }

    #[test]
    fn test_rule_with_premises() {
        let sexp = r#"((Neg :right) ((Var :right) "{x ↦ 3}(x) = 3" --- "{x ↦ 3} ⊢ x ⇓ 3") "v = -3" --- "{x ↦ 3} ⊢ (- x) ⇓ -3")"#;
        let node = parse_proof_sexp(sexp).unwrap();
        assert_eq!(node.rule_name.as_deref(), Some("Neg"));
        assert_eq!(node.premises.len(), 2);
        assert_eq!(node.premises[0].rule_name.as_deref(), Some("Var"));
        assert_eq!(node.premises[1].conclusion, "v = -3");
    }

    #[test]
    fn test_exercise2_solution() {
        let sexp = r#"((Let :right) ((Int :right) --- "{x ↦ 5} ⊢ 3 ⇓ 3") ((Add :right) ((Var :right) "{x ↦ 5, y ↦ 3}(y) = 3" --- "{x ↦ 5, y ↦ 3} ⊢ y ⇓ 3") ((Var :right) "{x ↦ 5, y ↦ 3}(x) = 5" --- "{x ↦ 5, y ↦ 3} ⊢ x ⇓ 5") "v = 3 + 5" --- "{x ↦ 5, y ↦ 3} ⊢ (+ y x) ⇓ 8") --- "{x ↦ 5} ⊢ (let ([y 3]) (+ y x)) ⇓ 8")"#;
        let node = parse_proof_sexp(sexp).unwrap();
        assert_eq!(node.rule_name.as_deref(), Some("Let"));
        assert_eq!(node.premises.len(), 2);
        assert_eq!(node.premises[1].rule_name.as_deref(), Some("Add"));
        assert_eq!(node.premises[1].premises.len(), 3);
    }

    #[test]
    fn test_empty_conclusion() {
        let sexp = r#"(--- "")"#;
        let node = parse_proof_sexp(sexp).unwrap();
        assert_eq!(node.conclusion, "");
        assert!(node.rule_name.is_none());
    }
}
