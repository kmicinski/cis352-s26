<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 2 &bull; Chapter 6</span>

# Lifetimes

## Every borrow lives for a region. The compiler names it.

---

## A function that returns a reference

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20longest%28x%3A%20%26str%2C%20y%3A%20%26str%29%20-%3E%20%26str%20%7B%0A%20%20%20%20if%20x.len%28%29%20%3E%20y.len%28%29%20%7B%20x%20%7D%20else%20%7B%20y%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

The compiler rejects this. Why?

> The returned `&str` points into *one* of the inputs &mdash; but **which one**?
> Its lifetime must be tied to whichever outlives the other... or rather,
> whichever is *shorter*.

Note:
The compiler's logic: when the caller looks at the return value, it
needs to know how long it can use the reference. If `x` goes away but
`y` is still alive, is the returned reference still valid? The answer
depends on which branch was taken, which is a runtime choice. The
solution is to constrain the relationship with a *name*.

---

## Lifetime parameters

```rust []
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("longer sentence");
    let s2 = String::from("short");
    let best = longest(&s1, &s2);
    println!("{best}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20longest%3C%27a%3E%28x%3A%20%26%27a%20str%2C%20y%3A%20%26%27a%20str%29%20-%3E%20%26%27a%20str%20%7B%0A%20%20%20%20if%20x.len%28%29%20%3E%20y.len%28%29%20%7B%20x%20%7D%20else%20%7B%20y%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s1%20%3D%20String%3A%3Afrom%28%22longer%20sentence%22%29%3B%0A%20%20%20%20let%20s2%20%3D%20String%3A%3Afrom%28%22short%22%29%3B%0A%20%20%20%20let%20best%20%3D%20longest%28%26s1%2C%20%26s2%29%3B%0A%20%20%20%20println%21%28%22%7Bbest%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `'a` is a **lifetime parameter** &mdash; pronounced "tick a"
- Read it as: "there exists some region `'a` such that both inputs and
  the output are valid for `'a`"
- Lifetimes are part of the type, not the value; they vanish at runtime

Note:
This is the hardest part of Rust for beginners. The good news: you
rarely *write* lifetime annotations yourself. The compiler has elision
rules that handle the common cases. You write them when the compiler
can't figure it out, which usually means you're doing something
interesting.

---

## What breaks without lifetimes

```rust
fn dangling<'a>() -> &'a String {
    let s = String::from("oops");
    &s                // ERROR: `s` doesn't live long enough
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20dangling%3C%27a%3E%28%29%20-%3E%20%26%27a%20String%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22oops%22%29%3B%0A%20%20%20%20%26s%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20ERROR%3A%20%60s%60%20doesn%27t%20live%20long%20enough%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

The borrow checker refuses because `s` is dropped at function exit.

<div class="callout">
In C, this would compile and return a dangling pointer to freed stack
memory. In Rust, it is a compile error with a helpful diagnostic.
</div>

Note:
This is a cost saved at runtime: C programmers pay for this bug with
mysterious crashes and security vulnerabilities. Rust programmers pay
for it with a compile error they can fix immediately.

---

## Lifetime elision

You do not always have to write lifetimes. Three rules let the compiler
infer them:

1. Every input reference gets its own lifetime.
2. If there is exactly one input lifetime, it is assigned to all outputs.
3. If one of the inputs is `&self` or `&mut self`, its lifetime is assigned to all outputs.

```rust []
// All three of these compile without explicit lifetimes:
fn first_char(s: &str) -> &str { &s[..1] }
fn id(s: &str) -> &str { s }
impl Foo {
    fn name(&self) -> &str { &self.name }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=struct%20Foo%20%7B%20name%3A%20String%20%7D%0A%0A%2F%2F%20All%20three%20of%20these%20compile%20without%20explicit%20lifetimes%3A%0Afn%20first_char%28s%3A%20%26str%29%20-%3E%20%26str%20%7B%20%26s%5B..1%5D%20%7D%0Afn%20id%28s%3A%20%26str%29%20-%3E%20%26str%20%7B%20s%20%7D%0Aimpl%20Foo%20%7B%0A%20%20%20%20fn%20name%28%26self%29%20-%3E%20%26str%20%7B%20%26self.name%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Note:
These rules cover 95% of real-world code. You mostly see explicit
lifetimes in generic data structures, iterator adapters, and
self-referential APIs.

---

## The `'static` lifetime

```rust []
let s: &'static str = "I live for the whole program";
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20s%3A%20%26%27static%20str%20%3D%20%22I%20live%20for%20the%20whole%20program%22%3B%0A%7D%0A">&#9654; Run in Playground</a>

- String literals have type `&'static str`
- Global constants, `const` items, anything in `.rodata`
- `'static` means "valid for the entire program run"
- Often seen in error types or `Box<dyn Error + 'static>`

Note:
'static sounds scary but it's usually just "string literal." The other
place you see it is for types that don't borrow anything, like `String`
or `i32` &mdash; they technically satisfy any lifetime bound, including
'static.
