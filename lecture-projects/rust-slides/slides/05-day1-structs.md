<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 1 &bull; Chapter 5</span>

# Structs &amp; Methods

## Bringing it all together

---

## Defining a struct

```rust []
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    let r = Rectangle { width: 3.0, height: 4.0 };
    println!("w={}, h={}", r.width, r.height);
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=struct%20Rectangle%20%7B%0A%20%20%20%20width%3A%20f64%2C%0A%20%20%20%20height%3A%20f64%2C%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20r%20%3D%20Rectangle%20%7B%20width%3A%203.0%2C%20height%3A%204.0%20%7D%3B%0A%20%20%20%20println%21%28%22w%3D%7B%7D%2C%20h%3D%7B%7D%22%2C%20r.width%2C%20r.height%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- Like a Racket struct, or a C struct, or a Java "plain old data" class
- Fields are `name: type`
- Access with `.` and construct with `Name { field: value, ... }`

Note:
Rust structs are dumb data. They do not have methods attached to them
in the declaration; methods go in a separate `impl` block, which we'll
see on the next slide. This means you can add methods to a type without
editing the original declaration.

---

## Methods live in `impl` blocks

```rust []
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function (no &self) &mdash; like a static method
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    // Method &mdash; takes &self, an immutable borrow of the receiver
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Method that mutates the receiver
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}
```

---

## Using it

```rust []
fn main() {
    let mut r = Rectangle::new(3.0, 4.0);
    println!("area = {}", r.area());
    r.scale(2.0);
    println!("after: w={}, h={}", r.width, r.height);
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=struct%20Rectangle%20%7B%0A%20%20%20%20width%3A%20f64%2C%0A%20%20%20%20height%3A%20f64%2C%0A%7D%0A%0Aimpl%20Rectangle%20%7B%0A%20%20%20%20fn%20new(width%3A%20f64%2C%20height%3A%20f64)%20-%3E%20Self%20%7B%0A%20%20%20%20%20%20%20%20Self%20%7B%20width%2C%20height%20%7D%0A%20%20%20%20%7D%0A%20%20%20%20fn%20area(%26self)%20-%3E%20f64%20%7B%20self.width%20*%20self.height%20%7D%0A%20%20%20%20fn%20scale(%26mut%20self%2C%20factor%3A%20f64)%20%7B%0A%20%20%20%20%20%20%20%20self.width%20*%3D%20factor%3B%0A%20%20%20%20%20%20%20%20self.height%20*%3D%20factor%3B%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20let%20mut%20r%20%3D%20Rectangle%3A%3Anew(3.0%2C%204.0)%3B%0A%20%20%20%20println!(%22area%20%3D%20%7B%7D%22%2C%20r.area())%3B%0A%20%20%20%20r.scale(2.0)%3B%0A%20%20%20%20println!(%22after%3A%20w%3D%7B%7D%2C%20h%3D%7B%7D%22%2C%20r.width%2C%20r.height)%3B%0A%7D">&#9654; Run in Playground</a>

- `::` calls an *associated* function (`Rectangle::new`)
- `.` calls a *method* on a value (`r.area()`)
- The receiver style (`&self`, `&mut self`, `self`) determines what you can do

Note:
The `&self` / `&mut self` / `self` distinction is the same as `&T` /
`&mut T` / `T` from the last chapter. `&self` is a read-only borrow of
the receiver, `&mut self` is an exclusive borrow for mutation, and
`self` moves the receiver (consuming it). You'll see all three.

---

## Tuple structs and unit structs

```rust []
// Tuple struct: named tuple
struct Wrapping(u32);
struct Meters(f64);
struct Seconds(f64);

// Unit struct: no fields, but still a type
struct Marker;

fn main() {
    let distance = Meters(42.0);
    let duration = Seconds(7.0);
    // speed = distance / duration; // compile error &mdash; good!
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%2F%2F%20Tuple%20struct%3A%20named%20tuple%0Astruct%20Wrapping%28u32%29%3B%0Astruct%20Meters%28f64%29%3B%0Astruct%20Seconds%28f64%29%3B%0A%0A%2F%2F%20Unit%20struct%3A%20no%20fields%2C%20but%20still%20a%20type%0Astruct%20Marker%3B%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20distance%20%3D%20Meters%2842.0%29%3B%0A%20%20%20%20let%20duration%20%3D%20Seconds%287.0%29%3B%0A%20%20%20%20%2F%2F%20speed%20%3D%20distance%20%2F%20duration%3B%20%2F%2F%20compile%20error%20%26mdash%3B%20good%21%0A%7D%0A">&#9654; Run in Playground</a>

Distinct types for distinct concepts &mdash; the compiler prevents unit mix-ups.

Note:
"Newtype" wrappers like Meters and Seconds are a very common Rust
pattern. They cost nothing at runtime &mdash; the compiler erases them &mdash;
but they prevent whole categories of bug where you accidentally add
inches to seconds. If you remember the Mars Climate Orbiter, that's
the category of bug.

---

## Deriving common traits

```rust []
#[derive(Debug, Clone, PartialEq)]
struct Point { x: f64, y: f64 }

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    let q = p.clone();
    println!("{p:?}");        // Point { x: 1.0, y: 2.0 }
    println!("p == q? {}", p == q);
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%23%5Bderive%28Debug%2C%20Clone%2C%20PartialEq%29%5D%0Astruct%20Point%20%7B%20x%3A%20f64%2C%20y%3A%20f64%20%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20p%20%3D%20Point%20%7B%20x%3A%201.0%2C%20y%3A%202.0%20%7D%3B%0A%20%20%20%20let%20q%20%3D%20p.clone%28%29%3B%0A%20%20%20%20println%21%28%22%7Bp%3A%3F%7D%22%29%3B%20%20%20%20%20%20%20%20%2F%2F%20Point%20%7B%20x%3A%201.0%2C%20y%3A%202.0%20%7D%0A%20%20%20%20println%21%28%22p%20%3D%3D%20q%3F%20%7B%7D%22%2C%20p%20%3D%3D%20q%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `#[derive(...)]` asks the compiler to auto-generate trait impls
- `Debug` enables `{:?}` printing
- `Clone` gives you `.clone()`
- `PartialEq` enables `==` and `!=`

Note:
Derive is Rust's killer feature for boilerplate. In Java you'd have
IDE-generated equals/hashCode/toString methods that you have to keep in
sync by hand. In Rust you write derive once and the compiler regenerates
them on every build, so they're always correct.

---

## Putting it together: a mini program

```rust []
#[derive(Debug)]
enum Event { Click { x: i32, y: i32 }, KeyPress(char), Focus, Blur }

fn summarize(events: &[Event]) -> (usize, usize) {
    let mut clicks = 0;
    let mut keys = 0;
    for e in events {
        match e {
            Event::Click { .. } => clicks += 1,
            Event::KeyPress(_)  => keys += 1,
            _ => {}
        }
    }
    (clicks, keys)
}

fn main() {
    let events = vec![
        Event::Click { x: 10, y: 20 },
        Event::KeyPress('r'),
        Event::Focus,
        Event::Click { x: 30, y: 40 },
    ];
    let (c, k) = summarize(&events);
    println!("{c} clicks, {k} key presses");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=%23%5Bderive%28Debug%29%5D%0Aenum%20Event%20%7B%20Click%20%7B%20x%3A%20i32%2C%20y%3A%20i32%20%7D%2C%20KeyPress%28char%29%2C%20Focus%2C%20Blur%20%7D%0A%0Afn%20summarize%28events%3A%20%26%5BEvent%5D%29%20-%3E%20%28usize%2C%20usize%29%20%7B%0A%20%20%20%20let%20mut%20clicks%20%3D%200%3B%0A%20%20%20%20let%20mut%20keys%20%3D%200%3B%0A%20%20%20%20for%20e%20in%20events%20%7B%0A%20%20%20%20%20%20%20%20match%20e%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20Event%3A%3AClick%20%7B%20..%20%7D%20%3D%3E%20clicks%20%2B%3D%201%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20Event%3A%3AKeyPress%28_%29%20%20%3D%3E%20keys%20%2B%3D%201%2C%0A%20%20%20%20%20%20%20%20%20%20%20%20_%20%3D%3E%20%7B%7D%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%0A%20%20%20%20%28clicks%2C%20keys%29%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20events%20%3D%20vec%21%5B%0A%20%20%20%20%20%20%20%20Event%3A%3AClick%20%7B%20x%3A%2010%2C%20y%3A%2020%20%7D%2C%0A%20%20%20%20%20%20%20%20Event%3A%3AKeyPress%28%27r%27%29%2C%0A%20%20%20%20%20%20%20%20Event%3A%3AFocus%2C%0A%20%20%20%20%20%20%20%20Event%3A%3AClick%20%7B%20x%3A%2030%2C%20y%3A%2040%20%7D%2C%0A%20%20%20%20%5D%3B%0A%20%20%20%20let%20%28c%2C%20k%29%20%3D%20summarize%28%26events%29%3B%0A%20%20%20%20println%21%28%22%7Bc%7D%20clicks%2C%20%7Bk%7D%20key%20presses%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

Everything you've seen today &mdash; enums, slices, borrows, pattern matching &mdash; working together.

---

<!-- .slide: class="big-point" -->

# End of Day 1

See you next time for **lifetimes, traits, and the rest of the iceberg.**

Note:
Before we end: any questions about today? The big things to digest
before day 2 are ownership (especially the move/borrow rules) and
pattern matching. Day 2 assumes both.
