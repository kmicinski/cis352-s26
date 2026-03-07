// Integration tests: parse → reduce → verify normal form
// Tests the full pipeline that the WASM API exercises.

use lambda_viz::parser::parse;
use lambda_viz::term::{Strategy, Term, encode_path};
use lambda_viz::render::build_term_info;

/// Reduce to normal form under a strategy, returning the final term.
/// Returns None if it doesn't terminate within `limit` steps.
fn reduce_to_nf(term: &Term, strategy: &Strategy, limit: usize) -> Option<Term> {
    let mut current = term.clone();
    for _ in 0..limit {
        match current.reduce_step(strategy) {
            Some(next) => current = next,
            None => return Some(current),
        }
    }
    None
}

// ── Full pipeline: parse → reduce → check result ────────────

#[test]
fn pipeline_identity_applied() {
    let t = parse("((λ (x) x) hello)").unwrap();
    let nf = reduce_to_nf(&t, &Strategy::Normal, 10).unwrap();
    assert_eq!(nf.display_string(), "hello");
}

#[test]
fn pipeline_k_combinator() {
    let t = parse("((λ (x y) x) a b)").unwrap();
    let nf = reduce_to_nf(&t, &Strategy::Normal, 10).unwrap();
    assert_eq!(nf.display_string(), "a");
}

#[test]
fn pipeline_succ_2_is_3() {
    let t = parse("((λ (n f x) (f ((n f) x))) (λ (f x) (f (f x))))").unwrap();
    let nf = reduce_to_nf(&t, &Strategy::Normal, 30).unwrap();
    // Church numeral 3: λf. λx. f(f(f x))
    let three = parse("(λ (f x) (f (f (f x))))").unwrap();
    assert_eq!(nf, three, "succ 2 = 3");
}

#[test]
fn pipeline_two_plus_two() {
    let t = parse("((λ (m n f x) ((n f) ((m f) x))) (λ (f x) (f (f x))) (λ (f x) (f (f x))))").unwrap();
    let nf = reduce_to_nf(&t, &Strategy::Normal, 50).unwrap();
    let four = parse("(λ (f x) (f (f (f (f x)))))").unwrap();
    assert_eq!(nf, four, "2 + 2 = 4");
}

#[test]
fn pipeline_not_true_is_false() {
    // CBV booleans: true = λtf.t(λ_._ ), false = λtf.f(λ_._)
    let t = parse("((λ (b) ((b (λ (_) (λ (t f) (f (λ (_) _))))) (λ (_) (λ (t f) (t (λ (_) _)))))) (λ (t f) (t (λ (_) _))))").unwrap();
    let nf = reduce_to_nf(&t, &Strategy::Normal, 30).unwrap();
    let fls = parse("(λ (t f) (f (λ (_) _)))").unwrap();
    assert_eq!(nf, fls, "not true = false");
}

// ── Church-Rosser property across strategies ────────────────

/// Assert that all strategies that terminate reach the same normal form.
fn assert_church_rosser(input: &str, limit: usize) {
    let t = parse(input).unwrap();
    let strategies = [
        ("Normal", Strategy::Normal),
        ("Applicative", Strategy::Applicative),
        ("CBV", Strategy::CallByValue),
        ("CBN", Strategy::CallByName),
    ];
    let results: Vec<(&str, String)> = strategies.iter().filter_map(|(name, s)| {
        reduce_to_nf(&t, s, limit).map(|nf| (*name, nf.display_string()))
    }).collect();

    if results.len() > 1 {
        for (name, nf) in &results[1..] {
            assert_eq!(
                nf, &results[0].1,
                "Church-Rosser violated for {:?}: {} got {:?}, {} got {:?}",
                input, results[0].0, results[0].1, name, nf
            );
        }
    }
}

#[test]
fn church_rosser_identity_app() {
    assert_church_rosser("((λ (x) x) y)", 10);
}

#[test]
fn church_rosser_two_redex() {
    assert_church_rosser("((λ (x) x) ((λ (y) y) z))", 10);
}

#[test]
fn church_rosser_demo_term() {
    // The exact term from the tutorial that triggered the bug.
    assert_church_rosser("((λ (x y) x) ((λ (a) a) z) w)", 20);
}

#[test]
fn church_rosser_k_combinator_app() {
    assert_church_rosser("((λ (x y) x) a b)", 10);
}

