---
layout: lecture-explorer
title: "Church Encoding"
permalink: /church-encoding
---

We will now turn our attention to convincing ourselves of the Turing
completeness of the λ-calculus. We will do this by systematically
taking forms from the λ-calculus and explaining how they translate
into simpler forms. We will do this in a piecemeal style, starting
with relatively simple forms and then expanding to more complex forms.

## Numbers

First, we need to encode numbers. What do we mean when we say
"encode?" Well, informally, we mean that when we have a number like 8,
we have a corresponding λ-calculus object that corresponds to 8. To be
a bit more formal about it, we want a pair of functions `church->nat`
and `nat->church` such that you can "round-trip" them:

```
∀n : nat, (church->nat (nat->church n)) = n
```

We will give one encoding here. The encoding I will give is the
"standard" one, but it is important to recognize that there are
others. As long as we can define the two functions above in a
harmonious way (along with defining the other operations we need on
numbers), we should be good.

Our representation of the number `N : nat` will be: (λ(f) (fᴺ x)),
where fᴺ is N applications of f. To give a few specific examples:

```
(nat->church 0) = (λ (f) (λ (x) x))           ;; call f 0 times
(nat->church 1) = (λ (f) (λ (x) (f x)))        ;; call f 1 time
(nat->church 2) = (λ (f) (λ (x) (f (f x))))    ;; call f 2 times
(nat->church 3) = (λ (f) (λ (x) (f (f (f x))))) ;; call f 3 times
```

> QuickAnswer
>
> Convert the following Church-encoded number to a Racket number:
>     (λ (f) (λ (x) (f (f (f (f (f (f (f x)))))))))

> QuickAnswer
>
> What number does `(λ (f) (λ (x) x))` represent? What about `(λ (g) (λ (y) (g y)))`?
> Are these two terms α-equivalent?

One immediate point we should make is that it may take an arbitrary
amount of reductions (β, α, η) to get any given term into normal
form. For example, consider the following term. Click it to load it
in the explorer and reduce it to normal form:

<code class="le-term">((λ (n) (λ (f) (λ (x) (f ((n f) x))))) (λ (g) (λ (y) (g y))))</code>

