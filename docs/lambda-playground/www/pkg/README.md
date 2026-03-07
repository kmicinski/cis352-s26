# Lambda Calculus Explorer

An interactive web application for exploring the untyped lambda calculus. Built for **CIS352** at Syracuse University.

Enter lambda terms, click subterms to reduce them, and explore different reduction paths through a branching derivation tree.

## Features

- **Interactive reduction**: click any redex and choose beta, alpha, or eta
- **Four reduction strategies**: Normal, Applicative, Call-by-Value, Call-by-Name
- **Derivation tree**: visual DAG of all reduction paths, with branch/merge tracking
- **Self-loop detection**: dashed arrow when a term (like omega) reduces to itself
- **Binding arrows**: hover a variable to see an arrow to its binder
- **15-slide interactive tutorial**: learn lambda calculus step by step
- **40+ built-in examples**: combinators, Church numerals, Booleans, lists, recursion
- **URL deep-linking**: share terms via `?term=...` parameter
- **Keyboard-driven**: Space to step, B to reveal redexes, arrow keys to navigate

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js](https://nodejs.org/) (v18+)

## Build & Run

```bash
npm install
./build.sh                # builds WASM + TypeScript
cd www && python3 -m http.server 8080
# open http://localhost:8080
```

For development with auto-rebuild:

```bash
npm run watch             # recompiles TypeScript on save
```

## Architecture

| Layer | Tech | Source |
|-------|------|--------|
| Core logic | Rust -> WASM | `src/` (term, parser, render, WASM API) |
| Frontend | TypeScript (strict) | `www/src/app.ts`, `www/src/types.ts` |
| Bundler | esbuild | `esbuild.mjs` |
| Styles | CSS (custom properties) | `www/style.css` |

109 Rust tests (`cargo test`). TypeScript type-checks with `npm run check`.

## Building Lectures

The tutorial system supports custom interactive slide decks. Add slides to the `TUTORIAL_STEPS` array in `www/src/app.ts` -- each slide can load a term, auto-step, demo strategies, or run convergence proofs.

See [`docs/TUTORIAL_AUTHORING.md`](docs/TUTORIAL_AUTHORING.md) for the full authoring guide.

## Syntax

```
variable:     x, y, foo
abstraction:  (λ (x) body)        -- also \ or lambda
application:  (e1 e2)
multi-param:  (λ (x y z) body)    -- curried sugar
multi-app:    (f a b c)           -- left-associative sugar
```

## License

ISC
