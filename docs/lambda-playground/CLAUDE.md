# Lambda Calculus Explorer — CLAUDE.md

## Project Overview
An immersive, full-page web application for interactively exploring the untyped lambda calculus. Users can enter terms, hover/click to perform reductions, and explore different reduction paths via a branching derivation tree.

**Designed for CIS352** at Syracuse University (Prof. Micinski). Light theme, glassmorphism, JetBrains Mono fonts, Official Syracuse University colors — Orange (`#F76900`) and Navy (`#000E54`) — as CSS custom properties.

## Architecture

### Rust/WASM Core (`src/`)
All lambda calculus logic lives in Rust, compiled to WASM via `wasm-pack`:

- **`src/term.rs`** — Core `Term` enum (`Var`, `Abs`, `App`), all operations:
  - Capture-avoiding substitution
  - Beta reduction, eta conversion, alpha conversion
  - Path-based navigation (`PathStep`: `Left`/`Right`/`Body`, encoded as `L`/`R`/`B` strings)
  - `map_at_path()` for applying transformations at arbitrary subterm positions
  - Four reduction strategies: Normal, Applicative, CBV, CBN (`next_redex()` for each)
  - 56 unit tests

- **`src/parser.rs`** — Recursive descent parser for CIS352 syntax:
  - `(λ (x) body)` or `(\ (x) body)` — abstraction (supports `\`, `λ`, `lambda`)
  - `(e1 e2)` — application
  - `x` — variable
  - Sugar: multi-param `(λ (x y z) body)` desugars to nested lambdas; multi-app `(f a b)` is left-associative
  - 24 unit tests

- **`src/render.rs`** — Builds a JSON render tree for JS consumption. Each node includes:
  - `kind`, `path`, `ops` (available operations)
  - For variables: `is_free`, `binder_path` (path to the binding lambda)
  - For abs: `param`, `body`; for app: `func`, `arg`

- **`src/lib.rs`** — WASM API (`LambdaEngine` struct exported via `wasm-bindgen`):
  - `parse_and_set()`, `apply_operation()`, `step_strategy()`, `undo()`
  - `get_render_tree()`, `get_term_info()`, `get_display()`
  - `random_term()` generation using `js_sys::Math::random()`
  - `set_strategy()` / `get_strategy()`

- **`tests/integration.rs`** — 23 integration tests: full pipeline, Church-Rosser across strategies, root-redex regression, capture avoidance, omega divergence

### TypeScript Frontend (`www/src/`)

The frontend is written in **strict TypeScript** and compiled to `www/app.js` via esbuild.

- **`www/src/app.ts`** — Main application logic (strict TypeScript, ~1800 lines):
  - `DerivationTree` class: typed DAG with dedup, tracks branching reduction history
  - `RenderNode`, `TermInfo`, `DTreeNode` interfaces for WASM JSON payloads
  - `loadNewTerm()` / `performOperation()` / `navigateToNode()` — core action functions
  - Hover: only `.term.has-op` elements highlight (non-actionable terms are inert)
  - Context menu: compact floating circles with just α/β/η symbols
  - Binding arrows: SVG bezier curves from variables to binders on hover
  - Zoom/pan: mouse wheel + drag with 4px movement threshold
  - URL params: `?term=...` is read on load and updated on every change
  - 15-slide interactive tutorial with strategy demo, Church-Rosser demo, auto-stepping

- **`www/src/types.ts`** — Shared type definitions for WASM JSON shapes

- **`www/style.css`** — Syracuse brand palette with CSS custom properties

- **`www/index.html`** — SPA structure, loads compiled `app.js`

## Building & Running

```bash
# Prerequisites: Rust, wasm-pack, Node.js
npm install                              # Install TypeScript + esbuild
wasm-pack build --target web --out-dir www/pkg  # Build WASM
npm run build                            # Type-check + compile TS → www/app.js

# Or use the build script (does both)
./build.sh

# Serve
cd www && python3 -m http.server 8080
# Open http://localhost:8080

# Development
npm run check                            # Type-check only
npm run watch                            # Auto-rebuild on change
cargo test                               # Run all 109 Rust tests
```

## Key Design Decisions

- **TypeScript with strict mode**: `noUncheckedIndexedAccess`, no `any` types. WASM types come from auto-generated `.d.ts`.
- **esbuild (not webpack)**: Zero-config, sub-second builds. Bundle=true inlines `types.ts`, external for WASM import.
- **Path encoding**: Subterm paths are strings like `"LBR"` (Left, Body, Right). Empty string = root.
- **Derivation tree is TS-only**: The tree stores display strings. Navigating to a node re-parses via `engine.parse_and_set()`.
- **Self-loop edges**: When a term reduces to itself (e.g., omega), `children` gets a self-edge but `parentIds` is NOT modified. This preserves correct DAG layout (depth computation, layer assignment).
- **Only actionable terms highlight**: `.term.has-op` class gates all hover/click behavior.
- **URL linking**: Other pages can link with `?term=...` URL parameter.
- **Empty string is NOT falsy for strategy_next**: `strategy_next` is `string | null`. Root-level redexes have path `""`. Always use `== null` checks, never truthiness.
- **CSS custom properties for all colors**: Brand colors (`--brand-*`) and semantic colors (`--sem-beta`, `--sem-alpha`, `--sem-eta`, `--sem-bound`, `--sem-free`, `--sem-binding`) are centralized in `:root`. Change the palette in one place.

## Tutorial / Interactive Lecture System

The 15-slide interactive tutorial is defined as a `TutorialStep[]` array in `www/src/app.ts`. Each slide can load a term, auto-step, demo strategies, or run the Church-Rosser convergence demo.

See **[`docs/TUTORIAL_AUTHORING.md`](docs/TUTORIAL_AUTHORING.md)** for the full guide on adding slides and building lectures.

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
lambda-playground/
├── Cargo.toml          # Rust project config
├── Cargo.lock
├── package.json        # Node.js: TypeScript + esbuild
├── tsconfig.json       # TypeScript strict config
├── esbuild.mjs         # esbuild build script
├── build.sh            # Full build (WASM + TS)
├── src/
│   ├── lib.rs          # WASM entry point, LambdaEngine
│   ├── term.rs         # Term type, operations, strategies
│   ├── parser.rs       # CIS352 syntax parser
│   └── render.rs       # JSON render tree builder
├── tests/
│   └── integration.rs  # 23 integration tests
├── docs/
│   └── TUTORIAL_AUTHORING.md  # Guide: adding slides & building lectures
├── www/
│   ├── index.html      # SPA HTML
│   ├── style.css       # All styles
│   ├── app.js          # GENERATED — do not edit (from src/app.ts)
│   ├── src/
│   │   ├── app.ts      # Main TypeScript source
│   │   └── types.ts    # Shared type definitions
│   └── pkg/            # WASM build output (gitignored)
└── CLAUDE.md           # This file
```