After three β-reduction steps, you'll find that this reduces to `(λ
(f) (λ (x) (f (f x))))`, the Church encoding of 2. Even though the
original term doesn't look anything like the encoding of 2, it
*computes to* 2 after reduction.

This is just as true in the λ-calculus as it is in arithmetic: we know
that `8 = 7 + 1 = 7 + 1 + 0 = 7 + 1 + 0 + 0 = ...`. In general, there
are an infinite number of algebraic expressions which "are" (compute
to) 8. However, in gradeschool arithmetic, we can *always* simplify /
canonicalize the term (as long as we don't have any variables). By
contrast, in the λ-calculus, it might be the case that some evaluation
strategies will never reduce (e.g., because the term contains
Ω). However, Church-Rosser tells us that when we _do_ hit a normal
form, it will be unique (up to α equivalence).

> QuickAnswer
>
> Use β-reduction to reduce the following term to β-normal form:
>
> <code class="le-term">((λ (g) (λ (f) (λ (x) (f ((g f) x))))) (λ (z) z))</code>
>
> What Church-encoded number does it represent?


## Round-Tripping Numbers

Now let's say we wanted to write a Racket function which generates a
Church-encoded version of the number N. We can do that as follows:

```racket
(define (nat->church n)
  (define (h n)
    (match n
      [0 'x]
      [_ `(f ,(h (- n 1)))]))
  `(λ (f) (λ (x) ,(h n))))
(nat->church 4)
```

The function produces a quoted expression (i.e., a Racket quoted
list). If I actually want to *use* that in Racket, I can call `eval`
(you will rarely want to do this, so this is just for illustrative
purposes here):

```racket
(eval (nat->church 4) (make-base-namespace))
#<procedure>
```

The call to `(make-base-namespace)` is required to tell eval how to
interpret variables (in this case, there are no free variables).

Now, Racket has no way of "inspecting" the `#<procedure>`. In fact,
this is kind of a key point: it's never meaningful to inspect a
function's source code at runtime, because we can never meaningfully
compare two functions for identity without actually running them. So
the question is: how do we know that the result of our `nat->church`
is correct?

This turns out to be a tricky question, because what "correctness"
means here is ill-defined, until we give the other side of the
definition, `church->nat`. Let's do that now. To start, let's look at
the following term:

```
(λ (f) (λ (x) (f (f (f (f x))))))
```


Think a little bit about what this function _does_: it consumes a
function `f`, and returns a function which consumes an input `x` and
then returns the result of applying `f` four times on `x`. So here's
the trick: if we make `f` be the Racket function `add1`, then we get:

```
(λ (x) (add1 (add1 (add1 (add1 x)))))
```

Hopefully you will agree that this is equivalent to the function:

```
(λ (x) (+ x 4))
```

So now we can see: if we are given a number `N`, and we apply it on
Racket's `add1`, we get back a function which takes in `x` and adds
`N` to it. To get the whole number, we can just make that `x` be `0`!

Thus, we can implement `(church->nat N)` (where `N` is a
*church-encoded representation of some number N*) as follows:

```racket
(define (church->nat N) ((N add1) 0))
```

> QuickAnswer
>
> Using the definition of `church->nat` above, trace the evaluation of
> `(church->nat (lambda (f) (lambda (x) (f (f (f x))))))`.
> What Racket value does it produce?

Just to reiterate: there's no special reason that we *must* use this
encoding. This is just the most direct encoding. There are other
possible encodings that could work. Also, notice that nothing in
`church->nat` requires `N` to be a Church-encoded number: if we give
`church->nat` something that *isn't* a Church-encoded number, it will
just produce nonsense. In our next lecture, we'll start studying type
theory: we'll use type systems to ensure that everything is tied
together as it should be; for now, we'll just say: it only works if
you use this encoding.

## Church-encoding `add1`

So far, we've only discussed how to encode constants (natural numbers)
into the lambda calculus. But the great thing about the encoding is
that we should be able to take our program, encode each *fragment*
into the λ-calculus, and then "run" (via β-reduction, or just running
it in Racket) our program until we get an answer.

We can implement the `add1` function as follows:

```
add1 = (λ (N) (λ (f) (λ (x) (f ((N f) x)))))
```

Look at how it works:
- It consumes an argument `N`, and returns `(λ (f) (λ (x) (f ((N f) x))))`
- What is that result? Well, it takes `f` and `x` and:
  - `((N f) x)` applies `f` `N` times to `x` (this is the definition of N!)
  - Takes the result of that and applies `f` on it *one more* time.

> QuickAnswer
>
> Using the definition of `add1` above, what does `((N f) x)` evaluate
> to when N is the Church encoding of 3? (Hint: N = (λ (f) (λ (x) (f
> (f (f x)))))).)

This is a point that deserves some contemplation, so let's work
through it with a bit of practice. This will be our first
**compile-then-reduce** exercise: we take a Racket expression, compile
it to the λ-calculus by substituting encodings, and then reduce in the
explorer.

- Let's say we want to Church-encode `(add1 2)` into the λ-calculus.

- Single-argument application is already in the λ-calculus, so no
  issue there, but neither `add1` / `2` is in the lambda calculus, so
  we translate those.

- Substituting `add1` with the term above, and `2` with its definition yields:

<code class="le-term">((λ (N) (λ (f) (λ (x) (f ((N f) x))))) (λ (f) (λ (x) (f (f x)))))</code>

Click this to load it in the explorer and *prove* that it reduces to
the Church encoding of `3`.

> QuickAnswer
>
> Church-encode `(add1 0)` by substituting the encodings for `add1` and
> `0`. Then reduce the resulting λ-term to β-normal form. What Church
> numeral do you get?

## Aside: Understanding the Encoding

At this point, each of the individual steps might make sense, but it
all feels a little confusing. We've studied a lot in this course:
interpretation, semantics, lambda calculus, etc. Let me write out a
few bullet points that might make things more clear:

- Q: Why are we doing the encoding?

- A: To show ourselves how high-level programming constructs like
  numbers, multi-argument functions, recursion, etc. work in terms of
  the λ-calculus. Equivalently: to convince ourselves that the
  λ-calculus is capable of expressing these things.

Remark: things such as multi-argument might feel like relatively
low-level features, rather than high-level features. But remember: the
λ-calculus is exceptionally small, a minimal rewriting-based
system. In practice, however, encodings of certain things (e.g.,
numbers) will have very bad performance: in CIS531, we study
compilation down to x86-64, in which we have an efficient machine
representation of 64-bit integers.

- Q: Are we interpreting, like in project 3?

- A: No, we are actually *not* interpreting, like in project 3.

- Q: Why not? Are we compiling instead?

- A: Yes, in this lecture, the encoding we're discussing is tantamount
  to writing a compiler. As for why it's important, the reason is that
  doing this transformation allows us to write the encodings of *all*
  programs, even if they don't terminate. For example, while we can't
  interpret `(+ 1 Ω)` (where `Ω` is the omega term), we can still
  *encode* it. The result will be a term in the λ-calculus.

So, the encoding is defining how to translate / compile high-level
features from a Scheme-like language into the λ-calculus.

## Multi-argument functions via currying

In many languages, including Racket, functions often have a fixed
number of parameters. In fact--even though we haven't used
them--Racket also allows functions of an arbitrary number of
parameters: `((lambda l (second l)) 1 2 3)` returns `2`. This reveals
a pretty good intuition: a function of multiple parameters can take a
*list* or *tuple* of inputs. Soon enough, we'll start looking at type
theory--that will give us what we need to give a rigorous definition
of a tuple (or even list) type.

First: how do we handle `(lambda () e)`? This is not super common, but it
does sometimes happen in practical programs (e.g., thunks)
sometimes. To convert `(lambda () e)` into the λ-calculus, we just attach a
superficial "bogus" parameter: `(lambda (_) e)`. But we also have to
remember that everywhere we apply `(e)` with zero arguments (we can
always identify this just by looking at the code), we translate that
to `(e (lambda (x) x))`: i.e., since we know `e` will eventually be `(lambda (_)
e)` (with a bogus argument).

`(lambda (x) e)` is already encoded, so we just have to encode `e`. Then,
`(lambda (x y) e)` is encoded as `(lambda (x) (lambda (y) e))`, and calls `(e x y)`
are encoded `((e x) y)`. We repeat this trick for any number of
arguments, and that gives us the encoding of any fixed-arity function.

> QuickAnswer
>
> How would you encode the following Racket expression into the
> λ-calculus?
>
>     ((lambda (x y z) (x z)) a b c)
>
> (Hint: curry both the lambda and the application.)

## Addition (encoding `+`)

Since `+` and `*` both take two arguments, we can use currying: `((+
3) 4)`, where we then encode 3/4 using the representation we just
discussed. Actually in Racket, `+` can take any number of arguments,
or even be applied to a list: while we could encode these things in
principle, we'll skip it here to keep things simple. Let's start with
`(+ n k)`. To be the natural number `n+k`, the result needs to be the
function:

```
(λ (f) (λ (x) (f (f ... (f x)))))
               |
               v
              n+k times
```

So let's define `+` with the two arguments, `n` and `k`: `(λ (n) (λ
(k) (λ (f) (λ (x) ...))))`. There are a lot of lambdas here: `n` and
`k` are the arguments to `+`. That then returns the `(λ (f) (λ (x)
...))`. Now, to fill in the `...`, we want to apply `f` to `x` `n+k`
times: this is the definition of the number `n+k`. To do that, we can
just do `((k f) ((n f) x))`. Let's think carefully about what is going
on here; first, let's look at the argument to `(k f)`, which is `((n f)
x)`: this applies `f` `n` times to `x` (notice we have to be
careful about currying). That is a term that builds a big stack of
`f`s to materialize the number `n`: `(f (f ... n times ... (f
x)))`. That is passed to `(k f)`, which is the function that does `f`
`k` times to some argument; in this case, that argument is the big
stack of `n` calls to `f`. Then, `(k f)` does `f` `k` times to that
stack of calls to `f`, building a stack of `n+k` calls to `f`.

So, the full definition is:

```
+ = (λ (n) (λ (k) (λ (f) (λ (x) ((k f) ((n f) x))))))
```

Let's do a **compile-then-reduce** exercise for `(+ 2 1)`. We encode
`+`, `2`, and `1`, then apply:

<code class="le-term">(((λ (n) (λ (k) (λ (f) (λ (x) ((k f) ((n f) x)))))) (λ (f) (λ (x) (f (f x))))) (λ (f) (λ (x) (f x))))</code>

Click to load and reduce. You should arrive at the Church encoding of 3.

> QuickAnswer
>
> Trace the first two β-reductions of the `(+ 2 1)` term above by
> hand (substituting `n` and then `k`). What does the term look like
> after those two steps?

---

## Multiplication (encoding `*`)

The encoding for multiplication has a beautiful intuition based on
*function composition*. Recall that a Church numeral `N` is a function
that takes `f` and applies it `N` times. So what does `(N g)` give
us? It gives us a function that applies `g` N times.

Now, if `g` itself is `(K f)`, a function that applies `f` K
times, then `(N (K f))` is a function that applies "f-K-times" N
times. That's `f` applied `N * K` times total!

So, the definition is:

```
* = (λ (n) (λ (k) (λ (f) (λ (x) ((n (k f)) x)))))
```

Breaking it down:
- `(k f)`: a function that applies `f` K times
- `(n (k f))`: apply that "K-times" function N times, giving a function that
  applies `f` N*K times
- `((n (k f)) x)`: start with `x` and do it

> QuickAnswer
>
> In the definition of `*`, what would happen if we used `((k (n f)) x)`
> instead of `((n (k f)) x)`? Would we still get `n * k`?
> (Hint: is multiplication commutative?)

Let's verify with a **compile-then-reduce** exercise for `(* 2 3)`.
We expect the result to be the Church encoding of 6:

<code class="le-term">(((λ (n) (λ (k) (λ (f) (λ (x) ((n (k f)) x))))) (λ (f) (λ (x) (f (f x))))) (λ (f) (λ (x) (f (f (f x))))))</code>

Click to load in the explorer and step through the reduction. You
should eventually reach `(λ (f) (λ (x) (f (f (f (f (f (f x))))))))`,
which is the Church encoding of 6.

> QuickAnswer
>
> Using the definitions of `*` and `+`, explain (informally) why
> `(* n 0)` always reduces to the Church encoding of 0 for any Church
> numeral `n`. (Hint: what is `(0 f)` for any `f`?)

---

## Evaluating Arithmetic Expressions

Given the encodings we've discussed so far, we can now compile the
following language entirely into the λ-calculus:

```
e ::= n        – Literals
    | (+ e e)  – Addition
    | (* e e)  – Multiplication
    | (λ (x ...) e) — λs of any arity
    | (e ...) – Applications of any arity
```

The process is systematic: for each construct, we substitute its
encoding. Let's trace through a more complex example to see the full
compilation pipeline. Consider `(+ (* 2 1) 1)`:

**Step 1:** Encode the innermost subexpressions:
- `2` → `(λ (f) (λ (x) (f (f x))))`
- `1` → `(λ (f) (λ (x) (f x)))` (appears twice)

**Step 2:** Encode `(* 2 1)` by substituting the definition of `*` and currying:
```
((* 2) 1)
```

**Step 3:** Encode `(+ (* 2 1) 1)` by substituting the definition of `+`:
```
((+ (* 2 1)) 1)
```

The fully-encoded term is large, but it's pure λ-calculus: no numbers,
no `+`, no `*`: just lambdas, variables, and application.

> QuickAnswer
>
> Church-encode the expression `(+ 1 (+ 1 1))`. You don't have to
> write out the full λ-term: just show the structure of how `+`, `1`,
> and `1` get composed (using the curried form `((+ a) b)`).

This is the fundamental technique behind [Project 4]({{ '/projects/4' | relative_url }}):
we are writing a *compiler* that translates high-level programs into
the λ-calculus. Each construct in the source language has a
corresponding translation rule, and the compiler applies these rules
recursively over the AST.

---

## Beyond Numbers: Encoding Booleans

Next let's discuss the encoding of booleans. To start, we'll assume
that users of our language never break the types: `(+ 1 #f)` will encode
into the λ-calculus just fine, but it won't reduce to anything
meaningful. This is typical, because our compilation into the
λ-calculus is not *injective*: there's plenty of stuff in the
λ-calculus that has no sensible meaning in our translation. Our next
topic will be type systems, where a big goal will be to use proof
theory to ensure that we can only write well-typed things. But for
now, we will simply say that if you use the encoding incorrectly, you
get undefined behavior and the encoding won't work.

Now on to the discussion of booleans. How should booleans work? Well,
in the λ-calculus, the only way we can really "use" any data is by
*calling* it as a function. For a number, the way we "use" it is to
apply it to two (curried) arguments: `f` and `x`. The guarantee is
that if `N` *is* a number, then we should be able to call it with `f`
and `x` and it should apply `f` `N` times to `x`. For booleans, what
do we do instead?

The idea is that a boolean is a *selector*: it takes two
arguments, one for "what to do when true" and one for "what to do
when false," and picks one of them.

In a "pure" presentation of Church booleans, we could simply use:

```
#t ≡ (λ (a) (λ (b) a))       ;; select first argument
#f ≡ (λ (a) (λ (b) b))       ;; select second argument
```

This works perfectly in the pure λ-calculus, where reduction strategy
is flexible. But we have a problem: we are ultimately going to
*execute* our Church-encoded programs in Racket, which uses
call-by-value evaluation. Under CBV, both arguments to a function are
fully evaluated before the function body runs. So if we wrote `(if e0
e1 e2)` as `((e0 e1) e2)`, Racket would evaluate *both* `e1` and `e2`
before `e0` gets to select one. If one branch diverges, the whole
thing diverges even when we wouldn't have taken that branch.

This is a bit of a leaky abstraction: the thunking we are about to
describe is an artifact of running our encoding in a CBV language like
Racket, not something inherent to the λ-calculus itself. Under normal
order reduction, the simple encoding above would work fine, because
arguments are not evaluated until they are actually used.

To work around CBV, we wrap each argument in a **thunk** (a
zero-argument function). Instead of passing values directly, we pass
callbacks that *produce* values when called. The boolean then calls the
selected thunk, triggering evaluation of only the chosen branch.

Here is the encoding. We write it first at the "source level" (using
multi-arg functions and zero-arg calls), then show the compiled
λ-calculus version:

**Source level** (Racket-like):
```
#t ≡ (λ (when-true when-false) (when-true))
#f ≡ (λ (when-true when-false) (when-false))
```

`#t` takes two thunks and calls the first one. `#f` takes two thunks
and calls the second one.

**Compiled to λ-calculus** (currying multi-arg, encoding zero-arg
calls with a dummy argument):
```
#t ≡ (λ (tt) (λ (ff) (tt (λ (z) z))))
#f ≡ (λ (tt) (λ (ff) (ff (λ (z) z))))
```

The zero-argument call `(when-true)` becomes `(tt (λ (z) z))`: we
call `tt` with the identity function as a dummy argument (which will
be ignored by the thunk).

> QuickAnswer
>
> Given the Church encoding of `#t`, what does the following reduce to?
>
>     ((#t (λ (d) 42)) (λ (d) 0))
>
> (Substitute the definition of `#t` and reduce. Remember `d` is a
> dummy parameter that gets ignored.)

Now let's encode `not`. It takes a boolean and returns a new boolean.
The trick: pass the input boolean two thunks, one that returns
`#f`, and one that returns `#t`:

```
not ≡ (λ (b) (b (λ () #f) (λ () #t)))
```

If `b` is `#t`, it calls the first thunk → returns `#f`. If `b` is
`#f`, it calls the second thunk → returns `#t`.

> QuickAnswer
>
> Trace through the evaluation of `(not #t)` using the definitions
> above. Verify that it produces `#f`. (You can work at the "source
> level": no need to fully expand into λ-calculus.)

We can also define `zero?`, which tests whether a Church numeral is zero:

```
zero? ≡ (λ (n) ((n (λ (d) #f)) #t))
```

The idea: if `n` is `0`, then `((0 (lambda (d) #f)) #t)` applies `(lambda (d) #f)` zero
times to `#t`, returning `#t`. If `n` is anything
greater than 0, the function `(lambda (d) #f)` is applied at least once to
`#t`, and since it ignores its argument and returns `#f`, the
result is `#f`.

> QuickAnswer
>
> Using the definition of `zero?` above, what does `(zero? 0)` reduce
> to? What about `(zero? 3)`?
> (Hint: recall that `0 = (λ (f) (λ (x) x))` and
> `3 = (λ (f) (λ (x) (f (f (f x)))))`.)

---

## Encoding `if`, `and` / `or`

### Encoding `if`

Since booleans are already functions that select between two thunks,
encoding `if` is surprisingly simple:

```
(if e0 e1 e2)  compiles to  (e0 (λ () e1) (λ () e2))
```

That's it! We pass the condition `e0` two thunks: one wrapping the
"then" branch and one wrapping the "else" branch. Since `#t` calls
the first thunk and `#f` calls the second, the boolean *is* the
if-expression.

After compiling to pure λ-calculus (currying the multi-arg
application, encoding zero-arg lambdas with a dummy parameter):

```
(if e0 e1 e2)  compiles to  ((e0 (λ (d) e1')) (λ (d) e2'))
```

where `e1'` and `e2'` are the Church-encoded versions of `e1` and `e2`.

Let's do a **compile-then-reduce** exercise for `(if #t 1 0)`:

1. Encode `#t` → `(λ (tt) (λ (ff) (tt (λ (z) z))))`
2. Wrap `1` in a thunk → `(λ (d) (λ (f) (λ (x) (f x))))`
3. Wrap `0` in a thunk → `(λ (d) (λ (f) (λ (x) x)))`
4. Assemble: `((#t thunk1) thunk2)`

<code class="le-term">(((λ (tt) (λ (ff) (tt (λ (z) z)))) (λ (d) (λ (f) (λ (x) (f x))))) (λ (d) (λ (f) (λ (x) x))))</code>

Click to load and reduce. You should reach `(λ (f) (λ (x) (f x)))` —
the Church encoding of 1. The `#f`-wrapping thunk is never called.

> QuickAnswer
>
> What would `(if #f 1 0)` reduce to? Try modifying the term above by
> swapping `#t` for `#f`: replace `(tt (λ (z) z))` with `(ff (λ (z) z))`
> in the boolean definition.

### Encoding `and` / `or`

Once we have `if`, encoding `and` / `or` is a simple desugaring:

```
(and e0 e1)  ≡  (if e0 e1 #f)
(or  e0 e1)  ≡  (if e0 #t e1)
```

The base cases are: `(and)` = `#t` and `(or)` = `#f`. For more than
two arguments, we desugar recursively:

```
(and e0 e1 e2 ...)  ≡  (if e0 (and e1 e2 ...) #f)
(or  e0 e1 e2 ...)  ≡  (if e0 #t (or e1 e2 ...))
```

> QuickAnswer
>
> Using the desugaring above, what does `(and #f anything)` reduce to
> for any expression `anything`? What about `(or #t anything)`?
> (Hint: `(if #f e1 #f)` → `#f` regardless of `e1`.)

This is exactly the approach used in [Project 4]({{ '/projects/4' | relative_url }}):
`and` and `or` are not primitive: they're syntactic sugar that gets
desugared into `if` before compilation to the λ-calculus.

---

## Encoding Cons Cells and Lists

Now we can encode numbers and booleans. Next, let's tackle lists. In
Racket, lists are built from two constructors: `cons` (which builds a
pair) and `null` (the empty list, `'()`). We observe them with `car`,
`cdr`, and `null?`.

The encoding follows the same **observer pattern** we used for
booleans. A boolean takes two callbacks ("what if true" and "what if
false") and calls one. Similarly, a cons cell or null value will take
two callbacks ("what if cons" and "what if null") and call the
appropriate one.

### Encoding `cons` and `null`

A cons cell stores two values `a` and `b`. When observed, it passes
them to the "when-cons" callback:

```
cons ≡ (λ (a b) (λ (when-cons when-null) (when-cons a b)))
```

In pure λ-calculus (after currying):
```
cons ≡ (λ (a) (λ (b) (λ (wc) (λ (wn) ((wc a) b)))))
```

The empty list, `null`, has no stored values. When observed, it calls
the "when-null" callback (a thunk):

```
null ≡ (λ (when-cons when-null) (when-null))
```

In pure λ-calculus:
```
null ≡ (λ (wc) (λ (wn) (wn (λ (z) z))))
```

### Encoding `car`

To extract the first element of a pair, we pass a cons cell two
callbacks: one that takes `a` and `b` and returns `a`, and a dummy
"when-null" callback (since `car` of `null` is undefined):

```
car ≡ (λ (p) (p (λ (a b) a) (λ () (λ (x) x))))
```

How does this work? `car` takes a pair `p` and calls it with two
callbacks. The first callback `(λ (a b) a)` selects the first element.
The second callback `(λ () (λ (x) x))` is a dummy for the null case
(which should never happen if `p` is a cons cell). Since a cons cell
always invokes `when-cons`, the first callback fires and returns `a`.

### Compile-then-reduce: `(car (cons 1 2))`

Let's trace `(car (cons 1 2))`:

**Step 1:** `(cons 1 2)` compiles to a function that, when given
`when-cons` and `when-null`, calls `(when-cons 1 2)`:
```
(λ (wc) (λ (wn) ((wc (λ (f) (λ (x) (f x)))) (λ (f) (λ (x) (f (f x)))))))
```

**Step 2:** `car` passes this a callback `(λ (a) (λ (b) a))` and a
dummy. The cons cell calls the callback with `a = 1` and `b = 2`,
selecting `a`.

The result is `(λ (f) (λ (x) (f x)))`, the Church encoding of 1.

> QuickAnswer
>
> Using the same observer pattern as `car`, write the encoding for
> `cdr`. (Hint: what do you change in the `when-cons` callback to
> select the *second* element instead of the first?)

> QuickAnswer
>
> Write the encoding for `null?`. It should return `#t` when given
> `null` and `#f` when given a cons cell. (Hint: what should each
> callback return? The `when-cons` callback can ignore its arguments.)

> QuickAnswer
>
> Notice a pattern: numbers take `f` and `x`; booleans take
> `when-true` and `when-false`; lists take `when-cons` and
> `when-null`. What is the common structure behind all three Church
> encodings? (Hint: think about how we "observe" or "use" each type
> of data.)

---

## Encoding `letrec`

We've now covered numbers, booleans, conditionals, and lists. The last
major feature we need to encode is **recursion**. In Racket, recursion
is typically written using `letrec`:

### Aside: what is `letrec`, a few examples

Recall `letrec` from Racket:

```racket
(letrec ([factorial (lambda (n)
                      (if (zero? n) 1 (* n (factorial (- n 1)))))])
  (factorial 5))
;; → 120
```

The key property of `letrec` is that the binding is *in scope within
its own definition*. Compare with `let`:

```racket
(let ([f (lambda (x) (f x))]) (f 0))  ;; ERROR: f is not defined yet!
```

With `let`, the right-hand side `(lambda (x) (f x))` can't refer to
`f` because `f` hasn't been bound yet. `letrec` fixes this by making
the binding available in its own definition, enabling recursion.

> QuickAnswer
>
> Why can't `let` be used for recursion? What goes wrong if we try
> `(let ([f (lambda (x) (f x))]) (f 0))`?

So how do we compile `letrec` into the λ-calculus? There's no notion
of "named definitions" or "self-reference" in the λ-calculus. We need
a trick.

### The U Combinator

The trick starts with an observation about self-application. Consider
the term <code class="le-term">((λ (x) (x x)) (λ (x) (x x)))</code>.
This is Ω, which we encountered before: it reduces to itself
and loops forever.

But what if, instead of blindly self-applying, we do something *useful*
with the copy of ourselves? Consider:

```
(λ (self) (λ (n) ... (self self) ...))
```

The idea: `self` is a *copy of the function itself*. Whenever we want
to recurse, we call `(self self)` to get another copy. This is
self-application used productively rather than divergently.

Let's see this concretely. Suppose we want to write `add-to-zero`:
a function that takes a Church numeral and adds 1 to 0 that
many times (effectively an identity on Church numerals, but
computed recursively). Without named recursion, we can write:

```
((λ (self) (self self))
 (λ (self) (λ (n)
   (if (zero? n) 0 (add1 ((self self) (sub1 n)))))))
```

The outer `(λ (self) (self self))` is called the **U combinator**. It
takes a function and passes it to itself. The inner function receives
`self` (a copy of itself) and uses `(self self)` whenever it needs to
recurse.

This works, but it's ugly: every recursive call has to say `(self
self)` instead of just `(self)`. Can we clean this up?

### The Y Combinator

The Y combinator abstracts away the `(self self)` pattern, giving us a
clean interface for recursion. Here it is:

```
Y = ((λ (u) (u u))
     (λ (y) (λ (mk) (mk (λ (x) (((y y) mk) x))))))
```

The Y combinator has this crucial property:

```
(Y f) = (f (Y f))
```

In other words, `(Y f)` is a **fixed point** of `f`: it's a value
that, when passed to `f`, gives back itself. This is exactly what we
need for recursion!

Here's how it works, step by step:
1. `(Y f)` reduces to `(f (λ (x) ((Y f) x)))`: the argument to `f` is
   a thunk that, when called with `x`, recursively invokes `(Y f)` on `x`.
2. Inside `f`, this argument acts as the "recursive callback": calling
   it triggers another round of `(Y f)`.
3. The η-expansion `(λ (x) (((y y) mk) x))` instead of just `((y y) mk)`
   is **crucial** for call-by-value evaluation. Without it, the
   self-application `(y y)` would be evaluated immediately, causing
   infinite recursion before `f` even gets called. The extra lambda
   delays the self-application until an argument `x` is actually
   provided.

> QuickAnswer
>
> Why is the η-expansion `(λ (x) (((y y) mk) x))` crucial in the Y
> combinator under call-by-value? What would happen if we used
> `((y y) mk)` directly?

### Using the Y Combinator to Compile `letrec`

With the Y combinator, the compilation of `letrec` is straightforward:

```
(letrec ([f (lambda (args ...) body)]) e)
```

compiles to:

```
(let ([f (Y (lambda (f) (lambda (args ...) body)))]) e)
```

The outer `(lambda (f) ...)` wraps the original function, giving it
access to a "recursive callback" parameter named `f`. The Y combinator
ensures that this callback *is* the function itself.

Let's trace through factorial:

```racket
(letrec ([fact (lambda (n)
                 (if (zero? n) 1 (* n (fact (sub1 n)))))])
  (fact 5))
```

This compiles to:

```racket
(let ([fact (Y (lambda (fact)
                 (lambda (n)
                   (if (zero? n) 1 (* n (fact (sub1 n)))))))])
  (fact 5))
```

Which desugars the `let` to:

```racket
((lambda (fact) (fact 5))
 (Y (lambda (fact)
      (lambda (n)
        (if (zero? n) 1 (* n (fact (sub1 n))))))))
```

The `Y` ensures that every reference to `fact` inside the body points
back to the function itself. When `(fact 5)` is called, it checks if
`n = 0`; if not, it calls `(fact (sub1 n))`, which triggers another
round through the Y combinator.

The fully church-encoded version of this term is very large: all of
`Y`, `zero?`, `*`, `sub1`, and the Church numerals get expanded into
pure λ-calculus. In [Project 4]({{ '/projects/4' | relative_url }}), the `church-compile` function handles
this by wrapping the program in a `let` that binds all the builtins:

```racket
(let ([Y-comb ...]
      [zero? ...]
      [+ ...]
      [* ...]
      [add1 ...]
      [sub1 ...]
      [cons ...]
      [car ...]
      [cdr ...]
      [null? ...]
      [not ...])
  program)
```

Then the `churchify` function recursively translates this entire
expression, including the builtin definitions, into pure λ-calculus.

> QuickAnswer
>
> Show how to compile the following `letrec` using the Y combinator:
>
> ```racket
> (letrec ([double (lambda (n) (if (zero? n) 0 (add1 (add1 (double (sub1 n))))))])
>   (double 3))
> ```
>
> Write the result using `let` and `Y` (you don't need to expand everything into λ-calculus).

---

## Summary

We've now seen how to Church-encode an entire programming language
into pure λ-calculus:

| Source construct | Encoding strategy |
|---|---|
| Natural numbers | `N = (λ (f) (λ (x) (fᴺ x)))` |
| `add1`, `+`, `*` | Manipulate the `f`/`x` structure |
| Booleans | `#t`/`#f` select between two thunks |
| `if` | `(e0 (lambda () e1) (lambda () e2))` |
| `and`/`or` | Desugar to `if` |
| Cons cells | Observer with `when-cons`/`when-null` callbacks |
| Multi-arg functions | Currying |
| `letrec` | Y combinator provides fixed-point recursion |

The key insight is the **observer pattern**: every data type is a
function that takes callbacks, one per constructor, and calls the
appropriate one. Numbers, booleans, and lists all follow this pattern.

This encoding shows that the λ-calculus, despite having only three
syntactic forms (variables, abstraction, application) and one
computational rule (β-reduction), can express everything that Racket
or any other Turing-complete language can express. In [Project
4]({{ '/projects/4' | relative_url }}), you will implement this as a
compiler: a function that translates high-level Scheme programs into
pure λ-calculus.
