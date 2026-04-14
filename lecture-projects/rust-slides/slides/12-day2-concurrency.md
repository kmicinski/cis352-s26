<!-- .slide: class="section-divider" -->
<span class="chapter-num">Day 2 &bull; Chapter 11</span>

# Fearless Concurrency

## Threads, channels, `Arc`, `Mutex`, and async

---

## Spawning a thread

```rust []
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  worker: {i}");
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..=3 {
        println!("main: {i}");
        thread::sleep(Duration::from_millis(80));
    }

    handle.join().unwrap();
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Athread%3B%0Ause%20std%3A%3Atime%3A%3ADuration%3B%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20handle%20%3D%20thread%3A%3Aspawn%28%7C%7C%20%7B%0A%20%20%20%20%20%20%20%20for%20i%20in%201..%3D5%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20println%21%28%22%20%20worker%3A%20%7Bi%7D%22%29%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20thread%3A%3Asleep%28Duration%3A%3Afrom_millis%2850%29%29%3B%0A%20%20%20%20%20%20%20%20%7D%0A%20%20%20%20%7D%29%3B%0A%0A%20%20%20%20for%20i%20in%201..%3D3%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22main%3A%20%7Bi%7D%22%29%3B%0A%20%20%20%20%20%20%20%20thread%3A%3Asleep%28Duration%3A%3Afrom_millis%2880%29%29%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20handle.join%28%29.unwrap%28%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

- `thread::spawn` takes a closure and returns a `JoinHandle`
- `.join()` waits for the thread to finish

Note:
Notice the closure isn't annotated with a lifetime. Spawned threads need
`'static` captures (they might outlive main), which is why the next
slide introduces `move`.

---

## Sharing data: the `move` closure

```rust []
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {        // move data into thread
        println!("sum = {}", data.iter().sum::<i32>());
    });

    // println!("{data:?}");   // error: data was moved

    handle.join().unwrap();
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Athread%3B%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20data%20%3D%20vec%21%5B1%2C%202%2C%203%5D%3B%0A%0A%20%20%20%20let%20handle%20%3D%20thread%3A%3Aspawn%28move%20%7C%7C%20%7B%20%20%20%20%20%20%20%20%2F%2F%20move%20data%20into%20thread%0A%20%20%20%20%20%20%20%20println%21%28%22sum%20%3D%20%7B%7D%22%2C%20data.iter%28%29.sum%3A%3A%3Ci32%3E%28%29%29%3B%0A%20%20%20%20%7D%29%3B%0A%0A%20%20%20%20%2F%2F%20println%21%28%22%7Bdata%3A%3F%7D%22%29%3B%20%20%20%2F%2F%20error%3A%20data%20was%20moved%0A%0A%20%20%20%20handle.join%28%29.unwrap%28%29%3B%0A%7D%0A">&#9654; Run in Playground</a>

Without `move`, the compiler would refuse: the closure might outlive `data`.

Note:
This is the borrow checker doing its usual job in a new context. Without
the move, the compiler can see that `data` might still be used on the
main thread while the worker is running. With `move`, ownership transfers
and the main thread can't touch `data` any more. Data race prevented,
statically.

---

## Channels: message passing

```rust []
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..4 {
        let tx = tx.clone();    // each thread gets its own sender
        thread::spawn(move || {
            tx.send(i * i).unwrap();
        });
    }
    drop(tx);                   // drop the extra sender so rx terminates

    for result in rx {
        println!("got {result}");
    }
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Async%3A%3Ampsc%3B%0Ause%20std%3A%3Athread%3B%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20%28tx%2C%20rx%29%20%3D%20mpsc%3A%3Achannel%28%29%3B%0A%0A%20%20%20%20for%20i%20in%200..4%20%7B%0A%20%20%20%20%20%20%20%20let%20tx%20%3D%20tx.clone%28%29%3B%20%20%20%20%2F%2F%20each%20thread%20gets%20its%20own%20sender%0A%20%20%20%20%20%20%20%20thread%3A%3Aspawn%28move%20%7C%7C%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20tx.send%28i%20%2A%20i%29.unwrap%28%29%3B%0A%20%20%20%20%20%20%20%20%7D%29%3B%0A%20%20%20%20%7D%0A%20%20%20%20drop%28tx%29%3B%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%20%2F%2F%20drop%20the%20extra%20sender%20so%20rx%20terminates%0A%0A%20%20%20%20for%20result%20in%20rx%20%7B%0A%20%20%20%20%20%20%20%20println%21%28%22got%20%7Bresult%7D%22%29%3B%0A%20%20%20%20%7D%0A%7D%0A">&#9654; Run in Playground</a>

