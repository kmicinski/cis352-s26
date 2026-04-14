<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 2 &bull; Chapter 12</span>

# Odds, Ends &amp; Where to Go Next

---

## Macros: a one-slide peek

Two kinds of macros in Rust:

**Declarative** (`macro_rules!`) &mdash; pattern match on token trees:

```rust []
macro_rules! vec_of_strings {
    ($($x:expr),* $(,)?) => {
        vec![$($x.to_string()),*]
    };
}

fn main() {
    let v = vec_of_strings!["apple", "banana", "cherry"];
    println!("{v:?}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=macro_rules%21%20vec_of_strings%20%7B%0A%20%20%20%20%28%24%28%24x%3Aexpr%29%2C%2A%20%24%28%2C%29%3F%29%20%3D%3E%20%7B%0A%20%20%20%20%20%20%20%20vec%21%5B%24%28%24x.to_string%28%29%29%2C%2A%5D%0A%20%20%20%20%7D%3B%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20v%20%3D%20vec_of_strings%21%5B%22apple%22%2C%20%22banana%22%2C%20%22cherry%22%5D%3B%0A%20%20%20%20println%21%28%22%7Bv%3A%3F%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

**Procedural** &mdash; compiler plugins written in Rust. `#[derive(Serialize)]`, `#[tokio::main]`, `html!` &mdash; all procedural macros.

Note:
Macros are how Rust handles anything that needs to look at syntax or
generate a lot of boilerplate. The famous ones &mdash; println!, vec!,
format!, assert_eq! &mdash; are all declarative. Proc macros are more
powerful but run actual Rust code at compile time.

---

## `unsafe`: the escape hatch

```rust []
fn main() {
    let mut v = vec![1, 2, 3, 4];

    unsafe {
        // SAFETY: index is in bounds
        let x = v.get_unchecked(0);
        println!("{x}");
    }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20v%20%3D%20vec%21%5B1%2C%202%2C%203%2C%204%5D%3B%0A%0A%20%20%20%20unsafe%20%7B%0A%20%20%20%20%20%20%20%20%2F%2F%20SAFETY%3A%20index%20is%20in%20bounds%0A%20%20%20%20%20%20%20%20let%20x%20%3D%20v.get_unchecked%280%29%3B%0A%20%20%20%20%20%20%20%20println%21%28%22%7Bx%7D%22%29%3B%0A%20%20%20%20%7D%0A%7D%0A">&#9654; Run in Playground</a>

- `unsafe` unlocks **five** superpowers (raw pointers, FFI, mutable
  statics, unions, unsafe traits)
- It does **not** disable the borrow checker
- You're promising the compiler that *you* verified the invariants
- Convention: every `unsafe` block has a `// SAFETY:` comment

<div class="callout">
<strong>Rule of thumb:</strong> avoid <code>unsafe</code> in application
code. Most people never need it. When you do, keep it small and wrap it
in a safe API.
</div>

Note:
The existence of unsafe is what lets Rust implement things like Vec,
HashMap, and Mutex in the first place &mdash; their internals use raw
pointers. But that unsafety is encapsulated: the public API is safe.
As an application programmer, you almost never write `unsafe` yourself.

---

## What we didn't cover

A quick inventory of things worth knowing exist:

- **Smart pointers**: `Box`, `Rc`, `Arc`, `RefCell`, `Cow`
- **Interior mutability**: `Cell`, `RefCell`, atomics
- **FFI**: calling C from Rust and vice versa
- **`no_std`**: Rust on microcontrollers, in kernels, in WebAssembly
- **Embedded**: `embedded-hal`, RTIC
- **WebAssembly**: `wasm-bindgen`, `wasm-pack` (your lambda playground uses these!)
- **Build tooling**: `build.rs`, workspaces, features, conditional compilation
- **The trait ecosystem**: `From`/`Into`, `AsRef`, `Borrow`, `Deref`, `Drop`
- **Pinning, async internals**, **GATs**, **const generics**

Note:
Every single item on this list has a whole lecture's worth of depth.
The Rust ecosystem is deep. The good news is the parts you need most
are the parts we've already covered.

---

## Learning resources

**Free & official:**

- [**The Rust Book**](https://doc.rust-lang.org/book/) &mdash; the canonical tutorial, reads like a textbook
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/) &mdash; learn by reading short, annotated programs
- [**Rustlings**](https://github.com/rust-lang/rustlings) &mdash; small exercises, compiler-driven, highly recommended
- [**The Rustonomicon**](https://doc.rust-lang.org/nomicon/) &mdash; the dark arts of `unsafe`

**Interactive:**

- [**Rust Playground**](https://play.rust-lang.org/) &mdash; what we used all lecture
- [**Exercism Rust track**](https://exercism.org/tracks/rust) &mdash; puzzle-based

**Go deeper:**

- [**Jon Gjengset's YouTube channel**](https://www.youtube.com/@jonhoo) &mdash; advanced Rust, multi-hour videos
- *Programming Rust* by Blandy, Orendorff, Tindall (O'Reilly)

Note:
If you only pick one, pick Rustlings. It's a sequence of small broken
programs and your job is to make them compile. The compiler errors are
so good that Rustlings is basically a tutorial delivered via compile
errors.

---

<!-- .slide: class="big-point" -->

# Thanks!

## Questions?

<br/>

*The compiler is your friend, not your enemy.*

<a class="playground" target="_blank" href="https://play.rust-lang.org/">&#9654; Open the Playground</a>

Note:
Final thought for students: when the borrow checker says no, it's
almost always right. The cost you pay up front &mdash; fighting the
compiler &mdash; buys you something valuable: when your program compiles,
there's a huge class of bugs it literally cannot have. After a few
weeks of writing Rust, the compiler errors stop feeling adversarial
and start feeling like a reviewer catching things before your coworkers
see them.
