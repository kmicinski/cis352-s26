<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 2 &bull; Chapter 10</span>

# Modules, Crates, Testing

## Organizing real code

---

## Crates and packages

- A **crate** is the compilation unit: one library or one binary.
- A **package** is one or more crates sharing a `Cargo.toml`.
- The standard library crate is called `std`.
- Third-party crates live on [crates.io](https://crates.io/).

```toml
# Cargo.toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
reqwest = "0.12"
tokio = { version = "1", features = ["full"] }
```

Note:
crates.io is the Rust equivalent of npm. You add a line to Cargo.toml,
cargo downloads and compiles it on the next build. Lockfile support is
built in. Versions use semver with a "default is caret" rule.

---

## Modules: in-crate namespacing

```rust []
// src/lib.rs
mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }

    pub mod advanced {
        pub fn gcd(a: u32, b: u32) -> u32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }
    }
}

fn main() {
    let s = math::add(2, 3);
    let g = math::advanced::gcd(12, 18);
    println!("sum={s}, gcd={g}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20src%2Flib.rs%0Amod%20math%20%7B%0A%20%20%20%20pub%20fn%20add%28a%3A%20i32%2C%20b%3A%20i32%29%20-%3E%20i32%20%7B%20a%20%2B%20b%20%7D%0A%0A%20%20%20%20pub%20mod%20advanced%20%7B%0A%20%20%20%20%20%20%20%20pub%20fn%20gcd%28a%3A%20u32%2C%20b%3A%20u32%29%20-%3E%20u32%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20if%20b%20%3D%3D%200%20%7B%20a%20%7D%20else%20%7B%20gcd%28b%2C%20a%20%25%20b%29%20%7D%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20s%20%3D%20math%3A%3Aadd%282%2C%203%29%3B%0A%20%20%20%20let%20g%20%3D%20math%3A%3Aadvanced%3A%3Agcd%2812%2C%2018%29%3B%0A%20%20%20%20println%21%28%22sum%3D%7Bs%7D%2C%20gcd%3D%7Bg%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `mod` declares a module (inline or from a file)
- `pub` makes an item visible outside its parent
- Default visibility is **private**

---

## `use` brings names into scope

```rust []
use std::collections::HashMap;
use std::io::{self, Read, Write};    // groups
use std::fs::File as FsFile;         // rename

fn main() -> io::Result<()> {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("answer".into(), 42);
    println!("{map:?}");
    Ok(())
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Acollections%3A%3AHashMap%3B%0Ause%20std%3A%3Aio%3A%3A%7Bself%2C%20Read%2C%20Write%7D%3B%20%20%20%20%2F%2F%20groups%0Ause%20std%3A%3Afs%3A%3AFile%20as%20FsFile%3B%20%20%20%20%20%20%20%20%20%2F%2F%20rename%0A%0Afn%20main%28%29%20-%3E%20io%3A%3AResult%3C%28%29%3E%20%7B%0A%20%20%20%20let%20mut%20map%3A%20HashMap%3CString%2C%20i32%3E%20%3D%20HashMap%3A%3Anew%28%29%3B%0A%20%20%20%20map.insert%28%22answer%22.into%28%29%2C%2042%29%3B%0A%20%20%20%20println%21%28%22%7Bmap%3A%3F%7D%22%29%3B%0A%20%20%20%20Ok%28%28%29%29%0A%7D%0A">&#9654; Run in Playground</a>

- Like Python's `from X import Y`
- `as` renames; `self` pulls in the module itself; `{A, B}` groups

Note:
Most crates publish a "prelude" module that re-exports the common
names. You'll see `use foo::prelude::*;` a lot. It's the rust way of
keeping imports tidy.

---

## Unit tests live next to the code

```rust []
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_negative() {
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    #[should_panic(expected = "divide")]
    fn panics_on_zero() {
        panic!("divide by zero");
    }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=pub%20fn%20add%28a%3A%20i32%2C%20b%3A%20i32%29%20-%3E%20i32%20%7B%20a%20%2B%20b%20%7D%0A%0A%23%5Bcfg%28test%29%5D%0Amod%20tests%20%7B%0A%20%20%20%20use%20super%3A%3A%2A%3B%0A%0A%20%20%20%20%23%5Btest%5D%0A%20%20%20%20fn%20adds_positive%28%29%20%7B%0A%20%20%20%20%20%20%20%20assert_eq%21%28add%282%2C%203%29%2C%205%29%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20%23%5Btest%5D%0A%20%20%20%20fn%20adds_negative%28%29%20%7B%0A%20%20%20%20%20%20%20%20assert_eq%21%28add%28-2%2C%20-3%29%2C%20-5%29%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20%23%5Btest%5D%0A%20%20%20%20%23%5Bshould_panic%28expected%20%3D%20%22divide%22%29%5D%0A%20%20%20%20fn%20panics_on_zero%28%29%20%7B%0A%20%20%20%20%20%20%20%20panic%21%28%22divide%20by%20zero%22%29%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

```bash
$ cargo test
running 3 tests
test tests::adds_positive ... ok
test tests::adds_negative ... ok
test tests::panics_on_zero - should panic ... ok
```

Note:
Tests in Rust live in the same file as the code they test, inside a
`mod tests` gated by `#[cfg(test)]` so they don't ship in release
builds. This keeps tests physically close to their subject and means
they can poke at private items. Integration tests live in a separate
`tests/` directory and see only the public API.

---

## Documentation is first-class

```rust []
/// Returns the sum of two integers.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%2F%20Returns%20the%20sum%20of%20two%20integers.%0A%2F%2F%2F%0A%2F%2F%2F%20%23%20Examples%0A%2F%2F%2F%0A%2F%2F%2F%20%60%60%60%0A%2F%2F%2F%20let%20result%20%3D%20my_crate%3A%3Aadd%282%2C%203%29%3B%0A%2F%2F%2F%20assert_eq%21%28result%2C%205%29%3B%0A%2F%2F%2F%20%60%60%60%0Apub%20fn%20add%28a%3A%20i32%2C%20b%3A%20i32%29%20-%3E%20i32%20%7B%20a%20%2B%20b%20%7D%0A%0Afn%20main%28%29%20%7B%7D%0A">&#9654; Run in Playground</a>

```bash
$ cargo doc --open      # generates HTML and opens it
$ cargo test             # doctests are run too!
```

- `///` starts a doc comment
- Code blocks in doc comments become tests
- Whole standard library is documented this way

Note:
Doctests are one of my favorite Rust features. Your examples in the
docs actually run as tests, so they can't rot. The entire rust-lang.org
API docs you've seen are just this: cargo doc output.
