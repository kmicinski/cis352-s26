---
layout: post
title: "Natural Deduction and Language Semantics"
permalink: /natural-deduction
proof_tree: true
---

<style>
/* ── Buttons ──────────────────────────────────────── */
.cl-btn {
  display: inline-block;
  padding: 7px 18px;
  font-size: 0.85em;
  font-weight: 500;
  font-family: inherit;
  color: #374151;
  background: #fff;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  text-decoration: none;
}
.cl-btn:hover {
  background: #f3f4f6;
  border-color: #9ca3af;
}

/* ── Solutions ────────────────────────────────────── */
details {
  margin: 1em 0 1.5em;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow: hidden;
}
details summary {
  padding: 10px 16px;
  font-weight: 500;
  font-size: 0.92em;
  color: #4f46e5;
  background: #f9fafb;
  cursor: pointer;
  user-select: none;
  transition: background 0.15s;
  list-style: none;
}
details summary::-webkit-details-marker { display: none; }
details summary::before {
  content: "▸ ";
  font-size: 0.8em;
  margin-right: 4px;
}
details[open] summary::before { content: "▾ "; }
details[open] summary {
  border-bottom: 1px solid #e5e7eb;
  background: #f3f4f6;
}
details .solution-body {
  padding: 16px;
}

/* ── Proof rule blocks ────────────────────────────── */
.rule-block {
  margin: 1.2em 0;
  padding: 0 0 0.5em;
  border-bottom: 1px solid #f3f4f6;
}
.rule-block:last-child { border-bottom: none; }
.rule-name {
  font-weight: 600;
  font-size: 0.95em;
  color: #374151;
  margin-bottom: 4px;
}
.rule-desc {
  font-size: 0.9em;
  color: #6b7280;
  margin-bottom: 8px;
}

/* ── Exercise blocks ──────────────────────────────── */
.exercise-box {
  margin: 1.5em 0;
  padding: 1em 1.2em;
  background: #fefce8;
  border-left: 4px solid #fbbf24;
  border-radius: 0 8px 8px 0;
}
.exercise-box h4 {
  margin: 0 0 0.4em;
  font-size: 1em;
  font-weight: 600;
  color: #92400e;
}
.exercise-box p { margin: 0.3em 0; }

/* ── Editor containers ────────────────────────────── */
.editor-wrap {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  overflow: hidden;
  margin: 0.8em 0;
  min-height: 180px;
}
</style>

<script>
// Rule name lists for editor dropdowns
var _natRules = ['Zero', 'Add1'];
var _boolTypeRules = ['True', 'False', 'And', 'Or'];
var _boolEvalRules = ['True', 'False', 'And', 'Or'];
</script>

## Topics

- Natural Deduction
- Language Semantics via Natural Deduction
- From natural semantics to interpreters

[Lecture notes in Racket (.rkt)](https://gist.github.com/kmicinski/99b757a5009cfb7749948c90aa923159)

---

## Natural Deduction

Natural deduction is a style of mathematical reasoning which has an
especially close relationship with computing. We will use natural
deduction in this class to write proofs, specifications, and define
the semantics of programming languages which we build.

Our goal today is to learn the format of natural deduction
proofs. We define natural deduction as a method for
defining both specifications and proofs of propositions (rigorous
statements).

We will learn natural deduction in a way which also introduces
the upcoming homework on transitive closure.

### Defining the Natural Numbers

Let's start by defining proofs for natural numbers. There are two ways
to build any natural number:

1. The constant 0 is a natural number.
2. If `n` is a natural number, `(S n)` is also a natural number.

We saw how to define the natural numbers via a term algebra over
signature {0 &mapsto; 0, S &mapsto; 1}. Now we'll see another way: using
natural deduction rules.

<div class="rule-block">
<div class="rule-name">Zero</div>
<div class="rule-desc">The constant 0 is a natural number.</div>
<div data-proof-tree='((Zero :right) --- "0 : nat")' data-proof-tree-zoom="false"></div>
</div>

<div class="rule-block">
<div class="rule-name">Add1</div>
<div class="rule-desc">If n is a natural number, then (S n) is also a natural number.</div>
<div data-proof-tree='((Add1 :right) (--- "n : nat") --- "(S n) : nat")' data-proof-tree-zoom="false"></div>
</div>

In natural deduction, rules have the following format:

```
         Assumption 0, Assumption 1, ...
  Name  -----------------------------------
                    Conclusion
```

The rule says: "If we can prove Assumption 0, Assumption 1, ...,
then applying rule `Name` allows us to prove the Conclusion." Some
rules have no assumptions above the line -- we call these
"axioms," because they represent statements we can make
without requiring any proof.

The assumptions and conclusions are "propositions:" mathematical
statements. In this case, propositions are of the form `_ : nat`
which means "some object *has type* nat." You should read `:` as
"has type" and `nat` as "natural number."

### Example: Proving (S (S (S 0))) : nat

Let's use these rules to prove that `(S (S (S 0))) : nat` (i.e.,
"3 is a natural number"):

