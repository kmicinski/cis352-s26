# CIS 352 &mdash; Rust Whirlwind Tour (Day 1 + Day 2)

A reveal.js slide deck covering Rust across two lectures for CIS 352 at
Syracuse University. Day 1 takes students from "what is Rust" through
ownership and borrowing; Day 2 covers lifetimes, traits, generics,
error handling, iterators, modules, testing, and concurrency.

## Running it

```bash
cd lecture-projects/rust-slides
npm install              # fetches reveal.js
npm run serve            # serves at http://localhost:8088
```

The deck opens at `http://localhost:8088/index.html`. Speaker notes
open with `s`, overview mode with `Esc`, fullscreen with `f`.

## Structure

```
rust-slides/
├── index.html                  # reveal.js bootstrap
├── package.json                # reveal.js dep + dev-server script
├── css/
│   └── cis352-theme.css        # Syracuse Orange + Navy reveal theme
├── slides/
│   ├── 00-title.md
│   ├── 01-day1-intro.md        # Why Rust, install, Cargo
│   ├── 02-day1-basics.md       # Variables, types, functions, control flow
│   ├── 03-day1-patterns.md     # Enums, match, Option
│   ├── 04-day1-ownership.md    # Ownership, borrowing, slices
│   ├── 05-day1-structs.md      # Structs, impl, methods, derive
│   ├── 06-day2-intro.md
│   ├── 07-day2-lifetimes.md
│   ├── 08-day2-traits-generics.md
│   ├── 09-day2-errors.md       # Result, ?, custom errors
│   ├── 10-day2-iterators.md    # Closures, iterators, collect
│   ├── 11-day2-modules-testing.md
│   ├── 12-day2-concurrency.md
│   └── 13-day2-outro.md        # Macros, unsafe, resources
└── node_modules/               # (gitignored) reveal.js + plugins
```

Each `slides/*.md` file is a separate reveal.js "section" &mdash; horizontal
progression between files, vertical progression within. Slide separators
use `\n---\n` (horizontal) and `\n--\n` (vertical). Speaker notes come
after `Note:` in each slide.

## Editing content

Everything is plain markdown with reveal.js conventions:

- `---` between slides separates horizontal slides
- `--` between slides separates vertical slides (same "track")
- `Note:` at the bottom becomes speaker notes (press `s`)
- ` ```rust ` fenced blocks get Rust syntax highlighting (Highlight.js)
- `[label](https://play.rust-lang.org/?code=...)` opens the Playground
- Add `<!-- .slide: class="title-slide" -->` etc. to tag slides for special styling

See `slides/01-day1-intro.md` for a complete example that uses most
features.

## Exporting to PDF

reveal.js has built-in PDF export. With the server running:

1. Open `http://localhost:8088/?print-pdf` in **Chrome** or **Chromium**
   (Firefox and Safari don't support reveal's PDF mode)
2. Print (`Cmd+P`) &rarr; Save as PDF
3. Set margins to "None," background graphics on

## Design notes

- **Theme**: Syracuse Orange `#F76900` accents on a Navy `#000E54` gradient
  background. Code blocks use Monokai via Highlight.js.
- **Typography**: Inter (headings + body), JetBrains Mono (code).
- **Playground links**: styled as orange pill buttons (`.playground` class
  in `css/cis352-theme.css`).
- **Callouts**: three flavors (`.callout`, `.callout.note`, `.callout.good`)
  for warnings, side-notes, and positive emphasis.
