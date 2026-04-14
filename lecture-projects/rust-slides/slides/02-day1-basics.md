<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 1 &bull; Chapter 2</span>

# Language Basics

## Variables, types, functions, control flow

---

## Variables are immutable by default

```rust []
fn main() {
    let x = 5;
    // x = 6;  // compile error: cannot assign twice to immutable variable
    let mut y = 5;
    y = 6;     // ok
    println!("x = {x}, y = {y}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main()%20%7B%0A%20%20%20%20let%20x%20%3D%205%3B%0A%20%20%20%20let%20mut%20y%20%3D%205%3B%0A%20%20%20%20y%20%3D%206%3B%0A%20%20%20%20println!(%22x%20%3D%20%7Bx%7D%2C%20y%20%3D%20%7By%7D%22)%3B%0A%7D">&#9654; Run in Playground</a>

- `let` introduces a binding &mdash; immutable unless you write `mut`
- The default is a nudge: most variables really are write-once
- Notice `{x}` &mdash; format strings can name locals directly

Note:
The default-immutable thing is philosophical. It is the same reason
functional programmers like Racket: reasoning about code is easier when
most things cannot change. But Rust is pragmatic: when you really do
need mutation, one keyword gets you there.

---

## Shadowing vs mutation

```rust []
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();   // rebind, new type!
    println!("{spaces}");        // prints 3

    // let mut spaces = "   ";
    // spaces = spaces.len();    // error: mismatched types
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20spaces%20%3D%20%22%20%20%20%22%3B%0A%20%20%20%20let%20spaces%20%3D%20spaces.len%28%29%3B%20%20%20%2F%2F%20rebind%2C%20new%20type%21%0A%20%20%20%20println%21%28%22%7Bspaces%7D%22%29%3B%20%20%20%20%20%20%20%20%2F%2F%20prints%203%0A%0A%20%20%20%20%2F%2F%20let%20mut%20spaces%20%3D%20%22%20%20%20%22%3B%0A%20%20%20%20%2F%2F%20spaces%20%3D%20spaces.len%28%29%3B%20%20%20%20%2F%2F%20error%3A%20mismatched%20types%0A%7D%0A">&#9654; Run in Playground</a>

- `let x = ...` twice creates a *new* variable that **shadows** the old one
- Shadowing can change type; `mut` cannot
- Handy for parsing pipelines: `let x = parse(x);`

Note:
Shadowing is a little surprising the first time. It is not the same as
rebinding. The old x still exists lower in the stack; the compiler
just routes new uses of the name to the new binding. Most importantly,
shadowing is allowed to change the type, so it is great for the common
"string to int to range" pipeline.

---

## Scalar types at a glance

| Kind | Examples | Notes |
|---|---|---|
| Signed integers | `i8 i16 i32 i64 i128 isize` | `i32` is the default |
| Unsigned | `u8 u16 u32 u64 u128 usize` | `usize` indexes memory |
| Floats | `f32 f64` | `f64` default |
| Boolean | `bool` | `true` / `false`, 1 byte |
| Character | `char` | 4 bytes, full Unicode scalar |