#[test]
fn church_rosser_nested_redex() {
    assert_church_rosser("((λ (f x) (f (f x))) (λ (y) y) z)", 20);
}

// ── The root-level redex bug (regression test) ──────────────

#[test]
fn root_redex_not_treated_as_normal_form() {
    // After one normal-order step on the CR demo term, the result is
    // ((λ (y) ((λ (a) a) z)) w) which has a ROOT-LEVEL redex.
    // strategy_next must be Some(""), not None.
    let t = parse("((λ (x y) x) ((λ (a) a) z) w)").unwrap();
    let after_one = t.reduce_step(&Strategy::Normal).unwrap();

    // after_one should be ((λ (y) ((λ (a) a) z)) w) — still has redexes
    assert!(
        !after_one.is_normal_form(&Strategy::Normal),
        "After one step, term {:?} should NOT be in normal form",
        after_one.display_string()
    );

    // The next redex should be at the root (empty path)
    let path = after_one.next_redex(&Strategy::Normal).unwrap();
    assert_eq!(path, vec![], "Next redex should be at root");
    assert_eq!(encode_path(&path), "", "Encoded path is empty string");
}

#[test]
fn build_term_info_root_redex_has_strategy_next() {
    // Verify that build_term_info correctly reports strategy_next for root redexes.
    let t = parse("((λ (y) z) w)").unwrap();
    let info = build_term_info(&t, &Strategy::Normal);
    assert!(!info.is_normal_form, "Root redex is not in normal form");
    assert_eq!(info.strategy_next, Some("".to_string()), "strategy_next must be Some(\"\") for root redex");
}

#[test]
fn cr_demo_reaches_z_in_multiple_steps() {
    // Regression: the demo was stopping after 1 step because the second
    // step's redex is at the root (path ""), and "" is falsy in JavaScript.
    let t = parse("((λ (x y) x) ((λ (a) a) z) w)").unwrap();

    let mut current = t.clone();
    let mut steps = 0;
    loop {
        let info = build_term_info(&current, &Strategy::Normal);
        if info.is_normal_form { break; }
        // strategy_next must be Some(...) since we're not in normal form
        assert!(
            info.strategy_next.is_some(),
            "Step {}: not in NF but strategy_next is None! Term: {}",
            steps, current.display_string()
        );
        current = current.reduce_step(&Strategy::Normal).unwrap();
        steps += 1;
        assert!(steps < 20, "Didn't reach normal form in 20 steps");
    }

    assert_eq!(current.display_string(), "z");
    assert!(steps >= 3, "Should take at least 3 steps, took {}", steps);
}

// ── Capture-avoiding substitution through reduction ─────────

#[test]
fn capture_avoidance_full_pipeline() {
    // ((λx. (λy. x)) y) should NOT reduce to (λy. y) (that changes meaning)
    let t = parse("((λ (x) (λ (y) x)) y)").unwrap();
    let nf = reduce_to_nf(&t, &Strategy::Normal, 10).unwrap();

    // Result should be λ<fresh>. y where <fresh> ≠ y
    if let Term::Abs(param, body) = &nf {
        assert_ne!(param, "y", "Capture avoidance: must rename binder");
        assert_eq!(body.as_ref(), &Term::Var("y".to_string()));
    } else {
        panic!("Expected Abs, got: {}", nf.display_string());
    }
}

// ── All example terms parse without error ───────────────────

#[test]
fn all_examples_parse() {
    let examples = vec![
        ("identity", "(λ (x) x)"),
        ("K", "(λ (x y) x)"),
        ("S", "(λ (x y z) ((x z) (y z)))"),
        ("B", "(λ (f g x) (f (g x)))"),
        ("C", "(λ (f x y) ((f y) x))"),
        ("W", "(λ (f x) ((f x) x))"),
        ("omega", "((λ (x) (x x)) (λ (x) (x x)))"),
        ("true", "(λ (t f) (t (λ (_) _)))"),
        ("false", "(λ (t f) (f (λ (_) _)))"),
        ("church0", "(λ (f x) x)"),
        ("church1", "(λ (f x) (f x))"),
        ("church2", "(λ (f x) (f (f x)))"),
        ("succ", "(λ (n f x) (f ((n f) x)))"),
        ("plus", "(λ (m n f x) ((n f) ((m f) x)))"),
        ("mult", "(λ (m n f x) ((m (n f)) x))"),
        ("pred", "(λ (n f x) (((n (λ (g h) (h (g f)))) (λ (u) x)) (λ (u) u)))"),
        ("cons", "(λ (a b when_cons when_null) ((when_cons a) b))"),
        ("car", "(λ (p) ((p (λ (a b) a)) (λ (_) (λ (x) x))))"),
        ("Z", "((λ (u) (u u)) (λ (y) (λ (mk) (mk (λ (x) (((y y) mk) x))))))"),
        ("SKK", "(((\u{03bb} (x y z) ((x z) (y z))) (\u{03bb} (x y) x)) (\u{03bb} (x y) x))"),
    ];
    for (name, input) in &examples {
        parse(input).unwrap_or_else(|e| panic!("Failed to parse {}: {}", name, e));
    }
}

