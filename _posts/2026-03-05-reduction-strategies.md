---
layout: lecture-explorer
title: "λ-Calculus: Reduction Strategies"
permalink: /reduction-strategies
---

The untyped λ-calculus is a term-rewriting-based system, in which the
program's "state" is captured completely by the term itself. In our
[closure-creating interpreters lecture]({{ '/closures' | relative_url }}), we saw a derivative system
of the λ-calculus, in which we defined the relation `ρ, e ⇓ v` via
natural deduction rules. A key insight of that lecture was that the
structure of the rules allowed us to write a function `(interp ρ e)`
(producing a value `v`) via recursion. However, in that lecture, we
have implicitly assumed a call-by-value evaluation strategy. We have
talked about evaluation order before informally (in [our lecture on
evaluation order]({{ '/evaluation-order' | relative_url }})), but in this lecture we will see how the untyped λ-calculus
is defined so as to be *agnostic* to evaluation strategy.

## "In-Context" β-Reduction

Recall that we defined β-reduction in our last lecture:

<div class="nd-rule">
<div class="nd-conclusion">((λ (x) e-b) e-a) <sup>β</sup>⟹ e-b [x ↦ e-a]</div>
<div class="nd-label">β</div>
</div>

However, we recall that for a given term, there may be numerous
β-redexes. A β-redex is an *opportunity* to apply β. However, the
reality is that a term of size `n` will (in general) have O(n)
β-redexes.

> QuickAnswer
>
> How many β-redexes are present in the following term? Click
> it to load it in the explorer, then hold <kbd>B</kbd> to reveal all
> redexes:
>
> <code class="le-term">((λ (x) ((λ (y) y) x)) ((λ (z) z) k))</code>

Because there are (in general) multiple possible β-redexes, we
_cannot_ write an interpreter for the λ-calculus with β-reduction
alone, at least using our same strategy we did before. The issue is
that--in our closure-creating interpreters lecture--the evaluation
relation `ρ, e ⇓ v` can be read as a *function*, accepting pairs of
environment / term and outputting values. However, because there are
multiple-possible β-redexes, it would seem reasonable to conclude that
the relation `β⟹` can *not* be a function.

We can formalize a notion of "in-context" β reduction via three rules.
These tell us that β can happen at the top level, or deep inside the
left or right side of an application:

