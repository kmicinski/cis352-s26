<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 2 &bull; Chapter 9</span>

# Closures &amp; Iterators

## Functional pipelines that compile to tight loops

---

## Closures: anonymous functions that capture

```rust []
fn main() {
    let threshold = 10;
    let is_big = |x: i32| x > threshold;   // captures `threshold`

    println!("{}", is_big(5));     // false
    println!("{}", is_big(42));    // true

    let double = |x| x * 2;         // types can be inferred
    let nums = [1, 2, 3].map(double);
    println!("{nums:?}");          // [2, 4, 6]
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20let%20threshold%20%3D%2010%3B%0A%20%20%20%20let%20is_big%20%3D%20%7Cx%3A%20i32%7C%20x%20%3E%20threshold%3B%20%20%20%2F%2F%20captures%20%60threshold%60%0A%0A%20%20%20%20println%21%28%22%7B%7D%22%2C%20is_big%285%29%29%3B%20%20%20%20%20%2F%2F%20false%0A%20%20%20%20println%21%28%22%7B%7D%22%2C%20is_big%2842%29%29%3B%20%20%20%20%2F%2F%20true%0A%0A%20%20%20%20let%20double%20%3D%20%7Cx%7C%20x%20%2A%202%3B%20%20%20%20%20%20%20%20%20%2F%2F%20types%20can%20be%20inferred%0A%20%20%20%20let%20nums%20%3D%20%5B1%2C%202%2C%203%5D.map%28double%29%3B%0A%20%20%20%20println%21%28%22%7Bnums%3A%3F%7D%22%29%3B%20%20%20%20%20%20%20%20%20%20%2F%2F%20%5B2%2C%204%2C%206%5D%0A%7D%0A">&#9654; Run in Playground</a>

- Syntax: `|args| expr` or `|args| { block }`
- Captures surrounding variables by reference, mut reference, or move
- `move |...| ...` forces a by-value capture (useful for threads)

Note:
Closures will feel very familiar from Racket. The big difference is
that Rust has three flavors based on how they use their captures: Fn,
FnMut, FnOnce. You don't usually write these by hand &mdash; the compiler
picks the right one &mdash; but you'll see them in the signatures of
functions that take closures.

---

## The iterator trait

```rust []
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ~80 default methods, all built on top of `next`
}
```

Everything that can be walked implements this trait.

Implementing `next` gives you `map`, `filter`, `sum`, `collect`,
`zip`, `enumerate`, `take`, `skip`, `chain`, `flatten`, `fold`,
`any`, `all`, `count`, `max`, `min`, `rev`, `peekable`, ...

Note:
This is one of the more elegant designs in the standard library. You
only implement `next`; the trait gives you 80+ derived methods for
free. The same pattern shows up in Python iterators and Java streams,
but in Rust it is zero-overhead: the whole chain gets inlined into a
single loop.

---

## A functional pipeline

```rust []
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum();

    println!("{sum_of_even_squares}");   // 220
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main()%20%7B%0A%20%20%20%20let%20numbers%20%3D%20vec!%5B1%2C2%2C3%2C4%2C5%2C6%2C7%2C8%2C9%2C10%5D%3B%0A%20%20%20%20let%20sum_of_even_squares%3A%20i32%20%3D%20numbers%0A%20%20%20%20%20%20%20%20.iter()%0A%20%20%20%20%20%20%20%20.filter(%7C%26%26n%7C%20n%20%25%202%20%3D%3D%200)%0A%20%20%20%20%20%20%20%20.map(%7C%26n%7C%20n%20*%20n)%0A%20%20%20%20%20%20%20%20.sum()%3B%0A%20%20%20%20println!(%22%7Bsum_of_even_squares%7D%22)%3B%0A%7D">&#9654; Run in Playground</a>

This compiles down to **one loop, one accumulator, zero allocations.**

Note:
This is the big claim of the iterator design: the chained adapter calls
do NOT produce intermediate collections. Each call wraps the previous
iterator in a new struct that's then inlined away. A benchmark compared
to a hand-written for loop will produce identical assembly.

---

## Three ways to iterate

