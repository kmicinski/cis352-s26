<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 2 &bull; Chapter 7</span>

# Traits &amp; Generics

## Interfaces, done right

---

## A trait is a set of behaviors

```rust []
trait Greet {
    fn hello(&self) -> String;
    // default method, can be overridden
    fn shout(&self) -> String {
        self.hello().to_uppercase()
    }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=trait%20Greet%20%7B%0A%20%20%20%20fn%20hello%28%26self%29%20-%3E%20String%3B%0A%20%20%20%20%2F%2F%20default%20method%2C%20can%20be%20overridden%0A%20%20%20%20fn%20shout%28%26self%29%20-%3E%20String%20%7B%0A%20%20%20%20%20%20%20%20self.hello%28%29.to_uppercase%28%29%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Haskell calls this a *type class*. Java calls it an *interface*. Go calls it an *interface*. Scala calls it a *trait*.

Note:
Traits are Rust's single biggest abstraction mechanism. Almost every
piece of generic code you write will involve a trait. The key
difference from Java interfaces is that you can implement a trait for
a type *after the fact*, even a type someone else defined.

---

## Implementing a trait

```rust []
struct Dog { name: String }
struct Robot { id: u32 }

impl Greet for Dog {
    fn hello(&self) -> String { format!("woof, I'm {}", self.name) }
}

impl Greet for Robot {
    fn hello(&self) -> String { format!("BEEP UNIT {}", self.id) }
    fn shout(&self) -> String { "[OVERLOADED]".into() }
}