<div class="nd-rules-row">
<div class="nd-rule">
<div class="nd-conclusion">((λ (x) e-b) e-a) <sup>β</sup>⟹ e-b [x ↦ e-a]</div>
<div class="nd-label">β-Here</div>
</div>
<div class="nd-rule">
<div class="nd-premise">e₀ <sup>β</sup>⟹ e₀'</div>
<div class="nd-conclusion">(e₀ e₁) <sup>β</sup>⟹ (e₀' e₁)</div>
<div class="nd-label">β-Left</div>
</div>
<div class="nd-rule">
<div class="nd-premise">e₁ <sup>β</sup>⟹ e₁'</div>
<div class="nd-conclusion">(e₀ e₁) <sup>β</sup>⟹ (e₀ e₁')</div>
<div class="nd-label">β-Right</div>
</div>
</div>

This allows us to use natural deduction rules to be specific about
what, specifically, is reduced. For example, click the term from the
QuickAnswer above to load it, hold <kbd>B</kbd> to see all redexes,
then press <kbd>Space</kbd> to step.

---

## Free Variables

So far, we have been intuitive about substitution, this lecture we
will learn that technically, we need to use *capture-avoiding
substitution*. For example:

```
  (λ (x) (λ (z) (a (z (λ (b) d))))) [ d ↦ (g g) ]
≡ (λ (x) (λ (z) (a (z (λ (b) (g g))))))
```

This case is easy: we scan through the term, copying it over--any
place we see `d`, we drop in `(g g)` instead. However, it turns out
the issue is a bit tricky in general:

```
  (λ (x) x) [ x ↦ k ]  ≢ (λ (x) k) ;; WRONG
```

Notice what we did in the above; we renamed the `x` to `k`, but the
issue is this: previously the term was the identity function, *now*
it's the function that accepts any argument and returns `k`. You might
say: "fine, but need to change it to `(λ (k) k)`, but now the problem
is this, `k` can never refer to anything *except for* the argument.

In general, we want to avoid variable "capture" when we do
substitution. In the context of the λ-calculus, a variable is captured
during substitution when a variable that was free in the substituted
term becomes bound by a binder in the context where the substitution
is inserted.

A variable is *free* when it is not bound:

- Any variable `x`, without context, is free. In other words, the set
  of free variables of the term `x` is simply the singleton, `{x}`.

- The free variables of `(e0 e1)` are the union of the free variables
  of `e0` and the free variables of `e1`.

- The free variables of `(λ (x) e)` are all free variables in `e`
  *except for* `x`. The lambda *binds* the variable `x`. In
  programming languages, we use the term "binder" to refer to a
  syntactic point within the code that establishes a variable binding,
  including a let, formal parameter, etc.

Here's the definition of freevars in Racket:

```racket
;; freevars : Expr → (Setof Symbol)
;; Return the set of free variables in a λ-calculus expression
(define (freevars e)
  (match e
    [(? symbol? x)    (set x)]
    [`(λ (,x) ,body)  (set-remove (freevars body) x)]
    [`(,e0 ,e1)       (set-union (freevars e0) (freevars e1))]))
```

Notice how the structure of this function mirrors the three cases of
the definition above. This is a common pattern in PL: we define a
mathematical concept by structural recursion, and the Racket
implementation follows the same structure.

Now that we have learned the definition of free variables, let's
practice a bit. Load each of these in the explorer and try to
identify the free variables yourself:

> QuickAnswer
>
> What are the free variables in the term:
> <code class="le-term">(λ (x) (x (λ (x) (x (y z)))))</code>

> QuickAnswer
>
> What are the free variables in the term:
> <code class="le-term">(((λ (x) (y x)) (λ (y) (x y))) (λ (z) z))</code>

It is best to think of free variables as "variables which will be
given a meaning later." For example, let's look at the following
expression, in Racket:

```
((λ (x) x) add1)
```

From the perspective of the λ-calculus, `add1` is a free variable. But
this isn't true in Racket--in Racket, `add1` is the function that adds
one to a number. So what's happening?

One way to think about the answer is this: because you use `#lang
racket` at the top of the file, the Racket language includes a large
standard library, which gives definitions to things like `add1`. One
mental model to understand this might be to imagine that `#lang
racket` is doing something like taking the above term and wrapping it
in a definition of the real `add1` implementation:

```
(let ([add1 ...]
      [+ ...]
	  ...)
((λ (x) x) add1))
```

Now we can see that free variables are important: they might have a
meaning *outside* of the current term, so we can't just arbitrarily
change them. For example, while the terms `(λ (x) x)` and `(λ (y) y)` are
equivalent, `(λ (x) add1)` and `(λ (x) y)` are *very* different terms. In
general, we do not want to touch free variables: we want to leave them
be.

---

## α-conversion

The λ-calculus actually has several different conversion rules. We
have mostly talked about β, which is the main driver of
computation--we will now learn about α-conversion, which allows us to
rewrite terms in a semantics-preserving way. We can use α-conversion
to rename a variable bound by a binder:

```
(λ (x) (x (λ (y) (λ (x) x)))) α[x ↦ z]⟹ (λ (z) (z (λ (y) (λ (x) x))))
```

Click the term <code class="le-term">(λ (x) (x (λ (y) (λ (x) x))))</code> to load it in
the explorer, then right-click on a λ binder to try α-conversion.

Notice how in the α-conversion above, we renamed the outermost binding
of `x` to the variable `z`. Doing so forced us to rewrite the first
variable reference (of `x`) to `z`, but **not** the second; this is
because the second occurrence of `x` is under a *new* binding for
`x`. In most programming languages, you can clobber a variable name
via nesting scopes. In the λ-calculus, this corresponds to re-binding
a variable when its binding is in scope, as in the following example:

<code class="le-term">(λ (x) (λ (x) x))</code> — here `x` refers to the *second* lambda.

Of course, this is not unique to the λ-calculus, it happens in many languages:

```racket
;; Racket: inner x shadows outer x
(let ([x 1])
  (let ([x 2])
    x))           ;; evaluates to 2
```

```haskell
-- Haskell
let x = 1 in let x = 2 in x  -- evaluates to 2
```

```java
// Java: parameter shadows field
class Example {
    int x = 1;
    void foo(int x) {
        System.out.println(x); // prints the parameter
    }
}
```

```c
// C: block scoping
int x = 1;
{
    int x = 2;
    printf("%d", x); // prints 2
}
```

```javascript
// JavaScript: block scoping with let
let x = 1;
{
    let x = 2;
    console.log(x); // prints 2
}
```

When we α-convert a term, we need to be careful to respect binders: if
we're α-converting `x`, we need to stop conversion when we hit another
`x`.

Here is α-rename and α-convert in Racket:

```racket
;; α-rename : Symbol Symbol Expr → Expr
;; Rename free occurrences of x to z within e
(define (α-rename x z e)
  (match e
    [(? symbol? y)    (if (equal? y x) z y)]
    [`(λ (,y) ,body)  (if (equal? y x)
                          e  ;; x is rebound here: stop
                          `(λ (,y) ,(α-rename x z body)))]
    [`(,e0 ,e1)       `(,(α-rename x z e0) ,(α-rename x z e1))]))

;; α-convert : Symbol Expr → Expr
;; Rename the outermost binder of a λ to z
(define (α-convert z e)
  (match e
    [`(λ (,x) ,body) `(λ (,z) ,(α-rename x z body))]))
```

Let's test it:

```racket
(check-equal? (α-convert 'z '(λ (x) (x y)))
              '(λ (z) (z y)))

(check-equal? (α-convert 'z '(λ (x) (x (λ (x) x))))
              '(λ (z) (z (λ (x) x))))
```

The second test is particularly instructive: notice that the inner
`(λ (x) x)` is left alone, because that `x` is bound by the inner λ,
not the outer one we are renaming.

---

## Capture-Avoiding Substitution

So far we have been a bit informal. It turns out that in our
definition of the β rule, we need to use *capture-avoiding*
substitution.  Capture-avoiding substitution makes a fairly simple
observation: we never want to perform a substitution that would change
a variable's binding status, we must never capture a variable that was
previously bound. Here are a few examples of where we cannot do
substitution naively:

```
;; Example 1: free variable captured by a binder
  (λ (y) x) [x ↦ y]
≢ (λ (y) y)              ;; WRONG: y was free, now it's bound

;; Example 2: free variable in substituted term gets captured
  (λ (y) (x y)) [x ↦ (y z)]
≢ (λ (y) ((y z) y))      ;; WRONG: the y in (y z) gets captured

;; Example 3: capture changes meaning of an application
  ((λ (y) x) x) [x ↦ y]
≢ ((λ (y) y) y)           ;; WRONG: identity applied to y ≠ original

;; Example 4: capture inside nested lambdas
  (λ (z) (λ (y) (x z))) [x ↦ (y y)]
≢ (λ (z) (λ (y) ((y y) z)))  ;; WRONG: y in (y y) captured by inner λ
```

In each case, the issue is that a variable which was *free* in the
term being substituted gets *captured* by a binder in the target
expression. After the substitution, the variable now refers to
something entirely different than it did before.

However, to avoid capturing variables inappropriately, we can use
α-conversion. This is because there will always be an unused variable,
so whenever substitution would cause a variable to be captured, we
just perform α-conversion so that the capture no longer occurs. There
are few places this might happen:

- When we substitute a term `e-a` into `(λ (y) body)`, and `e-a`
  contains a free variable named `y`. The binder `(λ (y) ...)`
  would capture the free `y` in `e-a`. **The fix:** α-rename `y` to a
  fresh variable before performing the substitution.

- When there are multiple nested binders, each of which could
  potentially capture a free variable in the substituted term. We must
  check each binder as we recurse into the term.

Here is capture-avoiding substitution, step by step, on Example 1:

```
;; Goal: (λ (y) x) [x ↦ y]
;; Problem: y is free in the substituted term, and the binder is (λ (y) ...)
;; Step 1: α-rename to avoid capture
  (λ (y) x) α[y↦y']⟹ (λ (y') x)
;; Step 2: now substitute safely
  (λ (y') x) [x ↦ y] = (λ (y') y)  ✓
```

And on Example 2:

```
;; Goal: (λ (y) (x y)) [x ↦ (y z)]
;; Problem: y is free in (y z), and the binder is (λ (y) ...)
;; Step 1: α-rename to avoid capture
  (λ (y) (x y)) α[y↦w]⟹ (λ (w) (x w))
;; Step 2: now substitute safely
  (λ (w) (x w)) [x ↦ (y z)] = (λ (w) ((y z) w))  ✓
```

> QuickAnswer
>
> Use capture-avoiding substitution to compute:
>
>     (λ (y) (y x)) [x ↦ (λ (z) y)]

> QuickAnswer
>
> Use capture-avoiding substitution to compute:
>
>     (λ (y) (λ (z) (x (y z)))) [x ↦ (y (λ (a) z))]
>
> Hint: which binders cause problems? You may need more than one α-rename.

---

## Advanced Aside: De Bruijn Indices

One source of complexity in the λ-calculus comes from variable names.
We just saw that substitution requires care to avoid capture, and
α-conversion exists precisely because variable names are, in some
sense, arbitrary. A natural question is: can we eliminate names
entirely?

The answer is yes. De Bruijn indices replace variable names with
*numbers* that indicate how many binders "up" a variable reference
points to. For example:

```
(λ (x) x)             becomes  (λ 0)
(λ (x) (λ (y) x))     becomes  (λ (λ 1))
(λ (x) (λ (y) y))     becomes  (λ (λ 0))
(λ (x) (λ (y) (x y))) becomes  (λ (λ (1 0)))
```

The number `0` means "the variable bound by the nearest enclosing λ,"
`1` means "the next one out," and so on. So in `(λ (λ 1))`, the `1`
refers to the *outer* λ, skipping past the inner one.

The key benefit: two terms that are α-equivalent (differ only in
variable names) get the *exact same* De Bruijn representation. For
example, `(λ (x) x)` and `(λ (y) y)` and `(λ (z) z)` all become `(λ
0)`. This means α-equivalence becomes *syntactic* equality, which is
very convenient.

The tradeoff is that De Bruijn terms are harder to read. The term
`(λ (λ (1 0)))` is much less clear than `(λ (x) (λ (y) (x y)))` if
you're a human trying to understand the code. For this reason, most
presentations aimed at humans (including this course) stick with named
variables, but real implementations of type checkers and compilers
frequently use De Bruijn indices under the hood.

---

## η-expansion and contraction

So far we have learned of two reduction rules: α (renaming) and β. We
will now discuss another, η. The η rule captures a notion of *function
extensionality*. In mathematics, function extensionality refers to the
principle that two functions are equal whenever their extensions
(behavior on all arguments) are equal. In other words, f = g whenever
∀x. f(x) = g(x). This is captured by the η rule:

<div class="nd-rules-row">
<div class="nd-rule">
<div class="nd-conclusion">(λ (x) (e x)) <sup>η</sup>⟹ e</div>
<div class="nd-label">η-contract</div>
</div>
<div class="nd-rule">
<div class="nd-side">x not free in e</div>
<div class="nd-conclusion">e <sup>η</sup>⟹ (λ (x) (e x))</div>
<div class="nd-label">η-expand</div>
</div>
</div>

We call these both "η-conversion," the first is contractive (makes the
term smaller), the second is expansive (makes the term
bigger). Because we can always expand, then contract, then expand,
etc. we can treat η as an equality. We will not use η very frequently,
however we will see at least one application of it in our lectures on
Church encoding.

---

## Reduction Strategies

The untyped λ-calculus is extremely economical, in the sense that it
is capable of expressing Turing-equivalent computation *just* by using
β. As we mentioned before, a term may have an arbitrary number of β
reductions. For example, the following term has six distinct
β-redexes. Click the term to load it, then hold <kbd>B</kbd> to see
all six highlighted:

<code class="le-term">((λ (a) ((λ (b) b) a)) ((λ (c) c) ((λ (d) d) ((λ (e) e) ((λ (f) f) x)))))</code>

There is a β-redex at the outermost application, one in the body
`((λ (b) b) a)`, and four nested applications on the right:
`((λ (c) c) ...)`, `((λ (d) d) ...)`, `((λ (e) e) ...)`, and
`((λ (f) f) x)`.

As we mentioned at the beginning of lecture, we can define a notion of
"in-context" β which allows us to be explicit about precisely what was
reduced. However, in practice, we don't want have to
nondeterministically "guess" which β rule to apply: we want the
programming language to make evaluation order explicit.

With respect to a function call, there are essentially two options:

- (Eager evaluation, "call by value"): Evaluate a function's arguments
  down to "values," (results), which are then bound to the formal
  parameters before jumping into the body.

- (Lazy evaluation, "call by name/need"): Instead of evaluating a
  function's arguments, bind the function's formal parameters to the
  *expressions* of the actual arguments.

To show an example, consider the following term. Let
Ω = <code class="le-term">((λ (z) (z z)) (λ (z) (z z)))</code>.
Notice that Ω is a β-redex which reduces to *itself*:
substituting `(λ (z) (z z))` for `z` in `(z z)` gives us
`((λ (z) (z z)) (λ (z) (z z)))` right back. So Ω loops forever.

Now consider applying a function that ignores its argument to Ω.
Click to load:
<code class="le-term">((λ (x) (λ (y) y)) ((λ (z) (z z)) (λ (z) (z z))))</code>

Under call-by-value, Ω is evaluated first (diverges). Under
call-by-name, the outer β fires immediately and Ω is discarded.
Try switching strategies in the explorer to see the difference.

This is a case where the evaluation strategy *matters*: call-by-value
diverges, but call-by-name successfully produces a result.

We will learn about four reduction strategies in this class, although
the last two are the only ones we implement in real programming
languages:

- **Normal order**: Always reduce the leftmost, outermost β-redex
  first. This reduces *under* lambdas. Normal order is guaranteed to
  find a normal form if one exists (a consequence of the Church-Rosser
  theorem), but it does reduce under lambdas, which is unusual for
  programming languages.

- **Applicative order**: Always reduce the leftmost, innermost β-redex
  first. This evaluates arguments before functions, and reduces
  *under* lambdas. Think of it as "evaluate from the inside out."

- **Call by value** (CBV): Like applicative order, but do *not* reduce
  under λ-abstractions. Evaluate arguments to *values*
  (λ-abstractions) before applying. This is what Racket, OCaml,
  Python, Java, C, and most mainstream languages use.

- **Call by name** (CBN): Like normal order, but do *not* reduce under
  λ-abstractions. Arguments are passed as unevaluated expressions.
  Haskell uses a variant called "call by need" which additionally
  memoizes the result of evaluating an argument the first time it is
  used, avoiding redundant work.

Load this term under each strategy and hold <kbd>B</kbd> to see which
redex each strategy picks:

<div class="le-strategy-group">
<button class="le-strat-btn" data-term="(((λ (x) (λ (y) (x y))) ((λ (a) a) z)) ((λ (b) b) w))" data-strategy="normal"><i class="fas fa-play"></i> Normal</button>
<button class="le-strat-btn" data-term="(((λ (x) (λ (y) (x y))) ((λ (a) a) z)) ((λ (b) b) w))" data-strategy="applicative"><i class="fas fa-play"></i> Applicative</button>
<button class="le-strat-btn" data-term="(((λ (x) (λ (y) (x y))) ((λ (a) a) z)) ((λ (b) b) w))" data-strategy="cbv"><i class="fas fa-play"></i> CBV</button>
<button class="le-strat-btn" data-term="(((λ (x) (λ (y) (x y))) ((λ (a) a) z)) ((λ (b) b) w))" data-strategy="cbn"><i class="fas fa-play"></i> CBN</button>
</div>

The distinction between these strategies comes down to two questions:
(1) Do we reduce arguments before applying a function? (2) Do we
reduce under λ-abstractions?

|                       | Reduce under λ? | Reduce arguments first? |
|-----------------------|:---------------:|:-----------------------:|
| Normal order          | Yes             | No (outermost first)    |
| Applicative order     | Yes             | Yes (innermost first)   |
| Call by value (CBV)   | No              | Yes                     |
| Call by name (CBN)    | No              | No                      |

---

## Why "not under lambdas?"

You may be wondering: why do real programming languages refuse to
reduce under λ-abstractions? It comes down to a simple idea: a
λ-abstraction is already a *value*. When we write
<code class="le-term">(λ (x) ((λ (y) y) x))</code>
in a real language, the language treats this as "a function"--it is
done. The body `((λ (y) y) x)` is not evaluated until the function
is actually *called* with an argument.

This makes practical sense. Consider:

```racket
(define (f x) (+ (* x x) 1))
```

When Racket encounters this definition, it creates a closure and moves
on. It does *not* try to "simplify" the body `(+ (* x x) 1)`. Why?
Because `x` isn't known yet--it's a parameter. There's nothing useful
to compute until someone actually calls `(f 5)`.

There's also an efficiency argument. If we reduced under lambdas, we'd
be doing work speculatively: computing inside function bodies that may
never be called. In a language like Racket, where functions are
first-class and passed around liberally, this would be enormously
wasteful.

Finally, reducing under lambdas can change the termination behavior of
programs. Some programs *only* terminate if we avoid reducing under
lambdas. The Ω example above is illustrative:
`(λ (x) (λ (y) y))` applied to Ω terminates under call-by-name
(which doesn't reduce the argument), but would loop under a strategy
that tried to fully reduce Ω first.

---

## The Church-Rosser Property

We have seen that a given term may have multiple possible β-redexes,
and that different reduction strategies may choose different redexes at
each step. This raises a natural question: if we reduce the same term
in two different ways, can we end up with *different* normal forms?

The answer is no, and this is guaranteed by the **Church-Rosser theorem**
(also called the *confluence* theorem):

**Theorem (Church-Rosser):** If a term `e` can be reduced (in zero or
more steps) to both `e1` and `e2`, then there exists a term `e3` such
that both `e1` and `e2` can be reduced to `e3`.

Visually, this looks like a diamond:

```
           e
          / \
         /   \
        e1    e2
         \   /
          \ /
           e3
```

No matter how we diverge in our reduction choices, we can always
converge again. An immediate consequence is:

**Corollary:** If a term has a β-normal form, it is *unique* (up to
α-equivalence).

This is reassuring. It means the λ-calculus, despite its
nondeterminism, is *not* ambiguous about results. If a computation has
an answer, that answer is the same regardless of evaluation strategy.

Let's see this concretely. Load the term
<code class="le-term">(((λ (x) (λ (y) (x y))) ((λ (a) a) z)) ((λ (b) b) w))</code>
under normal order and step to normal form. Then reload it under
applicative order and step again. You'll arrive at the same result
both ways — that's Church-Rosser in action.

<div class="le-strategy-group">
<button class="le-strat-btn" data-term="(((λ (x) (λ (y) (x y))) ((λ (a) a) z)) ((λ (b) b) w))" data-strategy="normal"><i class="fas fa-play"></i> Normal order</button>
<button class="le-strat-btn" data-term="(((λ (x) (λ (y) (x y))) ((λ (a) a) z)) ((λ (b) b) w))" data-strategy="applicative"><i class="fas fa-play"></i> Applicative order</button>
</div>

> QuickAnswer
>
> Load the term
> <code class="le-term">(((λ (x) (λ (y) x)) ((λ (a) a) z)) ((λ (b) (b b)) (λ (b) (b b))))</code>
> and try reducing it with **normal order** vs. **call by value**.
> Which strategy finds the normal form? Which one diverges? Why?

However, Church-Rosser does *not* guarantee that every strategy will
*find* the normal form. As we saw with the Ω example, call-by-value
can diverge on terms where call-by-name terminates. In fact, if any
strategy finds a normal form, normal order will also find it. This is
why normal order is sometimes called the "most powerful" reduction
strategy--but it comes at the cost of reducing under lambdas, which is
impractical for real languages.

---

## Advanced, Optional

Taking α, β, and η conversion together, we get an *equational theory*
over λ-calculus terms (modulo α-equivalence). Unfortunately, our
presentation here is a bit informal due to a complication: λ-calculus
binding is not algebraic, in the sense that ordinary free algebras are
not enough to capture binding structure. This is reflected by a lot of
semantic ugliness, like side conditions in our rules. More theoretical
treatments of the λ-calculus prefer to leverage more complex
representations, e.g., de-Bruijn indices or higher-order abstract
syntax (HOAS). These approaches require more mathematical complexity
from the start, but that is rewarded by a more fundamentally elegant
approach.