```rust []
fn main() {
    // 1. Borrow each element immutably
    let v = vec![1, 2, 3];
    for x in v.iter() { println!("{x}"); }
    println!("still ours: {v:?}");

    // 2. Borrow mutably to modify in place
    let mut v = vec![1, 2, 3];
    for x in v.iter_mut() { *x *= 2; }
    println!("doubled:    {v:?}");

    // 3. Consume the vec, element by element
    let v = vec![1, 2, 3];
    for x in v.into_iter() { println!("{x}"); }
    // v has been moved here &mdash; can't use it any more
}

// For-loop sugar, for reference:
//     for x in &v      == v.iter()
//     for x in &mut v  == v.iter_mut()
//     for x in v       == v.into_iter()  (consumes v!)
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20main%28%29%20%7B%0A%20%20%20%20%2F%2F%201.%20Borrow%20each%20element%20immutably%0A%20%20%20%20let%20v%20%3D%20vec%21%5B1%2C%202%2C%203%5D%3B%0A%20%20%20%20for%20x%20in%20v.iter%28%29%20%7B%20println%21%28%22%7Bx%7D%22%29%3B%20%7D%0A%20%20%20%20println%21%28%22still%20ours%3A%20%7Bv%3A%3F%7D%22%29%3B%0A%0A%20%20%20%20%2F%2F%202.%20Borrow%20mutably%20to%20modify%20in%20place%0A%20%20%20%20let%20mut%20v%20%3D%20vec%21%5B1%2C%202%2C%203%5D%3B%0A%20%20%20%20for%20x%20in%20v.iter_mut%28%29%20%7B%20%2Ax%20%2A%3D%202%3B%20%7D%0A%20%20%20%20println%21%28%22doubled%3A%20%20%20%20%7Bv%3A%3F%7D%22%29%3B%0A%0A%20%20%20%20%2F%2F%203.%20Consume%20the%20vec%2C%20element%20by%20element%0A%20%20%20%20let%20v%20%3D%20vec%21%5B1%2C%202%2C%203%5D%3B%0A%20%20%20%20for%20x%20in%20v.into_iter%28%29%20%7B%20println%21%28%22%7Bx%7D%22%29%3B%20%7D%0A%20%20%20%20%2F%2F%20v%20has%20been%20moved%20here%20%26mdash%3B%20can%27t%20use%20it%20any%20more%0A%7D%0A%0A%2F%2F%20For-loop%20sugar%2C%20for%20reference%3A%0A%2F%2F%20%20%20%20%20for%20x%20in%20%26v%20%20%20%20%20%20%3D%3D%20v.iter%28%29%0A%2F%2F%20%20%20%20%20for%20x%20in%20%26mut%20v%20%20%3D%3D%20v.iter_mut%28%29%0A%2F%2F%20%20%20%20%20for%20x%20in%20v%20%20%20%20%20%20%20%3D%3D%20v.into_iter%28%29%20%20%28consumes%20v%21%29%0A">&#9654; Run in Playground</a>

Note:
This tripped me up as a beginner. `for x in v` moves the vec &mdash; after
the loop, v is gone. `for x in &v` is the one you want most of the time.
The editor's quick fix will almost always add the ampersand when you
forget.

---

## `collect()` is the escape hatch

```rust []
use std::collections::HashMap;

fn main() {
    // Collect into a Vec
    let doubled: Vec<i32> = (1..=5).map(|n| n * 2).collect();
    println!("{doubled:?}");   // [2, 4, 6, 8, 10]

    // Collect into a HashMap
    let squares: HashMap<i32, i32> = (1..=5)
        .map(|n| (n, n * n))
        .collect();
    println!("{:?}", squares.get(&3));   // Some(9)

    // Collect with error propagation!
    let parsed: Result<Vec<i32>, _> =
        ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{parsed:?}");
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Acollections%3A%3AHashMap%3B%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20%2F%2F%20Collect%20into%20a%20Vec%0A%20%20%20%20let%20doubled%3A%20Vec%3Ci32%3E%20%3D%20%281..%3D5%29.map%28%7Cn%7C%20n%20%2A%202%29.collect%28%29%3B%0A%20%20%20%20println%21%28%22%7Bdoubled%3A%3F%7D%22%29%3B%20%20%20%2F%2F%20%5B2%2C%204%2C%206%2C%208%2C%2010%5D%0A%0A%20%20%20%20%2F%2F%20Collect%20into%20a%20HashMap%0A%20%20%20%20let%20squares%3A%20HashMap%3Ci32%2C%20i32%3E%20%3D%20%281..%3D5%29%0A%20%20%20%20%20%20%20%20.map%28%7Cn%7C%20%28n%2C%20n%20%2A%20n%29%29%0A%20%20%20%20%20%20%20%20.collect%28%29%3B%0A%20%20%20%20println%21%28%22%7B%3A%3F%7D%22%2C%20squares.get%28%263%29%29%3B%20%20%20%2F%2F%20Some%289%29%0A%0A%20%20%20%20%2F%2F%20Collect%20with%20error%20propagation%21%0A%20%20%20%20let%20parsed%3A%20Result%3CVec%3Ci32%3E%2C%20_%3E%20%3D%0A%20%20%20%20%20%20%20%20%5B%221%22%2C%20%222%22%2C%20%223%22%5D.iter%28%29.map%28%7Cs%7C%20s.parse%3A%3A%3Ci32%3E%28%29%29.collect%28%29%3B%0A%20%20%20%20println%21%28%22%7Bparsed%3A%3F%7D%22%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

Note:
`collect` uses a cute trick: it's generic in its return type, so you
annotate the destination and the compiler figures out which collection
you want. The last example is the best: collecting an iterator of
Results into a Result of Vec gives you "all or nothing" parsing for
free.

---

## Common collection types

| Type | Racket analog | Use for |
|---|---|---|
| `Vec<T>` | `list`, `vector` | growable sequence |
| `[T; N]` | &mdash; | stack array, size known |
| `&[T]` | &mdash; | a borrowed view of any of the above |
| `VecDeque<T>` | &mdash; | double-ended queue |
| `HashMap<K, V>` | `hash` | hash table |
| `BTreeMap<K, V>` | &mdash; | ordered hash table |
| `HashSet<T>` | `set` | hash set |
| `String` / `&str` | `string` | text |

Note:
Vec is the workhorse. You will use it 90% of the time. HashMap and
HashSet are the next most common. The B-tree variants give you ordered
iteration at the cost of slightly slower lookups.