<div data-proof-tree='((Add1 :right) ((Add1 :right) ((Add1 :right) ((Zero :right) --- "0 : nat") --- "(S 0) : nat") --- "(S (S 0)) : nat") --- "(S (S (S 0))) : nat")'></div>

### Reading Rules Forward and Backward

The tricky part about natural deduction is that rules can be read
either forward *or* backward. Consider the `Add1` rule:

- (Reading "forward," top-to-bottom): "If we have a proof of
  `n : nat`, then we can apply Add1 and prove `(S n) : nat`."

- (Reading "backward," bottom-to-top): "If we want to prove
  `(S n) : nat` for some `n`, then we can apply Add1 and simplify our
  task to proving `n : nat` -- which may or may not be possible."

To build a complete proof using natural deduction, we need two things:

1. **(Local correctness)** Each rule must be applied correctly
2. **(Completeness)** All leaves of the proof tree must be axioms

Here is an example that breaks *local correctness*:

<div data-proof-tree='(("Add1 (WRONG)" :right) ((Zero :right) --- "0 : nat") --- "(S (S (S 0))) : nat")' data-proof-tree-zoom="false"></div>

This is not a valid usage of Add1: if we have `(S n)` on the
bottom, we need `n` on the top. Here the top is `0`, so the bottom
should be `(S 0)` -- instead, it has `(S (S (S 0)))`.

Here is an example that breaks *completeness*:

<div data-proof-tree='((Add1 :right) ((Add1 :right) (--- "(S B) : nat") --- "(S (S B)) : nat") --- "(S (S (S B))) : nat")'></div>

The top of this proof is incomplete: `(S B) : nat` has not been
proven. Each local application of `Add1` is correct, but we'd
eventually need to prove `B : nat`, which is not possible by any rule.

---

## Exercise 1

<div class="exercise-box">
<h4>QuickAnswer</h4>
<p>Apply the Add1 rule "forwards," deriving a conclusion from the
premise <code>(S (S (S n))) : nat</code>.</p>
</div>

## Exercise 2

<div class="exercise-box">
<h4>QuickAnswer</h4>
<p>Apply the Add1 rule "backwards," deriving a proof obligation from the
conclusion <code>(S (S 0)) : nat</code>.</p>
</div>

## Exercise 3

<div class="exercise-box">
<h4>QuickAnswer</h4>
<p>Using the rules Zero and Add1, build a complete proof of <strong>(S (S 0)) : nat</strong></p>
</div>

<div class="editor-wrap" id="nat-exercise"></div>

<script>
(function() {
  var container = document.getElementById('nat-exercise');
  ProofTree.createEditor(container, {
    tree: '(--- "(S (S 0)) : nat")',
    theoryRules: _natRules
  });
})();
</script>

<details>
<summary>Show solution</summary>
<div class="solution-body">
<div id="nat-exercise-solution"></div>
</div>
<script>
(function() {
  window._natExSexp = '((Add1 :right) ((Add1 :right) ((Zero :right) --- "0 : nat") --- "(S 0) : nat") --- "(S (S 0)) : nat")';
  var details = document.getElementById('nat-exercise-solution').closest('details');
  var rendered = false;
  details.addEventListener('toggle', function() {
    if (details.open && !rendered) {
      rendered = true;
      ProofTree.renderReadonly(window._natExSexp, document.getElementById('nat-exercise-solution'));
    }
  });
})();
</script>
</details>

---

## Booleans

Let's do another short example. We'll define rules for constructing
boolean expressions:

<div class="rule-block">
<div class="rule-name">True</div>
<div data-proof-tree='((True :right) --- "#t : bool")' data-proof-tree-zoom="false"></div>
</div>

<div class="rule-block">
<div class="rule-name">False</div>
<div data-proof-tree='((False :right) --- "#f : bool")' data-proof-tree-zoom="false"></div>
</div>