- `mpsc` = multi-producer, single-consumer
- `Sender::clone()` gives each thread its own handle
- Iterating `rx` blocks until all senders drop

Note:
The mantra in Rust is "don't communicate by sharing memory, share memory
by communicating." Channels are the default. The std library channel is
fine; for fancier ones, reach for `crossbeam-channel` or `flume`.

---

## Shared state: `Arc<Mutex<T>>`

When you really need shared mutable state:

```rust []
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("final: {}", *counter.lock().unwrap());   // 10
}
```

<a class="playground" target="_blank" href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use%20std%3A%3Async%3A%3A%7BArc%2C%20Mutex%7D%3B%0Ause%20std%3A%3Athread%3B%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20let%20counter%20%3D%20Arc%3A%3Anew%28Mutex%3A%3Anew%280%29%29%3B%0A%20%20%20%20let%20mut%20handles%20%3D%20vec%21%5B%5D%3B%0A%0A%20%20%20%20for%20_%20in%200..10%20%7B%0A%20%20%20%20%20%20%20%20let%20counter%20%3D%20Arc%3A%3Aclone%28%26counter%29%3B%0A%20%20%20%20%20%20%20%20handles.push%28thread%3A%3Aspawn%28move%20%7C%7C%20%7B%0A%20%20%20%20%20%20%20%20%20%20%20%20let%20mut%20num%20%3D%20counter.lock%28%29.unwrap%28%29%3B%0A%20%20%20%20%20%20%20%20%20%20%20%20%2Anum%20%2B%3D%201%3B%0A%20%20%20%20%20%20%20%20%7D%29%29%3B%0A%20%20%20%20%7D%0A%0A%20%20%20%20for%20h%20in%20handles%20%7B%20h.join%28%29.unwrap%28%29%3B%20%7D%0A%20%20%20%20println%21%28%22final%3A%20%7B%7D%22%2C%20%2Acounter.lock%28%29.unwrap%28%29%29%3B%20%20%20%2F%2F%2010%0A%7D%0A">&#9654; Run in Playground</a>

- `Arc<T>` &mdash; **A**tomically **R**eference-**C**ounted shared pointer
- `Mutex<T>` &mdash; mutual exclusion, returns `MutexGuard` on lock
- The guard automatically unlocks when it goes out of scope

Note:
This is the closest you get to "classic" shared-memory threading. The
lock-free version of Arc is Rc, which is single-threaded only; the
compiler actually prevents you from sending an Rc across threads. That
protection comes from the Send and Sync marker traits, which I'll
mention next.

---

## `Send` and `Sync`: the safety markers

Two auto-implemented traits:

- **`Send`** &mdash; a type is safe to *move* across threads.
- **`Sync`** &mdash; a type is safe to *share* (`&T`) across threads.

Most types are both. Exceptions:

| Type | Send? | Sync? | Why |
|---|:---:|:---:|---|
| `Arc<T>` | yes | yes | atomic refcount |
| `Rc<T>` | **no** | **no** | non-atomic refcount |
| `Cell<T>`, `RefCell<T>` | yes | **no** | interior mutability |
| `MutexGuard<'_, T>` | **no** | yes | locks are thread-affine |

These markers are how the compiler rejects `thread::spawn` with a non-`Send` closure.

Note:
This is the trick that makes "fearless concurrency" the tagline. The
compiler rejects every attempt to cross a thread boundary with a type
that isn't Send (or Sync for shared references). You do not think about
this most of the time &mdash; the default is correct. But when the
compiler says no, the Send/Sync analysis is what's behind it.

---

## A peek at async

Rust has cooperative concurrency too, built on `Future`:

```rust
use reqwest;                    // HTTP client

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("got {} bytes", body.len());
    Ok(())
}
```

- `async fn` creates a function that returns a `Future`
- `.await` suspends until the future resolves
- Runtimes like [tokio](https://tokio.rs/) do the scheduling
- Zero-cost: no extra threads unless you ask for them

Note:
Async in Rust is a big topic that would take a whole lecture on its own.
The elevator pitch: `async/await` is syntactic sugar for a state
machine that the compiler generates automatically. You need a runtime
(tokio is the de facto standard) to actually drive these futures. The
main thing to know today is that async exists and is how most network
code is written.
