<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 1 &bull; Chapter 3</span>

# Enums &amp; Pattern Matching

## The most Racket-flavored part of Rust

---

## Enums: sum types with data

You already know this idea from Racket's tagged lists and structs.

```rust []
enum Shape {
    Circle(f64),                 // radius
    Rectangle { w: f64, h: f64 },// named fields
    Triangle(f64, f64, f64),     // three sides
    Point,                       // no data
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=enum%20Shape%20%7B%0A%20%20%20%20Circle%28f64%29%2C%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20radius%0A%20%20%20%20Rectangle%20%7B%20w%3A%20f64%2C%20h%3A%20f64%20%7D%2C%2F%2F%20named%20fields%0A%20%20%20%20Triangle%28f64%2C%20f64%2C%20f64%29%2C%20%20%20%20%20%2F%2F%20three%20sides%0A%20%20%20%20Point%2C%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20no%20data%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Each variant is a separate **case**; each can carry its own data.

Note:
Compare to Racket: you might write (define-type Shape [circle radius]
[rect w h] ...). Rust enums are the same idea with compile-time
guarantees that every case is handled. A rectangle and a circle are
both Shapes, and the compiler knows exactly how much memory each takes.

---

## Pattern matching

```rust []
fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { w, h } => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
        Shape::Point => 0.0,
    }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=enum%20Shape%20%7B%0A%20%20%20%20Circle%28f64%29%2C%0A%20%20%20%20Rectangle%20%7B%20w%3A%20f64%2C%20h%3A%20f64%20%7D%2C%0A%20%20%20%20Triangle%28f64%2C%20f64%2C%20f64%29%2C%0A%20%20%20%20Point%2C%0A%7D%0A%0Afn%20area%28s%3A%20%26Shape%29%20-%3E%20f64%20%7B%0A%20%20%20%20match%20s%20%7B%0A%20%20%20%20%20%20%20%20Shape%3A%3ACircle%28r%29%20%3D%3E%20std%3A%3Af64%3A%3Aconsts%3A%3API%20%2A%20r%20%2A%20r%2C%0A%20%20%20%20%20%20%20%20Shape%3A%3ARectangle%20%7B%20w%2C%20h%20%7D%20%3D%3E%20w%20%2A%20h%2C%0A%20%20%20%20%20%20%20%20Shape%3A%3ATriangle%28a%2C%20b%2C%20c%29%20%3D%3E%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20let%20s%20%3D%20%28a%20%2B%20b%20%2B%20c%29%20%2F%202.0%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%28s%20%2A%20%28s%20-%20a%29%20%2A%20%28s%20-%20b%29%20%2A%20%28s%20-%20c%29%29.sqrt%28%29%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%20%20%20%20Shape%3A%3APoint%20%3D%3E%200.0%2C%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

- `match` is an **expression**: every arm must yield the same type
- The compiler *refuses to compile* if a variant is missing
- This is exhaustiveness checking, done by the type system

Note:
Exhaustiveness is load-bearing. If you add a fourth shape six months
from now, every match that forgets to handle it becomes a compile
error, so the compiler walks you through the upgrade. This is the
single most beloved feature in Rust once you have lived with it.

---

## Patterns are structural