<div class="rule-block">
<div class="rule-name">And</div>
<div class="rule-desc">To show (and e0 e1) : bool, we must prove both e0 : bool and e1 : bool.</div>
<div data-proof-tree='((And :right) (--- "e0 : bool") (--- "e1 : bool") --- "(and e0 e1) : bool")' data-proof-tree-zoom="false"></div>
</div>

<div class="rule-block">
<div class="rule-name">Or</div>
<div class="rule-desc">To show (or e0 e1) : bool, we must prove both e0 : bool and e1 : bool.</div>
<div data-proof-tree='((Or :right) (--- "e0 : bool") (--- "e1 : bool") --- "(or e0 e1) : bool")' data-proof-tree-zoom="false"></div>
</div>

## Exercise 4

<div class="exercise-box">
<h4>QuickAnswer</h4>
<p>Using the rules above, build a proof of <strong>(and (or #t #f) #t) : bool</strong></p>
</div>

<div class="editor-wrap" id="bool-exercise"></div>

<script>
(function() {
  var container = document.getElementById('bool-exercise');
  ProofTree.createEditor(container, {
    tree: '(--- "(and (or #t #f) #t) : bool")',
    theoryRules: _boolTypeRules
  });
})();
</script>

<details>
<summary>Show solution</summary>
<div class="solution-body">
<div id="bool-exercise-solution"></div>
</div>
<script>
(function() {
  window._boolExSexp = '((And :right) ((Or :right) ((True :right) --- "#t : bool") ((False :right) --- "#f : bool") --- "(or #t #f) : bool") ((True :right) --- "#t : bool") --- "(and (or #t #f) #t) : bool")';
  var details = document.getElementById('bool-exercise-solution').closest('details');
  var rendered = false;
  details.addEventListener('toggle', function() {
    if (details.open && !rendered) {
      rendered = true;
      ProofTree.renderReadonly(window._boolExSexp, document.getElementById('bool-exercise-solution'));
    }
  });
})();
</script>
</details>

---

## Language Semantics via Natural Deduction

We will use natural deduction to specify the semantics (meaning) of
programming languages. This will allow us to rigorously make clear
*exactly* how programs are evaluated, to a degree that we could
rigorously test or even *prove* properties about programs in our
language.

Programming language semantics is a core topic in CIS352. There are
several methods for specifying semantics:

1. **Reference interpreter:** "The semantics is whatever Python does."
   - *Upside:* scales to complex languages
   - *Downside:* the host language may not have a well-defined meaning
     itself (e.g., C++ has undefined behavior)

2. **Denotational semantics:** Map the program into a mathematical
   domain.
   - *Upside:* mathematics is well-developed
   - *Downside:* complex for nontrivial features like loops

3. **"Big-step" / natural semantics:** Specify semantics via natural
   deduction.
   - *Upside:* we can often take the specification and directly build
     an interpreter (or compiler)
   - *Downside:* requires fully specifying the language, which may be
     impractical for truly production-grade languages

### A Small Boolean Language

When we specify a language by natural deduction, we define a
"reduction relation:"

> **e ⇓ v** &emsp; "expression e evaluates to value v"

where `e` is an expression (something to be evaluated) and `v` is a
value (a result). As our first example, consider a small boolean
language:

```racket
(define (expr? e)
  (match e
    ['true #t]
    ['false #t]
    [`(&& ,(? expr? e0) ,(? expr? e1)) #t]
    [`(|| ,(? expr? e0) ,(? expr? e1)) #t]
    [_ #f]))
```

This is the term algebra over {`true` arity 0, `false` arity 0,
`&&` arity 2, `||` arity 2}. For example:

```racket
(expr? '(&& (|| false true) (|| true false))) ;; => #t
```

Values will be Racket's booleans (`#t` and `#f`).
Now we define the semantics -- precisely when we can prove `e ⇓ v`:

<div class="rule-block">
<div class="rule-name">True</div>
<div data-proof-tree='((True :right) --- "true ⇓ #t")' data-proof-tree-zoom="false"></div>
</div>

<div class="rule-block">
<div class="rule-name">False</div>
<div data-proof-tree='((False :right) --- "false ⇓ #f")' data-proof-tree-zoom="false"></div>
</div>

<div class="rule-block">
<div class="rule-name">And</div>
<div class="rule-desc">Evaluate both subexpressions and combine with <code>and</code>.</div>
<div data-proof-tree='((And :right) (--- "e0 ⇓ v0") (--- "e1 ⇓ v1") "v = (and v0 v1)" --- "(&& e0 e1) ⇓ v")' data-proof-tree-zoom="false"></div>
</div>

<div class="rule-block">
<div class="rule-name">Or</div>
<div class="rule-desc">Evaluate both subexpressions and combine with <code>or</code>.</div>
<div data-proof-tree='((Or :right) (--- "e0 ⇓ v0") (--- "e1 ⇓ v1") "v = (or v0 v1)" --- "(|| e0 e1) ⇓ v")' data-proof-tree-zoom="false"></div>
</div>

Notice how in the `And`/`Or` rules we assume that there is a
definition of *and*/*or* above the line. We are implicitly assuming
a definition in the *metalanguage* (the language we use to define the
language). In this case, we use Racket's `and`/`or` as the
metalanguage, but in general the metalanguage will be "mathematics."

## Exercise 5

<div class="exercise-box">
<h4>QuickAnswer</h4>
<p>Write a proof that: <strong>(&& true true) ⇓ #t</strong></p>
</div>

<div class="editor-wrap" id="eval-exercise1"></div>

<script>
(function() {
  var container = document.getElementById('eval-exercise1');
  ProofTree.createEditor(container, {
    tree: '(--- "(&& true true) ⇓ #t")',
    theoryRules: _boolEvalRules
  });
})();
</script>

<details>
<summary>Show solution</summary>
<div class="solution-body">
<div id="eval-exercise1-solution"></div>
</div>
<script>
(function() {
  window._eval1Sexp = '((And :right) ((True :right) --- "true ⇓ #t") ((True :right) --- "true ⇓ #t") "#t = (and #t #t)" --- "(&& true true) ⇓ #t")';
  var details = document.getElementById('eval-exercise1-solution').closest('details');
  var rendered = false;
  details.addEventListener('toggle', function() {
    if (details.open && !rendered) {
      rendered = true;
      ProofTree.renderReadonly(window._eval1Sexp, document.getElementById('eval-exercise1-solution'));
    }
  });
})();
</script>
</details>

## Exercise 6

<div class="exercise-box">
<h4>QuickAnswer</h4>
<p>Write a proof that: <strong>(&& true (|| false false)) ⇓ #f</strong></p>
</div>

<div class="editor-wrap" id="eval-exercise2"></div>

<script>
(function() {
  var container = document.getElementById('eval-exercise2');
  ProofTree.createEditor(container, {
    tree: '(--- "(&& true (|| false false)) ⇓ #f")',
    theoryRules: _boolEvalRules
  });
})();
</script>

<details>
<summary>Show solution</summary>
<div class="solution-body">
<div id="eval-exercise2-solution"></div>
</div>
<script>
(function() {
  window._eval2Sexp = '((And :right) ((True :right) --- "true ⇓ #t") ((Or :right) ((False :right) --- "false ⇓ #f") ((False :right) --- "false ⇓ #f") "#f = (or #f #f)" --- "(|| false false) ⇓ #f") "#f = (and #t #f)" --- "(&& true (|| false false)) ⇓ #f")';
  var details = document.getElementById('eval-exercise2-solution').closest('details');
  var rendered = false;
  details.addEventListener('toggle', function() {
    if (details.open && !rendered) {
      rendered = true;
      ProofTree.renderReadonly(window._eval2Sexp, document.getElementById('eval-exercise2-solution'));
    }
  });
})();
</script>
</details>

---

## From Natural Semantics to Interpreters

A key insight of this course is that big-step natural deduction rules
can be read as a *recursive function*. Each rule tells us: "to
evaluate this kind of expression, evaluate the subexpressions and
combine the results." This is exactly what an interpreter does.

For our boolean language, the interpreter follows directly from the
rules:

```racket
;; interp : Expr → Value
(define (interp e)
  (match e
    ['true  #t]
    ['false #f]
    [`(&& ,e0 ,e1)
     (and (interp e0) (interp e1))]
    [`(|| ,e0 ,e1)
     (or (interp e0) (interp e1))]))
```

Notice the direct correspondence:

- The `True` rule says `true ⇓ #t` -- the interpreter returns `#t`
  for `'true`.
- The `And` rule says: evaluate `e0` to `v0`, evaluate `e1` to `v1`,
  return `(and v0 v1)` -- the interpreter does exactly that with
  recursive calls.

This pattern -- writing natural deduction rules, then translating
them directly into an interpreter -- is the central technique we will
use throughout the rest of CIS352. In our lecture on
[closure-creating interpreters]({{ '/closures' | relative_url }}), we
extend this idea to a much richer language with variables, lambdas,
closures, and environments.