```rust []
let a: i32 = 1_000_000;   // underscores are a readability aid
let b = 3.14_f32;         // suffix picks a type
let c: char = '🦀';        // four bytes, not one
let d = 0xff_u8;          // hex literal, explicit type
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20a%3A%20i32%20%3D%201_000_000%3B%20%20%20%2F%2F%20underscores%20are%20a%20readability%20aid%0A%20%20%20%20let%20b%20%3D%203.14_f32%3B%20%20%20%20%20%20%20%20%20%2F%2F%20suffix%20picks%20a%20type%0A%20%20%20%20let%20c%3A%20char%20%3D%20%27%F0%9F%A6%80%27%3B%20%20%20%20%20%20%20%20%2F%2F%20four%20bytes%2C%20not%20one%0A%20%20%20%20let%20d%20%3D%200xff_u8%3B%20%20%20%20%20%20%20%20%20%20%2F%2F%20hex%20literal%2C%20explicit%20type%0A%7D%0A">&#9654; Run in Playground</a>

Note:
Two things worth pointing out. First, integer overflow panics in debug
builds and wraps in release builds by default. You can opt in to
checked arithmetic explicitly with methods like `checked_add` or
`wrapping_add`. Second, char is a Unicode scalar, not a byte. A byte
is a u8. This matters a lot once you get to strings.

---

## Compound types: tuples and arrays

```rust []
fn main() {
    let point: (f64, f64, &str) = (1.0, 2.0, "origin-ish");
    let (x, y, label) = point;          // destructuring
    println!("{label}: ({x}, {y})");

    let primes: [i32; 5] = [2, 3, 5, 7, 11];
    println!("first prime: {}", primes[0]);
    println!("length: {}", primes.len());
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20point%3A%20%28f64%2C%20f64%2C%20%26str%29%20%3D%20%281.0%2C%202.0%2C%20%22origin-ish%22%29%3B%0A%20%20%20%20let%20%28x%2C%20y%2C%20label%29%20%3D%20point%3B%20%20%20%20%20%20%20%20%20%20%2F%2F%20destructuring%0A%20%20%20%20println%21%28%22%7Blabel%7D%3A%20%28%7Bx%7D%2C%20%7By%7D%29%22%29%3B%0A%0A%20%20%20%20let%20primes%3A%20%5Bi32%3B%205%5D%20%3D%20%5B2%2C%203%2C%205%2C%207%2C%2011%5D%3B%0A%20%20%20%20println%21%28%22first%20prime%3A%20%7B%7D%22%2C%20primes%5B0%5D%29%3B%0A%20%20%20%20println%21%28%22length%3A%20%7B%7D%22%2C%20primes.len%28%29%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- **Tuples** group fixed-size heterogeneous values
- **Arrays** are fixed-size, stack-allocated, *same type*
- Length is part of the type: `[i32; 5]` is a different type from `[i32; 6]`
- For a growable array, you want `Vec<T>` (coming later)

Note:
The fact that array length is part of the type often surprises people
coming from dynamic languages. It is what lets Rust check bounds at
compile time for constant indices.

---

## Strings: there are two kinds

```rust []
fn main() {
    let literal: &str = "hello";              // borrowed, static
    let owned: String = String::from("hello");// owned, heap-allocated
    let also_owned: String = "hello".to_string();

    println!("{} has {} bytes", literal, literal.len());
    println!("{} has {} bytes", owned, owned.len());
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20literal%3A%20%26str%20%3D%20%22hello%22%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20borrowed%2C%20static%0A%20%20%20%20let%20owned%3A%20String%20%3D%20String%3A%3Afrom%28%22hello%22%29%3B%2F%2F%20owned%2C%20heap-allocated%0A%20%20%20%20let%20also_owned%3A%20String%20%3D%20%22hello%22.to_string%28%29%3B%0A%0A%20%20%20%20println%21%28%22%7B%7D%20has%20%7B%7D%20bytes%22%2C%20literal%2C%20literal.len%28%29%29%3B%0A%20%20%20%20println%21%28%22%7B%7D%20has%20%7B%7D%20bytes%22%2C%20owned%2C%20owned.len%28%29%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- **`&str`** &mdash; a borrowed view of UTF-8 bytes somewhere
- **`String`** &mdash; a growable, heap-allocated, owned buffer
- Rule of thumb: **take `&str` as a parameter, store `String`**

Note:
This two-string thing trips up every Rust beginner. The short story:
&str is like a C `char*` with a length attached: it points at someone
else's memory. String is like a C++ `std::string`: it owns its bytes.
You can always get an &str from a String by borrowing, which is why
most functions take &str.

---

## Functions

```rust []
fn add(a: i32, b: i32) -> i32 {
    a + b                    // last expression is the return value
}

fn greet(name: &str) {       // no return arrow == returns ()
    println!("Hello, {name}!");
}

fn main() {
    let sum = add(2, 3);
    greet("CIS352");
    println!("2 + 3 = {sum}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20add%28a%3A%20i32%2C%20b%3A%20i32%29%20-%3E%20i32%20%7B%0A%20%20%20%20a%20%2B%20b%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20last%20expression%20is%20the%20return%20value%0A%7D%0A%0Afn%20greet%28name%3A%20%26str%29%20%7B%20%20%20%20%20%20%20%2F%2F%20no%20return%20arrow%20%3D%3D%20returns%20%28%29%0A%20%20%20%20println%21%28%22Hello%2C%20%7Bname%7D%21%22%29%3B%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20sum%20%3D%20add%282%2C%203%29%3B%0A%20%20%20%20greet%28%22CIS352%22%29%3B%0A%20%20%20%20println%21%28%222%20%2B%203%20%3D%20%7Bsum%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- Parameters **must** be annotated with types
- Return type after `->`; omit for `()`
- Final expression (no semicolon!) is the return value

Note:
The semicolon matters. `a + b` is an expression that evaluates to the
sum. `a + b;` is a statement that evaluates to (). If you add a trailing
semicolon to that last line, you will get a type error saying the
function returns () but i32 was expected. This is one of the first
mistakes you will make, and it will teach you the Rust mantra:
"expressions return values, statements do not."

---

## Expressions vs statements

Everything that can produce a value is an **expression**.

```rust []
fn main() {
    let y = {
        let x = 3;
        x + 1          // no semicolon == block's value
    };
    println!("y = {y}");   // 4

    let parity = if y % 2 == 0 { "even" } else { "odd" };
    println!("{y} is {parity}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20y%20%3D%20%7B%0A%20%20%20%20%20%20%20%20let%20x%20%3D%203%3B%0A%20%20%20%20%20%20%20%20x%20%2B%201%20%20%20%20%20%20%20%20%20%20%2F%2F%20no%20semicolon%20%3D%3D%20block%27s%20value%0A%20%20%20%20%7D%3B%0A%20%20%20%20println%21%28%22y%20%3D%20%7By%7D%22%29%3B%20%20%20%2F%2F%204%0A%0A%20%20%20%20let%20parity%20%3D%20if%20y%20%25%202%20%3D%3D%200%20%7B%20%22even%22%20%7D%20else%20%7B%20%22odd%22%20%7D%3B%0A%20%20%20%20println%21%28%22%7By%7D%20is%20%7Bparity%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `{ ... }` blocks are expressions
- `if` is an expression
- `loop`, `match`, and even `return` all play nicely with this

Note:
This is really the same idea you have already seen in Racket: everything
is an expression. The difference is that Rust also has statements (let
bindings, items like fn definitions), and the semicolon is what
converts an expression into a statement.

---

## Control flow: `if`, `loop`, `while`, `for`

```rust []
fn main() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("{i} is even");
        }
    }

    let mut n = 0;
    let tripled = loop {
        n += 1;
        if n == 10 { break n * 3; }   // `loop` can return a value
    };
    println!("tripled = {tripled}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main()%20%7B%0A%20%20%20%20for%20i%20in%200..5%20%7B%0A%20%20%20%20%20%20%20%20if%20i%20%25%202%20%3D%3D%200%20%7B%20println!(%22%7Bi%7D%20is%20even%22)%3B%20%7D%0A%20%20%20%20%7D%0A%20%20%20%20let%20mut%20n%20%3D%200%3B%0A%20%20%20%20let%20tripled%20%3D%20loop%20%7B%0A%20%20%20%20%20%20%20%20n%20%2B%3D%201%3B%0A%20%20%20%20%20%20%20%20if%20n%20%3D%3D%2010%20%7B%20break%20n%20*%203%3B%20%7D%0A%20%20%20%20%7D%3B%0A%20%20%20%20println!(%22tripled%20%3D%20%7Btripled%7D%22)%3B%0A%7D">&#9654; Run in Playground</a>

- `0..5` is a **range** (exclusive); `0..=5` includes 5
- `break value;` exits a `loop` with that value
- No parentheses around `if` conditions (but the braces are required)
