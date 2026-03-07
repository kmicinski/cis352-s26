# Lambda Calculus Explorer — CLAUDE.md

## Project Overview
An immersive, full-page web application for interactively exploring the untyped lambda calculus. Users can enter terms, hover/click to perform reductions, and explore different reduction paths via a branching derivation tree.

**Designed for CIS352** at Syracuse University (Prof. Micinski). Styling matches the CIS352 playground at `kmicinski.com/cis352-s26/playground` — light theme, glassmorphism, SF Mono/JetBrains Mono fonts, indigo accents.

## Architecture

### Rust/WASM Core (`src/`)
All lambda calculus logic lives in Rust, compiled to WASM via `wasm-pack`:

- **`src/term.rs`** — Core `Term` enum (`Var`, `Abs`, `App`), all operations:
  - Capture-avoiding substitution
  - Beta reduction, eta conversion, alpha conversion
  - Path-based navigation (`PathStep`: `Left`/`Right`/`Body`, encoded as `L`/`R`/`B` strings)
  - `map_at_path()` for applying transformations at arbitrary subterm positions
  - Four reduction strategies: Normal, Applicative, CBV, CBN (`next_redex()` for each)
  - 16 unit tests covering substitution, reductions, strategies

- **`src/parser.rs`** — Recursive descent parser for CIS352 syntax:
  - `(λ (x) body)` or `(\ (x) body)` — abstraction (supports `\`, `λ`, `lambda`)
  - `(e1 e2)` — application
  - `x` — variable
  - Sugar: multi-param `(λ (x y z) body)` desugars to nested lambdas; multi-app `(f a b)` is left-associative

- **`src/render.rs`** — Builds a JSON render tree for JS consumption. Each node includes:
  - `kind`, `path`, `ops` (available operations)
  - For variables: `is_free`, `binder_path` (path to the binding lambda)
  - For abs: `param`, `body`; for app: `func`, `arg`

- **`src/lib.rs`** — WASM API (`LambdaEngine` struct exported via `wasm-bindgen`):
  - `parse_and_set()`, `apply_operation()`, `step_strategy()`, `undo()`
  - `get_render_tree()`, `get_term_info()`, `get_display()`
  - `random_term()` generation using `js_sys::Math::random()`
  - `set_strategy()` / `get_strategy()`

### JavaScript Frontend (`www/`)

- **`www/app.js`** — Main application logic:
  - `DerivationTree` class: tracks branching reduction history in JS. Each node stores a display string. Navigating re-parses via the engine.
  - `loadNewTerm()` / `performOperation()` / `navigateToNode()` — core action functions
  - Hover: only `.term.has-op` elements highlight (non-actionable terms are inert)
  - Context menu: compact floating circles with just α/β/η symbols
  - Alpha dialog: glassmorphism modal with big α icon
  - Zoom/pan: mouse wheel + drag with 4px movement threshold
  - URL params: `?term=...` is read on load and updated on every change (for linking)
  - Toolbar auto-hides, revealed by hovering near top edge

- **`www/style.css`** — Matches CIS352 playground aesthetic:
  - `#fafafa` background, `backdrop-filter: blur(12px)` glassmorphism
  - Indigo (#4f46e5) primary accent
  - Term colors: λ=indigo, bound vars=teal, free vars=orange, bindings=blue, parens=gray
  - Redex=purple highlight, eta=green highlight
  - Right drawer for strategy panel, bottom-left floating derivation tree panel

- **`www/index.html`** — SPA structure: toolbar, viewport, strategy panel, context menu, alpha dialog, derivation tree panel, status bar

## Building & Running

```bash
# Prerequisites: Rust, wasm-pack
# Install wasm-pack if needed:
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
rustup target add wasm32-unknown-unknown

# Build WASM (outputs to www/pkg/)
wasm-pack build --target web --out-dir www/pkg

# Or use the build script
./build.sh

# Serve
cd www && python3 -m http.server 8080
# Open http://localhost:8080
```

## Key Design Decisions

- **Path encoding**: Subterm paths are strings like `"LBR"` (Left, Body, Right). Empty string = root.
- **Derivation tree is JS-only**: The tree stores display strings. Navigating to a node re-parses the term via `engine.parse_and_set()`. This avoids complex state synchronization.
- **Only actionable terms highlight**: `.term.has-op` class gates all hover/click behavior. Non-reducible subterms (like the outer app in nested applications) are visually inert.
- **URL linking**: Other pages (e.g., CIS352 class website) can link with `?term=...` URL parameter.

## Planned / Future Work

- **Typed lambda calculi**: The Rust `Term` enum is designed for extensibility. Could add `src/typed.rs` with simply-typed lambda calculus, System F, etc.
- **Proof trees**: Integration with the CIS352 playground proof tree renderer
- **Better pretty-printing**: Multi-line rendering with indentation for large terms
- **Step-back animation**: Animated transitions when reducing
- **Export/share**: Copy-to-clipboard for term or derivation tree
- **Touch support**: Mobile gestures for zoom/pan

## CIS352 Syntax Reference

```
variable:     x, y, foo, x0, x'
abstraction:  (λ (x) body)           — or (\ (x) body)
application:  (e1 e2)
multi-param:  (λ (x y z) body)       — sugar for nested λ
multi-app:    (f a b c)              — sugar for left-assoc ((f a) b) c)
```

## File Map

```
machine-viz/
├── Cargo.toml          # Rust project config
├── Cargo.lock
├── build.sh            # Build script
├── src/
│   ├── lib.rs          # WASM entry point, LambdaEngine
│   ├── term.rs         # Term type, operations, strategies
│   ├── parser.rs       # CIS352 syntax parser
│   └── render.rs       # JSON render tree builder
├── www/
│   ├── index.html      # SPA HTML
│   ├── style.css       # All styles
│   ├── app.js          # JS application
│   └── pkg/            # WASM build output (gitignored)
└── CLAUDE.md           # This file
```