```rust []
fn describe(pair: (i32, i32)) -> &'static str {
    match pair {
        (0, 0) => "origin",
        (_, 0) => "on the x-axis",
        (0, _) => "on the y-axis",
        (x, y) if x == y => "on the diagonal",
        _ => "somewhere else",
    }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20describe(pair%3A%20(i32%2C%20i32))%20-%3E%20%26'static%20str%20%7B%0A%20%20%20%20match%20pair%20%7B%0A%20%20%20%20%20%20%20%20(0%2C%200)%20%3D%3E%20%22origin%22%2C%0A%20%20%20%20%20%20%20%20(_%2C%200)%20%3D%3E%20%22on%20the%20x-axis%22%2C%0A%20%20%20%20%20%20%20%20(0%2C%20_)%20%3D%3E%20%22on%20the%20y-axis%22%2C%0A%20%20%20%20%20%20%20%20(x%2C%20y)%20if%20x%20%3D%3D%20y%20%3D%3E%20%22on%20the%20diagonal%22%2C%0A%20%20%20%20%20%20%20%20_%20%3D%3E%20%22somewhere%20else%22%2C%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20for%20p%20in%20%5B(0%2C0)%2C%20(3%2C0)%2C%20(0%2C2)%2C%20(4%2C4)%2C%20(1%2C2)%5D%20%7B%0A%20%20%20%20%20%20%20%20println!(%22%7B%3A%3F%7D%20%3D%3E%20%7B%7D%22%2C%20p%2C%20describe(p))%3B%0A%20%20%20%20%7D%0A%7D">&#9654; Run in Playground</a>

- Literal patterns, wildcards (`_`), variable bindings, **guards** (`if ...`)
- Arms are tried **top to bottom**; the first match wins
- If you leave off `_ =>`, the compiler tells you exactly what's missing

Note:
Guards are the little `if` clauses on match arms. Be aware that
exhaustiveness checking is conservative: the compiler cannot reason
about guards, so if you use a guard you will probably need a default
arm at the end.

---

## `Option<T>`: there is no null

The nullability problem, solved by a library type:

```rust []
pub enum Option<T> {
    None,
    Some(T),
}
```

You use it every time a value might be missing.

```rust []
fn find_even(xs: &[i32]) -> Option<i32> {
    for x in xs {
        if x % 2 == 0 { return Some(*x); }
    }
    None
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20find_even%28xs%3A%20%26%5Bi32%5D%29%20-%3E%20Option%3Ci32%3E%20%7B%0A%20%20%20%20for%20x%20in%20xs%20%7B%0A%20%20%20%20%20%20%20%20if%20x%20%25%202%20%3D%3D%200%20%7B%20return%20Some%28%2Ax%29%3B%20%7D%0A%20%20%20%20%7D%0A%20%20%20%20None%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Note:
This is the same idea as Haskell's Maybe, or a Racket tagged union of
'nothing and (list 'just x). What makes it work in Rust is that
Option is not special &mdash; it is just an enum defined in the standard
library. You will write your own types that look exactly like this
when you want to express "maybe something, maybe nothing."

---

## Unwrapping an `Option`

```rust []
fn main() {
    let xs = [1, 3, 4, 7];
    match find_even(&xs) {
        Some(n) => println!("first even is {n}"),
        None    => println!("no evens"),
    }

    // Shortcut: only one pattern, use `if let`
    if let Some(n) = find_even(&xs) {
        println!("also {n}");
    }

    // Very short: panic if None (only when you're sure!)
    let n = find_even(&xs).unwrap();
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20find_even%28xs%3A%20%26%5Bi32%5D%29%20-%3E%20Option%3Ci32%3E%20%7B%0A%20%20%20%20for%20x%20in%20xs%20%7B%0A%20%20%20%20%20%20%20%20if%20x%20%25%202%20%3D%3D%200%20%7B%20return%20Some%28%2Ax%29%3B%20%7D%0A%20%20%20%20%7D%0A%20%20%20%20None%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20xs%20%3D%20%5B1%2C%203%2C%204%2C%207%5D%3B%0A%20%20%20%20match%20find_even%28%26xs%29%20%7B%0A%20%20%20%20%20%20%20%20Some%28n%29%20%3D%3E%20println%21%28%22first%20even%20is%20%7Bn%7D%22%29%2C%0A%20%20%20%20%20%20%20%20None%20%20%20%20%3D%3E%20println%21%28%22no%20evens%22%29%2C%0A%20%20%20%20%7D%0A%0A%20%20%20%20%2F%2F%20Shortcut%3A%20only%20one%20pattern%2C%20use%20%60if%20let%60%0A%20%20%20%20if%20let%20Some%28n%29%20%3D%20find_even%28%26xs%29%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22also%20%7Bn%7D%22%29%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20%2F%2F%20Very%20short%3A%20panic%20if%20None%20%28only%20when%20you%27re%20sure%21%29%0A%20%20%20%20let%20n%20%3D%20find_even%28%26xs%29.unwrap%28%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `match` &mdash; the safe, exhaustive default
- `if let` &mdash; match one pattern, ignore the rest
- `.unwrap()` &mdash; "I promise this is `Some`" or crash

Note:
.unwrap is the "YOLO" option. It is perfect for prototypes and tests
and always wrong in production code because it panics on None. As you
get more comfortable you will see people use ? or `expect("reason")`
instead.
