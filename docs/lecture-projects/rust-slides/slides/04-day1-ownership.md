<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 1 &bull; Chapter 4 &bull; The Big Idea</span>

# Ownership &amp; Borrowing

## How Rust eliminates a huge class of bugs without a GC

---

## The problem Rust is solving

How do you manage memory for a heap-allocated value?

- **C / C++**: you do it by hand. `malloc`/`free`, `new`/`delete`.
  - Bugs: use-after-free, double-free, leaks, dangling pointers, data races.
- **Java, C#, Python, Racket**: a **garbage collector** does it for you.
  - Cost: runtime overhead, pause times, no control over when memory is freed.
- **Rust**: the **compiler** tracks lifetimes at compile time.
  - Result: no GC, no manual free, and the bugs above become compile errors.

Note:
This is the 90-second elevator pitch for Rust. The story is: C lets
programmers do anything, which is fast but unsafe. Java stops
programmers from doing dangerous things by running a garbage collector,
which is safe but slow. Rust's insight is that a fancy type system can
prove most programs safe at compile time, so you pay the analysis cost
once, up front, instead of paying runtime cost forever.

---

## Three rules

Every value in Rust has one **owner**, determined statically.

1. **Every value has exactly one owner.**
2. When the owner goes out of scope, the value is **dropped** (freed).
3. Ownership can be **moved** &mdash; but only to one place.

```rust []
fn main() {
    let s = String::from("hello");   // s owns a heap-allocated string
    // ... use s ...
}                                    // s goes out of scope, memory freed
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%20%20%20%2F%2F%20s%20owns%20a%20heap-allocated%20string%0A%20%20%20%20%2F%2F%20...%20use%20s%20...%0A%7D%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20s%20goes%20out%20of%20scope%2C%20memory%20freed%0A">&#9654; Run in Playground</a>

No `free()`. No GC. The compiler inserts the `drop` automatically.

Note:
Rule one is the big one. In Java, lots of variables can point at the
same object and the GC sorts it out. In Rust, exactly one binding is
the "owner" and the rest are either moved from, borrowed, or cloned.
This is what allows the compiler to know exactly when a value is done
being used, so it can insert the free at compile time.

---

## Move semantics

```rust []
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;              // ownership MOVED from s1 to s2

    println!("{}", s2);       // ok
    // println!("{}", s1);    // compile error: value borrowed after move
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20s1%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20let%20s2%20%3D%20s1%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20ownership%20MOVED%20from%20s1%20to%20s2%0A%0A%20%20%20%20println%21%28%22%7B%7D%22%2C%20s2%29%3B%20%20%20%20%20%20%20%2F%2F%20ok%0A%20%20%20%20%2F%2F%20println%21%28%22%7B%7D%22%2C%20s1%29%3B%20%20%20%20%2F%2F%20compile%20error%3A%20value%20borrowed%20after%20move%0A%7D%0A">&#9654; Run in Playground</a>

The compiler catches the use of `s1` after it has been moved.

<div class="callout note">
This is a <strong>shallow</strong> transfer &mdash; the heap buffer is not copied.
<code>s1</code> is simply invalidated.
</div>

Note:
This is the first real Rust surprise. Assigning `s1` to `s2` doesn't
copy; it moves. Afterwards `s1` is not a dangling pointer &mdash; it is
statically known to be invalid. If you try to use it, the compiler
refuses. No runtime check, no crash, just a friendly error at build
time.

---

## Moves on function boundaries

