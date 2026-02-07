---
layout: post
title: "Evaluation Order"
permalink: /evaluation-order
---

# CIS352 -- Spring 2026

[Link to code for these lecture notes](https://gist.github.com/kmicinski/7042e0c2143f3fa2236cfc4517a684e8)

## Today's agenda...
- □ Evaluation Order
- □ Printf-style debugging and reachability hypotheses
- □ Evaluation of function call
- □ The Stack
- □ Tail Position and Tail Calls
- □ Tail Call Optimization (TCO) and Tail Recursive Functions

---

## Evaluation Order

Evaluation order refers to the order in which certain subexpressions in our program are *evaluated*. Evaluation happens in small steps: at the most basic level, the machine executes instructions on a multi-GHz clock. It is crucial for debugging and code comprehension to have a firm understanding of evaluation order. Exactly when and where expressions in our code are evaluated is something we need to make rigorous. We will do so soon by writing interpreters, but we will talk about it informally for now.

**QuickAnswer:** what is printed to the console for the following fragment?

```racket
;; displayln x to the console, then return y
(define (display-and-return x y)
  (displayln x)
  y) ;; note: the above definition builds an implicit (begin ...)

(define (my-plus x y)
  (displayln "adding")
  (+ x y))

(displayln (my-plus (display-and-return 5 3)
                    (display-and-return 10 4)))
```

When we refer to "control" in the context of programming languages, we are often appealing to the idea that we are "focusing" on some subcomputation at each point. In functional languages, the notion of control is generally: what subexpression is being evaluated. In assembly language, the notion of control is %rip, i.e., the instruction pointer, which always points at the instruction currently being executed.
It is a crucial skill to be able to trace through the program's control flow and to be able to identify what is happening.

For example, let's look at the following program:

```racket
(define (foo x)
  (match x
    [`(,xs ...) (first xs)]
    ['() 0]))
;; (foo '())
```

---

## Printf-style debugging and reachability hypotheses

The program is broken, which we can see by uncommenting the line.

Let's imagine a hypothetical student, who looks at the code and doesn't understand what is broken. The student looks at the code and thinks: "I don't understand what's going on, I handle the empty list and return zero!"

To fix the program, we have to understand what is happening. This involves being able to look at the program and trace its control flow, explaining what is going on at each point.

We can often make the problem easier by using "printf"-style debugging: we instrument our code with print statements to understand what is happening and where.

We need to start by making a hypothesis. When you are confused about a bug, the first kind of hypothesis you should make is a reachability hypothesis: a prediction that some code is / isn't executed. The reason these hypotheses are so useful is that they are easy to falsify: we add a print statement to our code.

In this case, let's play the role of the student and make a hypothesis: "the case for '() is (or isn't) reached." Now, let's update our code:

```racket
(define (foo+ x)
  (match x
    [`(,xs ...) (first xs)]
    ['() (displayln "here!") 0]))
;; (foo+ '())
```

We can see: even though the code crashes, "here!" is never printed! Now we might be confused, so we could move the print statement to the beginning of the function, where it would be printed.

Now we ask: why is the code not even reached? The answer in this case is that the match statements are in the wrong order: the first subsumes the second. If we truly wanted to see that, we could rearrange to print "here!" (or something else) at the beginning of that branch.

When your code is broken, reachability hypotheses are one of the best tools at your disposal, because they allow you to gradually adjust the printing. As long as you can keep rerunning quickly, this can be an effective debugging technique. Of course, this style merely emulates step-based debugging (e.g., gdb), but: (a) many languages lack good debuggers and (b) sometimes running a debugger is not practical (such as in production environments where you can only emit logs).

Building intuition about control flow is closely related to printf-style debugging: when we encounter bugs in our code, incorrect control flow is often the culprit.

---

## Evaluation of function calls

To evaluate a function call, `(e-f e0 e1 ...)`, we:

- Evaluate the function expression `e-f`, which might be trivial (if it is just a variable), but could involve arbitrary computation.

- Evaluate each argument of the function down to a *value* (i.e., a result).

- Build an environment for the function's body to execute, by binding the formal parameters (among other things, which we will discuss later).

- Evaluate the function's body, awaiting a return value.

**QuickAnswer**

What is printed? Reminder: `(begin e0 ... en)` evaluates `e0 ...` for their effect, then returns the result of evaluating `en`.

```racket
(displayln
 ((begin (displayln 2) (lambda (x) (begin (displayln x) (+ x 3))))
  (begin (displayln 5) 3)))
```

To facilitate calling a function, the computer has to remember where to return. For example, consider the following functions:

```racket
(define (f x y)
  (- x y))

(define (h x y)
  (f (f x y) (f y x)))
```

The point is that--during execution--the call to `(h 1 2)` executes in fine-grained steps, evaluating one expression at a time. For example, when we call `(h x y)`, many things happen:

1. Enter body of `(h x y)`, await its return value
2. To evaluate `(f (f 1 2) (f 2 1))`:
   3. First evaluate `f`: it is an identifier, so that is trivial.
   4. Next, evaluate `(f 1 2)`, await its return value...
      5. To evaluate `(f 1 2)`:
         6. First evaluate `f`, `1`, `2`, all variables / constants
         7. Now, evaluate the body of `f`, await its return value:
            8. To evaluate `(- 1 2)`, the body of `f`...
               9. Applying the builtin `-` results in `-1`
      10. Because `(- 1 2)` was the body of `f`, return `-1` from `f`
   11. Next, evaluate `(f 2 1)`, await its return value
       12. First evaluate `f`, `2`, `1`
       13. Now, evaluate the body of `f`, await its return value:
           14. Evaluate `(- 2 1)` to `1`
   15. Now, take the results of both subexpressions (`-1` and `1`) and apply `(f -1 1)`:
       16. `(- -1 1)` results in `-2`
17. The return value from the whole computation is `-2`

This was a lengthy sequence, and I even skipped some steps, but the basic idea is to break the computation down into each atomic step.

One thing to notice is that the top-level call to `(f ...)` involves significant computation. Ultimately, the first call `(f x y)` returns `-1`--but it has to call `(f y x)` before it can begin to call `-` on the result. The reason is simple: we can't actually do the subtraction until we know which two values we're subtracting.

At various places in the code, we have to *suspend* the execution to *wait* for results. For example, the call `(f (f x y) (f y x))` has to: (a) evaluate `(f x y)`, which involves work of its own, then (b) evaluate `(f y x)`, and then (c) take the result of each of those subordinate expressions and apply `f` to it. In effect, what is happening is:

```racket
(define (h+ x y)
  (let ([a0 (f x y)]
        [a1 (f y x)])
    (f a0 a1)))
```

This code is in A-normal form, where all functions are called with "simple" arguments. You will not need to know A-normal form in this class (it is covered more seriously in CIS531, "Compiler Design," where it is related to SSA forms), but it is important to understand that we can apply transformations on our code to break it down so as to make all of the control as explicit as possible.

Let's work another example where we transform the code so that every function has only variables / constants as its arguments:

```racket
(define (demo l)
  (* (first l) (+ (second l) (third l))))
```

Notice how in `demo-anf`, we have "flattened" the order of evaluation, to make it *absolutely clear* where things happen. This translation is one of the key steps in the translation to machine code.

```racket
(define (demo-anf l)
  (let ([a0 (first l)]
        [a1 (second l)] ; newly-introduced "administrative" variable
        [a2 (third l)])
    (let ([a3 (+ a1 a2)])
      (* a0 a3))))
```

**QuickAnswer**

Transform the following function, `baz`, into `baz+`, a version of the function that satisfies the following constraint: every function's argument can only be a variable / constant.

```racket
(define (baz x l)
  (list-set l x (+ (list-ref l x) 10)))

(define (baz+ x l)
  'todo) ;; introduce new lets, think carefully about evaluation order
```

---

## The Stack

**(IMPORTANT!)** Here is one big high-level lesson to take away from today's lecture: the computer uses a *stack* to *remember where to return* during the execution of a function. This is an advanced concept, because it appeals (to some degree) to how the machine executes functions at the assembler level.

Say, for example, we have an expression such as:

```
(f (g x y) (h z))
```

While we are doing the call to `(g x y)`, we know we are *awaiting* its return value `v`, so that we may continue by calculating `(h z)`, and then finally applying `f` to the results of both evaluations.

The idea is this: at every point in time, we need to have some memory of "where to go next." When we're evaluating `(g x y)`, we know that what we're going to do next is (a) store the result and then (b) continue to `(h z)` and then (c) use the two results to call `f`, followed by (d) whatever we're doing with the result of the call to `f`.

The computer uses a *stack* (last in, first out queue) to do this: whenever we call a function, the computer's stack remembers where to return. For example, during the call to `(g x y)`, the stack--a special, designated portion of memory--encodes the information necessary to return from the call to `g` and continue the execution of the program.

This lesson is true across languages. For example, in this C code here, https://godbolt.org/z/zPPnK6rcG, we see the fibonacci function is compiled to use the `call` instruction in assembly. The call instruction pushes a "return point" on the computer's stack, and then jumps to the beginning of the function's body. By default, every time we call the function, it pushes a stack frame. Thus, functions with deep recursions will potentially use a lot of stack space.

```racket
(define (fac x)
  (match x
    [0 1]
    [n (* n (fac (- x 1)))]))
```

Let's consider `(fac 4)`, it will look like:

```
(fac 4) = (match 4 [0 1] [n (* n (fac (- 4 1)))])
        = (* 4 (fac (- 4 1)))
        = (* 4 (fac 3))
        = (* 4 (match 3 [0 1] [n (* n (fac (- 3 1)))]))
        = (* 4 (* 3 (fac (- 3 1))))
        = (* 4 (* 3 (fac 2)))
        = (* 4 (* 3 (match 2 [0 1] [n (* n (fac (- 2 1)))])))
        = (* 4 (* 3 (* 2 (fac (- 2 1)))))
        = (* 4 (* 3 (* 2 (fac 1))))
        = (* 4 (* 3 (* 2 (match 1 [0 1] [n (* n (fac (- 1 1)))]))))
        = (* 4 (* 3 (* 2 (* 1 (fac (- 1 1))))))
        = (* 4 (* 3 (* 2 (* 1 (fac 0)))))
        = (* 4 (* 3 (* 2 (* 1 (match 0 [0 1] [n (* n (fac (- 0 1)))])))))
        = (* 4 (* 3 (* 2 (* 1 1))))
        = (* 4 (* 3 (* 2 1)))
        = (* 4 (* 3 2))
        = (* 4 6)
        = 24
```

Notice how the expressions grow horizontally as the computation proceeds. This reflects the fact that as the recursion deepens, we keep pushing stack frames. Ultimately, the recursion bottoms out and returns 1: the stack consists of a long chain of `*`s, which are waiting to multiply up the partial results as the stack unwinds back down to the original return point (the caller of `(fac 4)`).

The key thing to understand is this: generally (there is an exception we'll discuss in a minute), when you call a function, you are *saving* a return point on the stack. This will genuinely cost memory: if you have a large operation with a lot of deep recursions, that could start to be quite expensive. In fact, this was considered a major contributor to why functional languages (LISP, etc.) were seen as slow: in a functional language, you use recursion in place of iteration. Loops are fast, and generally use `jump` or `goto` instructions: no matter how many elements the loop processes, it doesn't have any effect on the stack.

**QuickAnswer**

Compared to the input, `n`, what will the maximum stack depth of the following function be (give a worst-case bound, i.e., big-O):

```racket
(define (example l)
  (if (< l 1)
      1
      (example (/ l 2))))
```

**QuickAnswer**

Consider the following code:

```
(g (+ 1 (f x)) 5)
```

Assume that we are evaluating the call `(f x)`. Describe, in plain English, what the stack will look like. Be as descriptive as possible, and try (if possible) to give enough information that we would know how to do the rest of the execution without seeing the term itself.

---

## Tail Position and Tail Calls

We will now define a term that is ubiquitous in computing. With respect to some parent expression, a subexpression is in "tail position" when it is the last thing to be evaluated. For example, let's look at this function:

```racket
(define (f+ x a)
  (match x
    [0 a]
    [n (f+ (- x 1) (* x a))]))
```

With respect to the execution of the function `f+`, the call to `f+` is in tail position. Why is that the case? It's because after returning from the *call* `f+`, there is *nothing* left for the execution of `f+` to do other than to immediately return.

The call to `f+` in the above function is a special type of call. It is a *tail* call, since it's a *call* which is in tail position. Tail calls are handled in a special way. We'll discuss more in a second. For now, let's see some more examples of tail calls.

With respect to the `if` form, which calls are in tail position?

```racket
#;
(if (equal? (add1 0) 1)
    (f 2)
    (g 3))
```

- The calls to `equal?` and `add1` are NOT in tail position. After returning from either, computation must continue: a tail call's return value must be the return value of the entire expression.

- Both the call to `(f 2)` and `(g 3)` are tail calls: because the return values for *either* of them is the return value for the whole `if`.

Now, more practice: which of the following calls (if any) is a tail call?

```racket
#;
(let ([x (+ z z)])
  (* x (+ x 1)))
```

Answer: the call `(* x (+ x 1))` is the return value of the *entire let form*. However, the `(+ x 1)` and `(+ z z)` are *not* tail calls.

To make this more clear, let's rewrite the code to make the evaluation order explicit:

```racket
#;
(let ([x (+ z z)])
  (let ([a0 (+ x 1)])
    (* x a0)))
```

We can see now how `*` is the very last call in the `let` form.

**QuickAnswer**

Consider *all* possible calls in the code below, which calls are in tail position (we call these "tail calls")?

```racket
#;
(cond [(equal? x 3) (add1 x)]
      [(f x) (sub1 x)]
      [else (g (+ x 1))])
```

---

## Tail Call Optimization (TCO) and Tail Recursive Functions

Tail calls are optimized by the compiler. To understand why, let's consider the function we looked at before:

```racket
(define (f+ x a)
  (match x
    [0 a]
    [n (f+ (- x 1) (* x a))]))
```

Consider the call to `f+` in this function. The call is a tail call: from the perspective of the whole function `f+`, the return value of the inner call to `f+` is the return value from the *whole* function `f+`. If you don't understand this, stop and read it again carefully: the return value from the *recursive call* to `f+` _is_ the return value of the entire function.

Because of this, using the stack would be inefficient: the *only* thing the stack is doing--in a tail call--is to wait for the return value and then "copy along" the return value from `f+` so that it becomes *our* (the function currently being called, `f+` in the example) return value. Instead of pushing a stack frame--just to then copy over the result--tail call optimization says this:

**"Tail Call Optimization (TCO):"** TCO is an optimization wherein a tail call is replaced by a *goto*. Instead of saving a return point on the stack--only to then copy our callee's return value along--we will simply *not* save a return point on the stack, and will instead *jump* to the beginning of the function being called, leaving the stack alone. Now, the proximate return point will be *our* caller, and when that function eventually returns, it will return to *our* caller.

In many functional languages--including Racket--tail calls are *guaranteed* to be optimized: no tail call in Racket will *ever* push a stack frame. To be even more specific about it, the function above:

```racket
(define (f+ x a)
  (match x
    [0 a]
    [n (f+ (- x 1) (* x a))]))
```

will not push *any* stack frames. This is because--unlike the original factorial--the recursive call to `f+` acts more like a goto. In fact--in terms of its effect on the stack--the code above is essentially:

```python
acc = a
while (x != 0):
  x := x - 1
  a := x * a
```

We will not discuss how while loops are compiled, but the essence is this: at the end of the loop body, you go back up to the loop header (the check for `x != 0`).

Code that avoids pushing stack frames can be *significantly* faster; deep recursion is bad for performance for several reasons:

- (a) The stack is generally finite in size--it is actually possible to run out of stack space (a "stack overflow"), but there are ways around this.

- (b) If we push many frames onto the stack and then pop them off, we may evict useful data from the cache. When the recursion unwinds, that data will have been evicted and will need to be reloaded (assuming it is still needed).

Thus, it is reasonable to say:

> "When functional programmers say *loops*, they mean *tail-recursive functions*."

We call a function "tail recursive" when *every* recursive call is a tail call. Because every tail call (without exception) does not push (or pop) any stack frames, tail calls are effectively "free" with respect to stack usage (in terms of the assembly, they are `goto`s or `jmp` instructions rather than *calls*, which save a return address).

To be more specific, let's look at another function:

```racket
(define (list-length-direct l)
  (match l
    ['() 0]
    [`(,hd ,tl ...) (+ 1 (list-length-direct tl))]))
```

This function is *not* tail recursive. It uses normal, direct-style recursion. With respect to the size `n` of the list `l`, the stack usage of `list-length-direct` is O(n).

By contrast, the following function uses O(1) stack space:

```racket
(define (list-length-acc l acc)
  (match l
    ['() acc]
    [`(,hd ,tl ...) (list-length-acc tl (+ 1 acc))])) ;; list-length-acc is a tail call.
```

Compared to the direct-style implementation...

- The tail-recursive version `(list-length-acc l acc)` shifts the addition to *before* the call, by using an extra argument to represent the "accumulator."

- The direct-version builds up a large chain of `(+ 1 ...)` stack frames, waiting until the recursion finally bottoms out to "unwind" the stack, finally doing all of the `(+ 1 ...)`s.

**QuickAnswer**

Which of the following is tail recursive?

```racket
(define (h+ x y)
  (if (equal? x y)
      (+ 1 (h+ y x))
      (h+ y x)))

(define (g+ x y)
  (cond [(x y) (g+ y x)]
        [else y]))
```

**QuickAnswer**

Consider the following direct-style implementation of `(list-sum l)`:

```racket
(define (list-sum l)
  (match l
    ['() 0]
    [`(,hd ,tl ...) (+ hd (list-sum tl))]))
```

Please write `(list-sum-tail l acc)`, which takes an additional argument as an accumulator.

NOTE: In general, we *don't* want to expose the extra accumulator argument--it exposes an implementation detail to the caller of the function (they have to pass in 0 everywhere). But I am making it explicit to show the trick: make sure you make the recursive call in tail position, update the accumulator *before* (as an argument to) the call.

```racket
(define (list-sum-tail l acc)
  (match l
    ['() 'todo]
    [`(,hd ,tl ...) 'todo]))
```