// ── Round-trip: parse → display → parse ─────────────────────

#[test]
fn roundtrip_all_examples() {
    let examples = vec![
        "(λ (x) x)",
        "(λ (x y) x)",
        "((λ (x) (x x)) (λ (x) (x x)))",
        "((λ (x) x) y)",
        "(λ (f x) (f (f x)))",
    ];
    for input in &examples {
        let t = parse(input).unwrap();
        let displayed = t.display_string();
        let t2 = parse(&displayed)
            .unwrap_or_else(|e| panic!("Round-trip failed for {:?} → {:?}: {}", input, displayed, e));
        assert_eq!(t, t2, "Round-trip mismatch for {:?}", input);
    }
}

// ── Omega self-loop ─────────────────────────────────────────

#[test]
fn omega_is_fixpoint() {
    let t = parse("((λ (x) (x x)) (λ (x) (x x)))").unwrap();
    let next = t.reduce_step(&Strategy::Normal).unwrap();
    assert_eq!(t, next, "Omega reduces to itself");
}

#[test]
fn omega_never_reaches_nf() {
    let t = parse("((λ (x) (x x)) (λ (x) (x x)))").unwrap();
    // After 10 steps, still the same term
    let mut current = t.clone();
    for _ in 0..10 {
        current = current.reduce_step(&Strategy::Normal).unwrap();
    }
    assert_eq!(t, current);
}

// ── Alpha conversion error messages ─────────────────────────

#[test]
fn alpha_capture_error_is_descriptive() {
    let t = parse("(λ (y) (x y))").unwrap();
    let err = t.alpha_convert("x").unwrap_err();
    assert!(err.contains("capture"), "Error should mention capture: {}", err);
    assert!(err.contains("x"), "Error should mention the conflicting variable: {}", err);
}

#[test]
fn alpha_on_non_lambda_error() {
    let t = parse("(f x)").unwrap();
    let err = t.alpha_convert("y").unwrap_err();
    assert!(err.contains("λ-abstraction"), "Error: {}", err);
}

// ── Strategy-specific multi-step reduction ───────────────────

#[test]
fn cbv_reduces_arg_before_function_app() {
    // (λx.x) ((λy.y) z): CBV reduces inner first, then outer
    let t = parse("((λ (x) x) ((λ (y) y) z))").unwrap();

    // Step 1: reduce inner ((λy.y) z) → z
    let s1 = t.reduce_step(&Strategy::CallByValue).unwrap();
    assert_eq!(s1.display_string(), "((λ (x) x) z)");

    // Step 2: reduce outer ((λx.x) z) → z
    let s2 = s1.reduce_step(&Strategy::CallByValue).unwrap();
    assert_eq!(s2.display_string(), "z");
}

#[test]
fn normal_reduces_outer_before_inner() {
    // (λx.x) ((λy.y) z): Normal reduces outer first → ((λy.y) z) → z
    let t = parse("((λ (x) x) ((λ (y) y) z))").unwrap();

    // Step 1: reduce outer ((λx.x) ((λy.y) z)) → ((λy.y) z)
    let s1 = t.reduce_step(&Strategy::Normal).unwrap();
    assert_eq!(s1.display_string(), "((λ (y) y) z)");

    // Step 2: reduce remaining → z
    let s2 = s1.reduce_step(&Strategy::Normal).unwrap();
    assert_eq!(s2.display_string(), "z");
}

#[test]
fn cbn_stops_at_weak_head_normal_form() {
    // (λx. ((λy.y) z)) is a WHNF — CBN won't reduce the body
    let t = parse("(λ (x) ((λ (y) y) z))").unwrap();
    assert!(t.reduce_step(&Strategy::CallByName).is_none(),
        "CBN should not reduce under lambda");
}