```rust []
fn takes_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn main() {
    let greeting = String::from("hi");
    takes_ownership(greeting);
    // println!("{}", greeting);  // error: `greeting` has been moved
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20takes_ownership%28s%3A%20String%29%20%7B%0A%20%20%20%20println%21%28%22%7B%7D%22%2C%20s%29%3B%0A%7D%20%2F%2F%20s%20is%20dropped%20here%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20greeting%20%3D%20String%3A%3Afrom%28%22hi%22%29%3B%0A%20%20%20%20takes_ownership%28greeting%29%3B%0A%20%20%20%20%2F%2F%20println%21%28%22%7B%7D%22%2C%20greeting%29%3B%20%20%2F%2F%20error%3A%20%60greeting%60%20has%20been%20moved%0A%7D%0A">&#9654; Run in Playground</a>

Passing a `String` into a function **moves** it.

If you want to keep using the value, either:

- **return it** from the function, or
- **borrow** it (next slide), or
- **clone** it (slow, explicit)

Note:
The "return it" pattern gets old fast, which motivates borrowing. You
basically never see code that takes ownership and then gives it back
&mdash; instead you see code that borrows the value for as long as it
needs, which we'll see next.

---

## Copy types are different

Small, stack-only types implement the `Copy` trait:

```rust []
fn main() {
    let a: i32 = 5;
    let b = a;                // COPIED, not moved
    println!("a = {a}, b = {b}");   // both still work
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20a%3A%20i32%20%3D%205%3B%0A%20%20%20%20let%20b%20%3D%20a%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20COPIED%2C%20not%20moved%0A%20%20%20%20println%21%28%22a%20%3D%20%7Ba%7D%2C%20b%20%3D%20%7Bb%7D%22%29%3B%20%20%20%2F%2F%20both%20still%20work%0A%7D%0A">&#9654; Run in Playground</a>

- `i32`, `f64`, `bool`, `char`, `(i32, i32)` &mdash; all `Copy`
- `String`, `Vec<T>`, `Box<T>` &mdash; **not** `Copy` (they own heap memory)
- The rule: if copying is "just memcpy of the bytes," a type is `Copy`

Note:
This is why our earlier slides with `let x = 5; let y = x;` worked
fine. Copy types give you the "natural" semantics. Move semantics
only kick in for types that own something on the heap, because those
are the ones where sharing a pointer would be ambiguous.

---

## Borrowing: references that don't take ownership

Instead of moving, you can lend a reference:

```rust []
fn word_count(s: &String) -> usize {   // borrows s
    s.split_whitespace().count()
}

fn main() {
    let text = String::from("Rust is fun");
    let n = word_count(&text);           // lend it
    println!("{text} has {n} words");    // still ours!
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20word_count%28s%3A%20%26String%29%20-%3E%20usize%20%7B%20%20%20%2F%2F%20borrows%20s%0A%20%20%20%20s.split_whitespace%28%29.count%28%29%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20text%20%3D%20String%3A%3Afrom%28%22Rust%20is%20fun%22%29%3B%0A%20%20%20%20let%20n%20%3D%20word_count%28%26text%29%3B%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20lend%20it%0A%20%20%20%20println%21%28%22%7Btext%7D%20has%20%7Bn%7D%20words%22%29%3B%20%20%20%20%2F%2F%20still%20ours%21%0A%7D%0A">&#9654; Run in Playground</a>

- `&text` creates an **immutable reference** (aka a borrow)
- `&String` in the parameter says "I just want to read it"
- The borrow ends when the function returns &mdash; the owner keeps going

Note:
Notice how much more natural this reads than "move in, do work, move
out." Borrowing is the default way you pass data around in Rust. In
practice you will almost always take `&str` instead of `&String` so
string literals also work &mdash; we'll get there.

---

## Mutable borrows

```rust []
fn push_bang(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut greeting = String::from("hello");
    push_bang(&mut greeting);
    println!("{greeting}");   // hello!
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20push_bang%28s%3A%20%26mut%20String%29%20%7B%0A%20%20%20%20s.push%28%27%21%27%29%3B%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20greeting%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%0A%20%20%20%20push_bang%28%26mut%20greeting%29%3B%0A%20%20%20%20println%21%28%22%7Bgreeting%7D%22%29%3B%20%20%20%2F%2F%20hello%21%0A%7D%0A">&#9654; Run in Playground</a>

- `&mut T` is an **exclusive** reference
- You can read *and* write through it
- The owner must itself be declared `mut` to lend a `&mut`

Note:
The difference between `&T` and `&mut T` is the difference between
"readers" and "writers" in classic concurrency theory. Rust uses the
same idea statically: many readers or one writer, never both.

---

## The borrow checker's two commandments

At any point in the program, for any given value:

- you can have **any number** of immutable references `&T`, **OR**
- you can have **exactly one** mutable reference `&mut T`

But never both at the same time.

```rust []
let mut s = String::from("hi");
let r1 = &s;
let r2 = &s;
let r3 = &mut s;   // ERROR: cannot borrow as mutable
println!("{r1} {r2} {r3}");
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20s%20%3D%20String%3A%3Afrom%28%22hi%22%29%3B%0A%20%20%20%20let%20r1%20%3D%20%26s%3B%0A%20%20%20%20let%20r2%20%3D%20%26s%3B%0A%20%20%20%20let%20r3%20%3D%20%26mut%20s%3B%20%20%20%2F%2F%20ERROR%3A%20cannot%20borrow%20as%20mutable%0A%20%20%20%20println%21%28%22%7Br1%7D%20%7Br2%7D%20%7Br3%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

Note:
This is THE rule. It's what eliminates data races at compile time.
If no one else can see the value while you're mutating it, there is
nothing to race against. This rule is also why iterator invalidation
bugs (modifying a collection while iterating over it) are impossible
in safe Rust: the iterator holds a borrow, so you can't also grab a
mutable borrow to modify.

---

## A real-world borrow error

```rust []
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];            // immutable borrow of v
    v.push(4);                    // mutable borrow &mdash; ERROR
    println!("{first}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20mut%20v%20%3D%20vec%21%5B1%2C%202%2C%203%5D%3B%0A%20%20%20%20let%20first%20%3D%20%26v%5B0%5D%3B%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20immutable%20borrow%20of%20v%0A%20%20%20%20v.push%284%29%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20mutable%20borrow%20%26mdash%3B%20ERROR%0A%20%20%20%20println%21%28%22%7Bfirst%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

```
error[E0502]: cannot borrow `v` as mutable because it is
              also borrowed as immutable
  |
4 |     let first = &v[0];
  |                  -  immutable borrow occurs here
5 |     v.push(4);
  |     ^^^^^^^^^ mutable borrow occurs here
6 |     println!("{first}");
  |                ----- immutable borrow later used here
```

Why is this *actually* dangerous in C++? `push` might reallocate the
backing buffer, leaving `first` dangling.

Note:
This is the example that makes it click. In C++ this exact code is a
use-after-free waiting to happen: if push reallocates, first points to
freed memory. Rust sees that you're holding a reference into v at the
same time you're asking to mutate v, and says no. The error message is
unusually friendly: it points at both borrows and explains the
conflict. Rust's error messages are one of its best features.

---

## Slices: borrows into sequences

```rust []
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[..i]; }
    }
    &s[..]
}

fn main() {
    let sentence = String::from("hello CIS352");
    let w = first_word(&sentence);
    println!("first word: {w}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20first_word%28s%3A%20%26str%29%20-%3E%20%26str%20%7B%0A%20%20%20%20let%20bytes%20%3D%20s.as_bytes%28%29%3B%0A%20%20%20%20for%20%28i%2C%20%26b%29%20in%20bytes.iter%28%29.enumerate%28%29%20%7B%0A%20%20%20%20%20%20%20%20if%20b%20%3D%3D%20b%27%20%27%20%7B%20return%20%26s%5B..i%5D%3B%20%7D%0A%20%20%20%20%7D%0A%20%20%20%20%26s%5B..%5D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20sentence%20%3D%20String%3A%3Afrom%28%22hello%20CIS352%22%29%3B%0A%20%20%20%20let%20w%20%3D%20first_word%28%26sentence%29%3B%0A%20%20%20%20println%21%28%22first%20word%3A%20%7Bw%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `&s[..i]` is a **string slice**: a `&str` borrowed from `s`
- `&[T]` is the same idea over any array or `Vec<T>`
- Slices carry a pointer and a length &mdash; no ownership, no copy

Note:
Slices are how you write generic "give me a view of this sequence"
code. The reason we take `&str` instead of `&String` is that `&str`
can point into a `String`, a string literal, a slice of a Vec<u8>, etc.
It's the most general reader. Same thing for `&[T]` instead of `&Vec<T>`.

---

## Recap: the ownership model in one slide

- **Ownership** &mdash; each value has exactly one owner; dropped on scope exit.
- **Move** &mdash; ownership transfers; the source is invalidated statically.
- **Copy** &mdash; small stack-only types are duplicated instead of moved.
- **Borrow** &mdash; references let functions *read* or *write* without owning.
- **Borrow checker** &mdash; many readers OR one writer, never both.
- **Slices** &mdash; views into sequences; the bread-and-butter of Rust APIs.

<div class="callout good">
Once you internalize this, <strong>Rust feels simple.</strong> Almost
every compiler error you hit is one of these rules being enforced.
</div>

Note:
If students walk out of Day 1 with nothing else, they need this slide.
Everything in Day 2 &mdash; lifetimes, iterators, concurrency, async &mdash;
builds on these rules. Lifetimes, for example, are just "how long does
this borrow live?" The traits we see tomorrow include Send and Sync,
which are defined entirely in terms of what kinds of borrows can cross
thread boundaries.
