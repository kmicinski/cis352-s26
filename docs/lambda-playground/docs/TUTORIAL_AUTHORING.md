# Tutorial & Interactive Lecture Authoring Guide

This document explains how to create interactive slide-based lectures using the lambda calculus explorer's tutorial system. The system is designed so that adding new slides requires only editing the `TUTORIAL_STEPS` array in `www/src/app.ts` -- no HTML, no routing, no build changes.

## Quick Start

Add a slide to the `TUTORIAL_STEPS` array at the top of `www/src/app.ts`:

```typescript
{
    title: 'My Slide Title',
    body: `<p>Explanation with <strong>formatting</strong>.</p>`,
    term: '((λ (x) x) y)',   // optional: loads this term when the slide appears
}
```

Rebuild: `npm run build` (or `npm run watch` during development).

## Slide Schema

Each slide is a `TutorialStep` object (defined in `www/src/types.ts`):

```typescript
interface TutorialStep {
    title: string;        // slide heading
    body: string;         // HTML content (rendered directly, supports full HTML)
    term?: string;        // lambda term to load (CIS352 syntax)
    autoSteps?: number;   // auto-press Space N times after loading
    strategyDemo?: boolean; // cycle through reduction strategies on a timer
    crDemo?: boolean;     // run the Church-Rosser convergence demo
}
```

### Fields

| Field | Required | Description |
|-------|----------|-------------|
| `title` | yes | Displayed as `<div class="tutorial-title">` at the top of the slide |
| `body` | yes | Raw HTML string. Use template literals for multi-line. All standard HTML works. |
| `term` | no | If provided, `loadNewTerm(term)` is called when the slide opens, resetting the derivation tree and centering the viewport. Uses CIS352 syntax (see below). |
| `autoSteps` | no | Triggers N automatic Space-key presses after the slide loads (with visual key indicators and 1.2s delay between steps). Useful for demos that build up a derivation tree. |
| `strategyDemo` | no | When `true`, cycles through Normal / Applicative / CBV / CBN strategies every 2.5 seconds, highlighting the strategy-selected redex in green. |
| `crDemo` | no | When `true`, runs the Church-Rosser demo: reduces by Normal order, then reloads the term and reduces by Applicative order, showing both paths converge to the same normal form. |

## HTML Content in Slides

The `body` field supports arbitrary HTML. Commonly used patterns:

### Text and formatting
```html
<p>Regular paragraph text.</p>
<p><strong>Bold</strong>, <em>italic</em>, <code>inline code</code>.</p>
<p class="tut-note">Tip or aside (styled with smaller font, muted color).</p>
```

### Keyboard hints
```html
<p>Press <kbd>Space</kbd> to step.</p>
<p>Hold <kbd>B</kbd> to reveal all redexes.</p>
```

### Semantic code coloring (uses CSS variables)
```html
<strong class="tut-beta">β</strong>     <!-- navy, for beta reduction -->
<strong class="tut-alpha">α</strong>    <!-- blue, for alpha conversion -->
<code class="tut-var-bound">x</code>   <!-- teal, for bound variables -->
```

### Lists
```html
<ul>
    <li><strong>Normal order</strong> (<kbd>N</kbd>) — leftmost-outermost first.</li>
    <li><strong>Call-by-Value</strong> (<kbd>V</kbd>) — arguments to values first.</li>
</ul>
```

### Anchoring demo elements
Some demos inject status into specific elements. Use a known `id`:
```html
<div id="tut-cr-status" class="tut-cr-status"></div>
<div id="tut-space-indicator" class="tut-space-indicator"></div>
```

## Term Syntax (CIS352)

Terms loaded by the `term` field use CIS352 lambda calculus syntax:

```
variable:     x, y, foo, x0, x'
abstraction:  (λ (x) body)        -- or (\ (x) body) or (lambda (x) body)
application:  (e1 e2)
multi-param:  (λ (x y z) body)    -- sugar for (λ (x) (λ (y) (λ (z) body)))
multi-app:    (f a b c)           -- sugar for (((f a) b) c)
```

In TypeScript strings, use `\u03bb` for `λ`:
```typescript
term: '((\u03bb (x) (x x)) (\u03bb (x) (x x)))'
```

## Demo Modes

### Auto-stepping (`autoSteps`)

Visually simulates the user pressing Space, with animated key indicators:

```typescript
{
    title: 'The Derivation Tree',
    body: `
        <p>Watch the derivation tree grow...</p>
        <div id="tut-space-indicator" class="tut-space-indicator"></div>
    `,
    term: '((\u03bb (x y) x) ((\u03bb (a) a) z) w)',
    autoSteps: 4,
}
```

- The `tut-space-indicator` div is where animated Space key badges appear
- After the first step, a callout arrow points to the derivation tree panel
- Steps happen 1.2 seconds apart with a 0.8 second initial delay

### Strategy cycling (`strategyDemo`)

Automatically rotates through all four reduction strategies, highlighting which redex each one selects:

```typescript
{
    title: 'Reduction Strategies',
    body: `<p>Watch the green highlight...</p>`,
    term: '((\u03bb (x) x) ((\u03bb (y) y) z))',
    strategyDemo: true,
}
```

- Adds the `show-strategy-redexes` CSS class to the term display
- All redexes get a subtle outline; the strategy-selected one gets a green highlight
- Cycles every 2.5 seconds: Normal -> Applicative -> CBV -> CBN -> repeat
- Timer is cleaned up automatically when leaving the slide

### Church-Rosser demo (`crDemo`)

