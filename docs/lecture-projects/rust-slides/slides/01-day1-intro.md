<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 1 &bull; Chapter 1</span>

# Why Rust?

## The language, the promise, and the problem it solves

---

## What is Rust?

A **systems programming language** focused on three goals:

- **Safety** without a garbage collector
- **Speed** on par with C and C++
- **Concurrency** without data races

> *"A language empowering everyone to build reliable and efficient software."*
> &mdash; rust-lang.org

Note:
Rust has been voted the "most-loved language" on the Stack Overflow
developer survey every year from 2016 through 2023. That is a striking
amount of goodwill for a language that forces you to think hard about
memory.

---

## The memory-safety tax

Most languages pick two of three:

| &nbsp; | Fast | Safe | Low-level |
|---|:---:|:---:|:---:|
| C / C++ | yes | no | yes |
| Java / C# | mostly | yes | no |
| Python / JS | no | yes | no |
| Racket | no | yes | no |
| **Rust** | **yes** | **yes** | **yes** |

Rust's key idea: enforce safety *at compile time*, so the runtime pays nothing.

Note:
Most of this class has lived in Racket. Racket is safe because it has a
garbage collector and boxes values at runtime. Java and C# are similar.
C and C++ are fast and low level but hand you a loaded footgun. Rust's
bet is that a rich type system can give us the safety of Racket and the
speed of C, at the cost of making the programmer do more up-front work.

---

## Where Rust lives today

- **Linux kernel** (since 6.1, 2022): drivers, filesystems
- **Windows kernel**: font parsing, GDI, win32k
- **Firefox**: Servo, Stylo CSS engine, parts of Gecko
- **Cloudflare, Discord, Dropbox, Figma, 1Password**: infrastructure
- **Deno, Ruff, Turbopack, Biome, SWC**: the modern JS toolchain is Rust
- **Android**: now allows Rust alongside C++ in platform code

Note:
The point to hammer home: this is not a toy. Every major operating
system now accepts Rust code in its kernel. That is unprecedented for a
language that is barely 10 years old (1.0 shipped in May 2015).

---

## A quick history

- **2006**: Graydon Hoare starts Rust as a personal project
- **2010**: Mozilla sponsors the project
- **2015**: Rust 1.0 released &mdash; stability guarantee begins
- **2018, 2021, 2024**: *editions* let the language evolve without breaking old code
- **2021**: Rust Foundation founded (Google, Microsoft, Amazon, Meta, Mozilla)
- **Today**: 6-week release cadence, ~500 features in flight

Note:
"Editions" are a clever idea worth a minute. Every three years Rust cuts
an "edition" that can introduce small backward-incompatible syntax
changes, like reserving a new keyword. Crates declare which edition they
use in Cargo.toml, and the compiler reads code differently per edition.
All editions interoperate at the library level. This is how Rust gets
to evolve without an ecosystem-split-the-baby.

---

## What you will learn

**Today (Day 1):**

1. Installing Rust and using Cargo
2. Variables, types, functions, control flow
3. Pattern matching and enums
4. The big idea: **ownership & borrowing**
5. Structs and methods

**Day 2:**

6. Lifetimes, traits, generics
7. Error handling with `Result`
8. Iterators, closures, collections
9. Modules, testing, concurrency
10. A taste of async and macros

Note:
Day 1 is about getting to the point where you can read Rust code and
understand the ownership model. Day 2 is about the abstractions that
make Rust actually pleasant: iterators, traits, and error handling.

---

## Installing Rust

One tool installs everything: [**rustup**](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

That gives you:

- `rustc` &mdash; the compiler
- `cargo` &mdash; the build tool, package manager, and test runner
- `rustfmt` &mdash; the official formatter
- `clippy` &mdash; the official linter

> For today, you don't even need to install anything &mdash; we'll use the
> in-browser [Rust Playground](https://play.rust-lang.org/).

Note:
rustup manages toolchains the way nvm manages node versions. You can
have stable, beta, nightly, and arbitrary historical versions all
installed at once, and switch per-project.

---

## Hello, Rust

```rust []
fn main() {
    println!("Hello, CIS352!");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main()%20%7B%0A%20%20%20%20println!(%22Hello%2C%20CIS352!%22)%3B%0A%7D">&#9654; Run in Playground</a>

- `fn main()` &mdash; program entry point, same idea as C
- `println!` &mdash; a **macro** (the `!` tells you so), not a function
- Semicolons terminate statements
- No `return` needed &mdash; `main` returns `()`, the unit type

Note:
Three small things to point out on this slide. First: println! has that
bang because it is a macro. Rust uses macros for anything that takes a
variable number of arguments or needs to look at a literal string at
compile time. Second: those braces are mandatory, even for a one-line
function. Third: notice there is no include or import. The "prelude"
gives you println, Vec, Option, Result, and friends for free.

---

## Cargo: one tool for the whole workflow

```bash
$ cargo new hello
     Created binary (application) `hello` package

$ cd hello && tree
.
├── Cargo.toml         # manifest (like package.json)
├── src
│   └── main.rs        # entry point
└── .gitignore

$ cargo run
   Compiling hello v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
     Running `target/debug/hello`
Hello, world!
```

Cargo handles: build, run, test, doc, publish, format, lint.

Note:
Cargo is the reason Rust's ecosystem bootstrapped so fast. Compare to
C++: there is no standard build tool, no standard package manager, no
standard test runner. Rust shipped all three on day one and they work
together. `cargo new`, `cargo run`, `cargo test`, `cargo doc`,
`cargo fmt`, `cargo clippy` &mdash; all the same.