fn main() {
    let d = Dog { name: "Rex".into() };
    let r = Robot { id: 42 };
    println!("{}", d.hello());
    println!("{}", r.shout());
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=trait%20Greet%20%7B%0A%20%20%20%20fn%20hello%28%26self%29%20-%3E%20String%3B%0A%20%20%20%20fn%20shout%28%26self%29%20-%3E%20String%20%7B%20self.hello%28%29.to_uppercase%28%29%20%7D%0A%7D%0A%0Astruct%20Dog%20%7B%20name%3A%20String%20%7D%0Astruct%20Robot%20%7B%20id%3A%20u32%20%7D%0A%0Aimpl%20Greet%20for%20Dog%20%7B%0A%20%20%20%20fn%20hello%28%26self%29%20-%3E%20String%20%7B%20format%21%28%22woof%2C%20I%27m%20%7B%7D%22%2C%20self.name%29%20%7D%0A%7D%0A%0Aimpl%20Greet%20for%20Robot%20%7B%0A%20%20%20%20fn%20hello%28%26self%29%20-%3E%20String%20%7B%20format%21%28%22BEEP%20UNIT%20%7B%7D%22%2C%20self.id%29%20%7D%0A%20%20%20%20fn%20shout%28%26self%29%20-%3E%20String%20%7B%20%22%5BOVERLOADED%5D%22.into%28%29%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20d%20%3D%20Dog%20%7B%20name%3A%20%22Rex%22.into%28%29%20%7D%3B%0A%20%20%20%20let%20r%20%3D%20Robot%20%7B%20id%3A%2042%20%7D%3B%0A%20%20%20%20println%21%28%22%7B%7D%22%2C%20d.hello%28%29%29%3B%0A%20%20%20%20println%21%28%22%7B%7D%22%2C%20r.shout%28%29%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

Note:
The pattern `format!` is the string-returning cousin of `println!`.
Both are macros that accept Python-style format arguments. `.into()` is
a handy conversion &mdash; here it turns `&str` into `String`.

---

## Generics and trait bounds

```rust []
fn announce<T: Greet>(thing: &T) {
    println!("Announcing: {}", thing.hello());
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=trait%20Greet%20%7B%20fn%20hello%28%26self%29%20-%3E%20String%3B%20%7D%0A%0Afn%20announce%3CT%3A%20Greet%3E%28thing%3A%20%26T%29%20%7B%0A%20%20%20%20println%21%28%22Announcing%3A%20%7B%7D%22%2C%20thing.hello%28%29%29%3B%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Read: "for any type `T` that implements `Greet`, here's a function."

Equivalent, more common in real code:

```rust []
fn announce(thing: &impl Greet) {
    println!("Announcing: {}", thing.hello());
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=trait%20Greet%20%7B%20fn%20hello%28%26self%29%20-%3E%20String%3B%20%7D%0A%0Afn%20announce%28thing%3A%20%26impl%20Greet%29%20%7B%0A%20%20%20%20println%21%28%22Announcing%3A%20%7B%7D%22%2C%20thing.hello%28%29%29%3B%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Or, for multiple bounds:

```rust []
fn print_it<T: Greet + std::fmt::Debug>(thing: &T) { /* ... */ }
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=trait%20Greet%20%7B%20fn%20hello%28%26self%29%20-%3E%20String%3B%20%7D%0A%0Afn%20print_it%3CT%3A%20Greet%20%2B%20std%3A%3Afmt%3A%3ADebug%3E%28thing%3A%20%26T%29%20%7B%20%2F%2A%20...%20%2A%2F%20%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Note:
Generics in Rust are *monomorphized*: the compiler generates a separate
version of the function for each concrete type you call it with. This
means zero runtime cost &mdash; as if you'd written each version by hand.
The downside is code size. The trade-off is almost always worth it.

---

## Dynamic dispatch with `dyn Trait`

Monomorphization requires the type at compile time. Sometimes you don't have it:

```rust []
fn announce_all(things: &[Box<dyn Greet>]) {
    for t in things {
        println!("{}", t.hello());
    }
}

fn main() {
    let crew: Vec<Box<dyn Greet>> = vec![
        Box::new(Dog { name: "Rex".into() }),
        Box::new(Robot { id: 7 }),
    ];
    announce_all(&crew);
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=trait%20Greet%20%7B%20fn%20hello%28%26self%29%20-%3E%20String%3B%20%7D%0A%0Astruct%20Dog%20%7B%20name%3A%20String%20%7D%0Astruct%20Robot%20%7B%20id%3A%20u32%20%7D%0A%0Aimpl%20Greet%20for%20Dog%20%7B%20fn%20hello%28%26self%29%20-%3E%20String%20%7B%20format%21%28%22woof%2C%20%7B%7D%22%2C%20self.name%29%20%7D%20%7D%0Aimpl%20Greet%20for%20Robot%20%7B%20fn%20hello%28%26self%29%20-%3E%20String%20%7B%20format%21%28%22BEEP%20%7B%7D%22%2C%20self.id%29%20%7D%20%7D%0A%0Afn%20announce_all%28things%3A%20%26%5BBox%3Cdyn%20Greet%3E%5D%29%20%7B%0A%20%20%20%20for%20t%20in%20things%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22%7B%7D%22%2C%20t.hello%28%29%29%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20crew%3A%20Vec%3CBox%3Cdyn%20Greet%3E%3E%20%3D%20vec%21%5B%0A%20%20%20%20%20%20%20%20Box%3A%3Anew%28Dog%20%7B%20name%3A%20%22Rex%22.into%28%29%20%7D%29%2C%0A%20%20%20%20%20%20%20%20Box%3A%3Anew%28Robot%20%7B%20id%3A%207%20%7D%29%2C%0A%20%20%20%20%5D%3B%0A%20%20%20%20announce_all%28%26crew%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `dyn Greet` is a **trait object** &mdash; like a Java interface type
- Dispatch happens at runtime through a vtable
- Slightly slower than generics, but more flexible

Note:
The split between `impl Trait` (static) and `dyn Trait` (dynamic) is a
choice you get to make per-call-site. Static is faster but monomorphizes;
dynamic is slightly slower but lets you store heterogeneous things in one
collection. Start with static and reach for dynamic when you really need
it.

---

## `derive` automates common traits

```rust []
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    id: u64,
    name: String,
}

fn main() {
    let u = User { id: 1, name: "Kris".into() };
    let v = u.clone();
    println!("{u:?}");
    assert_eq!(u, v);
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%23%5Bderive%28Debug%2C%20Clone%2C%20PartialEq%2C%20Eq%2C%20Hash%29%5D%0Astruct%20User%20%7B%0A%20%20%20%20id%3A%20u64%2C%0A%20%20%20%20name%3A%20String%2C%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20u%20%3D%20User%20%7B%20id%3A%201%2C%20name%3A%20%22Kris%22.into%28%29%20%7D%3B%0A%20%20%20%20let%20v%20%3D%20u.clone%28%29%3B%0A%20%20%20%20println%21%28%22%7Bu%3A%3F%7D%22%29%3B%0A%20%20%20%20assert_eq%21%28u%2C%20v%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

Traits commonly derived:

| Trait | What it gives you |
|---|---|
| `Debug` | `{:?}` formatting |
| `Clone`, `Copy` | `.clone()`, copy semantics |
| `PartialEq`, `Eq` | `==`, `!=`, hashability |
| `Hash` | key in `HashMap`, `HashSet` |
| `PartialOrd`, `Ord` | `<`, `>`, sorting |
| `Default` | `T::default()` |
| `Serialize`, `Deserialize` | JSON / TOML / bincode / ... |

Note:
Serialize/Deserialize come from `serde`, the de facto serialization
crate. It is downloaded hundreds of millions of times a month. In Rust
culture, "we'll use serde" is the default answer for any data format.

---

## Generic data structures

```rust []
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { items: Vec::new() } }
    fn push(&mut self, x: T) { self.items.push(x); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
    fn peek(&self) -> Option<&T> { self.items.last() }
    fn len(&self) -> usize { self.items.len() }
}

fn main() {
    let mut s: Stack<i32> = Stack::new();
    s.push(1); s.push(2); s.push(3);
    while let Some(x) = s.pop() { println!("{x}"); }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=struct%20Stack%3CT%3E%20%7B%20items%3A%20Vec%3CT%3E%20%7D%0A%0Aimpl%3CT%3E%20Stack%3CT%3E%20%7B%0A%20%20%20%20fn%20new()%20-%3E%20Self%20%7B%20Stack%20%7B%20items%3A%20Vec%3A%3Anew()%20%7D%20%7D%0A%20%20%20%20fn%20push(%26mut%20self%2C%20x%3A%20T)%20%7B%20self.items.push(x)%3B%20%7D%0A%20%20%20%20fn%20pop(%26mut%20self)%20-%3E%20Option%3CT%3E%20%7B%20self.items.pop()%20%7D%0A%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20mut%20s%3A%20Stack%3Ci32%3E%20%3D%20Stack%3A%3Anew()%3B%0A%20%20%20%20s.push(1)%3B%20s.push(2)%3B%20s.push(3)%3B%0A%20%20%20%20while%20let%20Some(x)%20%3D%20s.pop()%20%7B%20println!(%22%7Bx%7D%22)%3B%20%7D%0A%7D">&#9654; Run in Playground</a>

Note:
Monomorphization again: the compiler generates a `Stack<i32>` type,
a `Stack<String>` type, etc. Each version is a concrete struct with
concrete methods. This is why you pay no runtime cost for generics
in Rust.