Automated multi-phase demo that shows confluence:

```typescript
{
    title: 'The Church-Rosser Property',
    body: `
        <p>Watch two different strategies converge...</p>
        <div id="tut-cr-status" class="tut-cr-status"></div>
    `,
    term: '((\u03bb (x y) x) ((\u03bb (a) a) z) w)',
    crDemo: true,
}
```

Phase 1: Reduces the term to normal form using Normal order, one step at a time.
Phase 2: Reloads the same term, reduces using Applicative order. Both paths are visible in the derivation tree as a DAG. Edge labels and off-path nodes become visible to show both branches.

## Extending with New Demo Types

To add a new demo mode:

1. Add the field to `TutorialStep` in `www/src/types.ts`:
   ```typescript
   interface TutorialStep {
       // ...existing fields
       myNewDemo?: boolean;
   }
   ```

2. Add the demo logic in `showTutorialStep()` in `www/src/app.ts` (around line 1755):
   ```typescript
   if (step.myNewDemo) {
       // Your demo logic here
       // Use engine.parse_and_set(), engine.step_strategy(), etc.
       // Call renderCurrentTerm() to update the view
       // Check tutorialStep === TUTORIAL_STEPS.indexOf(step) to stop if user navigated away
   }
   ```

3. Add any needed CSS classes to `www/style.css`.

## CSS Theming

The app uses CSS custom properties for all semantic colors. Override the `:root` variables to re-theme:

```css
:root {
    /* Brand palette */
    --brand:        #F76900;   /* primary accent (Syracuse Orange) */
    --brand-dark:   #D74100;   /* hover/pressed */
    --brand-soft:   #FFB366;   /* muted accent */
    --brand-pale:   #FDDCB5;   /* light borders */
    --brand-wash:   #FFF2E5;   /* light backgrounds */

    /* Semantic colors (lambda calculus operations) */
    --sem-beta:     #000E54;   /* beta reduction */
    --sem-alpha:    #2B72D7;   /* alpha conversion */
    --sem-eta:      #059669;   /* eta conversion */
    --sem-bound:    #0d9488;   /* bound variables */
    --sem-free:     #FF431B;   /* free variables */
    --sem-binding:  #2B72D7;   /* binding site (lambda parameter) */
    --sem-result:   #f59e0b;   /* reduction result highlight */
}
```

## Navigation & Keyboard

Tutorial slides support:
- **Next/Back** buttons at the bottom of the card
- **Dot indicators** for direct navigation to any slide
- **Close button** to exit the tutorial
- While a tutorial is active, the user can still interact with the explorer (click terms, use keyboard shortcuts like `B`, `Space`, strategy keys, etc.)

## URL Deep-Linking

The `?term=...` URL parameter lets external pages (e.g., course website, lecture notes) link directly to a specific term:

```
https://your-domain/lambda/?term=((λ (x) (x x)) (λ (x) (x x)))
```

This works independently of the tutorial system. Useful for embedding links in slides, problem sets, or course notes.

## Architecture for Reuse

The tutorial system is self-contained within `www/src/app.ts`. Key functions:

| Function | Purpose |
|----------|---------|
| `showTutorialStep()` | Renders the current slide, triggers demos |
| `startTutorial()` | Opens tutorial at step 0 |
| `endTutorial()` | Cleans up and closes |
| `confirmExitTutorial()` | Gate for actions that would disrupt the tutorial |
| `loadNewTerm(input)` | Parses and loads a term, resets the tree |
| `renderCurrentTerm()` | Full re-render of the viewport |
| `navigateToNode(id)` | Jump to a derivation tree node |

The WASM engine API (available as `engine` in app.ts):

| Method | Purpose |
|--------|---------|
| `parse_and_set(term)` | Parse a term string and set it as current |
| `apply_operation(path, op, arg)` | Apply beta/eta/alpha at a subterm path |
| `step_strategy()` | Reduce one step using the current strategy |
| `set_strategy(name)` | Set reduction strategy |
| `get_display()` | Get the current term as a string |
| `get_render_tree()` | Get JSON render tree for display |
| `get_term_info()` | Get term metadata (size, free vars, redex count, etc.) |

## Example: Adding a Lecture on Church Numerals

```typescript
// Add these to the TUTORIAL_STEPS array:
{
    title: 'Church Numerals: Zero',
    body: `
        <p>In lambda calculus, we encode numbers as functions.
        <strong>Zero</strong> is: <code>(λ (f x) x)</code></p>
        <p>Zero takes a function <code>f</code> and a base <code>x</code>,
        and applies <code>f</code> zero times.</p>
    `,
    term: '(λ (f x) x)',
},
{
    title: 'Church Numerals: Successor',
    body: `
        <p><strong>Successor</strong> wraps one more application of <code>f</code>:</p>
        <p><code>(λ (n f x) (f ((n f) x)))</code></p>
        <p>Press <kbd>Space</kbd> to watch <code>(succ 2)</code> compute to <code>3</code>.</p>
    `,
    term: '((λ (n f x) (f ((n f) x))) (λ (f x) (f (f x))))',
    autoSteps: 6,
},
{
    title: 'Church Numerals: Addition',
    body: `
        <p>Can different strategies compute <code>2 + 2 = 4</code>?</p>
        <p>Watch the strategy highlight change...</p>
    `,
    term: '((λ (m n f x) ((n f) ((m f) x))) (λ (f x) (f (f x))) (λ (f x) (f (f x))))',
    strategyDemo: true,
},
```
