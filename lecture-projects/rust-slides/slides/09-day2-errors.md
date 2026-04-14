<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 2 &bull; Chapter 8</span>

# Error Handling

## `Result<T, E>`, `?`, and the "errors are values" philosophy

---

## Two kinds of errors

**Unrecoverable** &mdash; bugs, invariant violations, OOM. Use `panic!`.

```rust
panic!("invariant broken: queue was empty");
```

**Recoverable** &mdash; things the caller might want to handle: file not
found, parse error, network timeout. Use `Result<T, E>`.

```rust []
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Note:
Rust takes the hard line that exceptions are a bad idea. Errors are
values, the type system tracks them, and the caller is forced to think
about them. This is the same philosophy as Go's error returns, except
that Rust gives you `?` so it isn't painful, and the type system so you
can't silently drop an error.

---

## A function that can fail

```rust []
use std::num::ParseIntError;

fn parse_age(s: &str) -> Result<u32, ParseIntError> {
    s.trim().parse::<u32>()
}

fn main() {
    match parse_age("42") {
        Ok(n) => println!("age is {n}"),
        Err(e) => println!("couldn't parse: {e}"),
    }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Anum%3A%3AParseIntError%3B%0A%0Afn%20parse_age%28s%3A%20%26str%29%20-%3E%20Result%3Cu32%2C%20ParseIntError%3E%20%7B%0A%20%20%20%20s.trim%28%29.parse%3A%3A%3Cu32%3E%28%29%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20match%20parse_age%28%2242%22%29%20%7B%0A%20%20%20%20%20%20%20%20Ok%28n%29%20%3D%3E%20println%21%28%22age%20is%20%7Bn%7D%22%29%2C%0A%20%20%20%20%20%20%20%20Err%28e%29%20%3D%3E%20println%21%28%22couldn%27t%20parse%3A%20%7Be%7D%22%29%2C%0A%20%20%20%20%7D%0A%7D%0A">&#9654; Run in Playground</a>

- `parse::<u32>()` returns a `Result<u32, ParseIntError>`
- The caller **must** handle both cases &mdash; the compiler warns if they don't
- No forgotten errors, no silently-ignored return codes

---

## The `?` operator

Chaining `match` blocks is tedious. `?` early-returns on error:

```rust []
use std::num::ParseIntError;

fn parse_pair(s: &str) -> Result<(u32, u32), ParseIntError> {
    let (a, b) = s.split_once(',').unwrap();
    let a = a.trim().parse::<u32>()?;   // bail if Err
    let b = b.trim().parse::<u32>()?;   // bail if Err
    Ok((a, b))
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Anum%3A%3AParseIntError%3B%0A%0Afn%20parse_pair%28s%3A%20%26str%29%20-%3E%20Result%3C%28u32%2C%20u32%29%2C%20ParseIntError%3E%20%7B%0A%20%20%20%20let%20%28a%2C%20b%29%20%3D%20s.split_once%28%27%2C%27%29.unwrap%28%29%3B%0A%20%20%20%20let%20a%20%3D%20a.trim%28%29.parse%3A%3A%3Cu32%3E%28%29%3F%3B%20%20%20%2F%2F%20bail%20if%20Err%0A%20%20%20%20let%20b%20%3D%20b.trim%28%29.parse%3A%3A%3Cu32%3E%28%29%3F%3B%20%20%20%2F%2F%20bail%20if%20Err%0A%20%20%20%20Ok%28%28a%2C%20b%29%29%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

`expr?` expands to roughly:

```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
}
```

Note:
This operator is the single feature that makes Rust error handling
pleasant. It composes Results the way `await` composes Futures. The
`.into()` call is important: it means `?` will automatically convert
from one error type to another if there's a `From` impl &mdash; which is
how you unify heterogeneous errors in real code.

---

## Custom error types

```rust []
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    NotFound,
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self { AppError::Parse(e) }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%23%5Bderive%28Debug%29%5D%0Aenum%20AppError%20%7B%0A%20%20%20%20Io%28std%3A%3Aio%3A%3AError%29%2C%0A%20%20%20%20Parse%28std%3A%3Anum%3A%3AParseIntError%29%2C%0A%20%20%20%20NotFound%2C%0A%7D%0A%0Aimpl%20From%3Cstd%3A%3Aio%3A%3AError%3E%20for%20AppError%20%7B%0A%20%20%20%20fn%20from%28e%3A%20std%3A%3Aio%3A%3AError%29%20-%3E%20Self%20%7B%20AppError%3A%3AIo%28e%29%20%7D%0A%7D%0A%0Aimpl%20From%3Cstd%3A%3Anum%3A%3AParseIntError%3E%20for%20AppError%20%7B%0A%20%20%20%20fn%20from%28e%3A%20std%3A%3Anum%3A%3AParseIntError%29%20-%3E%20Self%20%7B%20AppError%3A%3AParse%28e%29%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

Now `?` automatically converts to `AppError` across both source types.

In real code you would reach for the [`thiserror`](https://docs.rs/thiserror) crate to generate this boilerplate.

Note:
This pattern is so common that a whole ecosystem exists around it:
`thiserror` for library errors, `anyhow` for application errors, and
`eyre` for fancier reports. The ergonomics are great once you know the
pattern.

---

## `Option` and `Result` are friends

Both have rich adapter methods that let you transform values without matching:

```rust []
fn lookup(_id: u32) -> Option<String> { None }

fn main() {
    let name_or_default = lookup(42).unwrap_or_else(|| "anon".into());
    let upper = lookup(42).map(|s| s.to_uppercase());
    let result: Result<String, &str> = lookup(42).ok_or("not found");
    println!("{name_or_default} {upper:?} {result:?}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20lookup%28_id%3A%20u32%29%20-%3E%20Option%3CString%3E%20%7B%20None%20%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20name_or_default%20%3D%20lookup%2842%29.unwrap_or_else%28%7C%7C%20%22anon%22.into%28%29%29%3B%0A%20%20%20%20let%20upper%20%3D%20lookup%2842%29.map%28%7Cs%7C%20s.to_uppercase%28%29%29%3B%0A%20%20%20%20let%20result%3A%20Result%3CString%2C%20%26str%3E%20%3D%20lookup%2842%29.ok_or%28%22not%20found%22%29%3B%0A%20%20%20%20println%21%28%22%7Bname_or_default%7D%20%7Bupper%3A%3F%7D%20%7Bresult%3A%3F%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `.unwrap_or(default)`, `.unwrap_or_else(|| ...)`
- `.map(f)`, `.and_then(f)`, `.or_else(f)`
- `.ok()` turns `Result` into `Option`; `.ok_or(e)` does the reverse

Note:
You rarely match on Option directly once you've been in Rust for a
while. These adapters express most of what you want in one line, and
they compose beautifully with iterators, which we're about to see.
