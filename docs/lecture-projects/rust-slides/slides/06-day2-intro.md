<!-- .slide: class="title-slide" -->
<span class="course-tag">CIS 352 &bull; Day 2</span>

# Rust, Day Two

## The abstractions that make it actually pleasant

<div class="footer">cis352-s26</div>

---

## Quick recap of Day 1

- **Cargo** is the build tool, package manager, test runner, formatter.
- **Immutable by default**, `mut` to opt in.
- **Enums + `match`** give you algebraic data types with exhaustiveness.
- **`Option<T>`** replaces `null`.
- **Ownership:** one owner, moved on assignment, dropped on scope exit.
- **Borrowing:** `&T` (shared) or `&mut T` (exclusive), never both.
- **Structs + `impl` blocks** tie data and behavior together.

Today we build on these to get **lifetimes, traits, generics, errors,
iterators, and concurrency.**

Note:
If any of these feel fuzzy, speak up now. Today is going to move fast
and everything builds on these pieces.

---

## Where we're heading

1. **Lifetimes** &mdash; how does the compiler know how long a borrow lasts?
2. **Traits &amp; generics** &mdash; Rust's version of interfaces + templates.
3. **Error handling** &mdash; `Result<T, E>` and the `?` operator.
4. **Iterators &amp; closures** &mdash; functional-style pipelines that compile down to loops.
5. **Modules &amp; testing** &mdash; organizing real code.
6. **Concurrency** &mdash; fearless threads, channels, `Arc<Mutex<T>>`.
7. **A peek at async, macros, and `unsafe`.**
