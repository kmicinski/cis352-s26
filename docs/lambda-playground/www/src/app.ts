// ============================================================
// Lambda Calculus Explorer — Main Application (TypeScript)
// ============================================================

import init, { LambdaEngine } from '../pkg/lambda_viz.js';
import type { RenderNode, TermInfo, TutorialStep, DTreeNode, StrategyName } from './types.js';

// --- Constants ---
const EXAMPLES: Record<string, string> = {
    // Combinators
    id:      '(\u03bb (x) x)',
    K:       '(\u03bb (x y) x)',
    S:       '(\u03bb (x y z) ((x z) (y z)))',
    B:       '(\u03bb (f g x) (f (g x)))',
    C:       '(\u03bb (f x y) ((f y) x))',
    W:       '(\u03bb (f x) ((f x) x))',
    omega:   '((\u03bb (x) (x x)) (\u03bb (x) (x x)))',
    // Booleans (CBV: thunked selectors, matching CIS352 church-compile)
    true:    '(\u03bb (t f) (t (\u03bb (_) _)))',
    false:   '(\u03bb (t f) (f (\u03bb (_) _)))',
    and:     '(\u03bb (p q) ((p (\u03bb (_) q)) (\u03bb (_) (\u03bb (t f) (f (\u03bb (_) _))))))',
    or:      '(\u03bb (p q) ((p (\u03bb (_) (\u03bb (t f) (t (\u03bb (_) _))))) (\u03bb (_) q)))',
    not:     '(\u03bb (b) ((b (\u03bb (_) (\u03bb (t f) (f (\u03bb (_) _))))) (\u03bb (_) (\u03bb (t f) (t (\u03bb (_) _))))))',
    ite:     '(\u03bb (p x y) ((p x) y))',
    // Church numerals
    church0: '(\u03bb (f x) x)',
    church1: '(\u03bb (f x) (f x))',
    church2: '(\u03bb (f x) (f (f x)))',
    church3: '(\u03bb (f x) (f (f (f x))))',
    succ:    '(\u03bb (n f x) (f ((n f) x)))',
    plus:    '(\u03bb (m n f x) ((n f) ((m f) x)))',
    mult:    '(\u03bb (m n f x) ((m (n f)) x))',
    exp:     '(\u03bb (m n) (n m))',
    pred:    '(\u03bb (n f x) (((n (\u03bb (g h) (h (g f)))) (\u03bb (u) x)) (\u03bb (u) u)))',
    iszero:  '(\u03bb (n) ((n (\u03bb (_) (\u03bb (t f) (f (\u03bb (_) _))))) (\u03bb (t f) (t (\u03bb (_) _)))))',
    // Lists (matching CIS352 church-compile cons/car/cdr encoding)
    cons:    '(\u03bb (a b when_cons when_null) ((when_cons a) b))',
    car:     '(\u03bb (p) ((p (\u03bb (a b) a)) (\u03bb (_) (\u03bb (x) x))))',
    cdr:     '(\u03bb (p) ((p (\u03bb (a b) b)) (\u03bb (_) (\u03bb (x) x))))',
    null:    '(\u03bb (when_cons when_null) (when_null (\u03bb (_) _)))',
    nullq:   '(\u03bb (p) ((p (\u03bb (a b) (\u03bb (t f) (f (\u03bb (_) _))))) (\u03bb (_) (\u03bb (t f) (t (\u03bb (_) _))))))',
    // Recursion (CBV Z combinator, matching CIS352 church-compile Y-comb)
    Y:       '((\u03bb (u) (u u)) (\u03bb (y) (\u03bb (mk) (mk (\u03bb (x) (((y y) mk) x))))))',
    // Reductions
    skk:     '(((\u03bb (x y z) ((x z) (y z))) (\u03bb (x y) x)) (\u03bb (x y) x))',
    succ2:   '((\u03bb (n f x) (f ((n f) x))) (\u03bb (f x) (f (f x))))',
    twoplustwo: '((\u03bb (m n f x) ((n f) ((m f) x))) (\u03bb (f x) (f (f x))) (\u03bb (f x) (f (f x))))',
    twotimesthree: '((\u03bb (m n f x) ((m (n f)) x)) (\u03bb (f x) (f (f x))) (\u03bb (f x) (f (f (f x)))))',
    twocubed: '((\u03bb (m n) (n m)) (\u03bb (f x) (f (f x))) (\u03bb (f x) (f (f (f x)))))',
    nottrue:  '((\u03bb (b) ((b (\u03bb (_) (\u03bb (t f) (f (\u03bb (_) _))))) (\u03bb (_) (\u03bb (t f) (t (\u03bb (_) _)))))) (\u03bb (t f) (t (\u03bb (_) _))))',
    andtt:   '((\u03bb (p q) ((p (\u03bb (_) q)) (\u03bb (_) (\u03bb (t f) (f (\u03bb (_) _)))))) (\u03bb (t f) (t (\u03bb (_) _))) (\u03bb (t f) (t (\u03bb (_) _))))',
    ortf:    '((\u03bb (p q) ((p (\u03bb (_) (\u03bb (t f) (t (\u03bb (_) _))))) (\u03bb (_) q))) (\u03bb (t f) (t (\u03bb (_) _))) (\u03bb (t f) (f (\u03bb (_) _))))',
    iszero0: '((\u03bb (n) ((n (\u03bb (_) (\u03bb (t f) (f (\u03bb (_) _))))) (\u03bb (t f) (t (\u03bb (_) _))))) (\u03bb (f x) x))',
    pred3:   '((\u03bb (n f x) (((n (\u03bb (g h) (h (g f)))) (\u03bb (u) x)) (\u03bb (u) u))) (\u03bb (f x) (f (f (f x)))))',
    carcons: '((\u03bb (p) ((p (\u03bb (a b) a)) (\u03bb (_) (\u03bb (x) x)))) ((\u03bb (a b when_cons when_null) ((when_cons a) b)) (\u03bb (f x) (f x)) (\u03bb (f x) (f (f x)))))',
    factbase: '(((\u03bb (u) (u u)) (\u03bb (y) (\u03bb (mk) (mk (\u03bb (x) (((y y) mk) x)))))) (\u03bb (f n) (((\u03bb (n) ((n (\u03bb (_) (\u03bb (t f) (f (\u03bb (_) _))))) (\u03bb (t f) (t (\u03bb (_) _))))) n) (\u03bb (_) (\u03bb (f x) (f x))) (\u03bb (_) ((\u03bb (m n f x) ((m (n f)) x)) n (f ((\u03bb (n f x) (((n (\u03bb (g h) (h (g f)))) (\u03bb (u) x)) (\u03bb (u) u))) n)))))))',
};

// --- Tutorial Steps ---
const TUTORIAL_STEPS: TutorialStep[] = [
    {
        title: 'Welcome to the \u03bb-Calculus Explorer',
        body: `
            <p>This tool lets you interactively explore the <strong>untyped lambda calculus</strong> \u2014 the assembly language of functional programming.</p>
            <p>Everything in lambda calculus is built from just three things: <em>variables</em>, <em>abstractions</em> (functions), and <em>applications</em> (function calls).</p>
            <p class="tut-note">This tutorial will walk you through the basics. You can interact with the explorer at any time.</p>
        `,
    },
    {
        title: 'Variables',
        body: `
            <p>A <strong>variable</strong> is just a name, like <code>x</code> or <code>y</code>.</p>
            <p>By itself, a variable doesn't do much \u2014 it's a placeholder that will get its meaning from a surrounding function.</p>
        `,
        term: 'x',
    },
    {
        title: 'Abstractions (Functions)',
        body: `
            <p>An <strong>abstraction</strong> creates a function. We write <code>(\u03bb (x) body)</code> \u2014 this is a function that takes a parameter <code>x</code> and returns <code>body</code>.</p>
            <p>This is the <strong>identity function</strong>: it takes <code>x</code> and returns <code>x</code> unchanged.</p>
            <p class="tut-note">Hover over the term to see how the variable <code class="tut-var-bound">x</code> is <em>bound</em> by the \u03bb.</p>
        `,
        term: '(\u03bb (x) x)',
    },
    {
        title: 'Application (Calling a Function)',
        body: `
            <p>An <strong>application</strong> <code>(f x)</code> calls function <code>f</code> with argument <code>x</code>.</p>
            <p>Here we apply the identity function to <code>y</code>. The purple highlight shows this is a <strong>\u03b2-redex</strong> \u2014 a function applied to an argument, ready to reduce.</p>
            <p class="tut-note">Click the term, then click <strong class="tut-beta">\u03b2</strong> to reduce it!</p>
        `,
        term: '((\u03bb (x) x) y)',
    },
    {
        title: '\u03b2-Reduction',
        body: `
            <p><strong>\u03b2-reduction</strong> is the fundamental computation step: substitute the argument for the parameter in the body.</p>
            <p><code>((\u03bb (x) body) arg)</code> \u2192 <code>body[x := arg]</code></p>
            <p>Try pressing <kbd>Space</kbd> to step through the reduction automatically.</p>
        `,
        term: '((\u03bb (x) (x x)) y)',
    },
    {
        title: 'Multi-argument Functions',
        body: `
            <p>Lambda calculus functions take only <strong>one</strong> argument. But we can write <code>(\u03bb (x y) body)</code> as shorthand for nested functions: <code>(\u03bb (x) (\u03bb (y) body))</code>.</p>
            <p>This is called <strong>currying</strong>. The function below takes two arguments and returns the first one \u2014 this is the <strong>K combinator</strong>.</p>
        `,
        term: '(\u03bb (x y) x)',
    },
    {
        title: 'Redexes & the B Key',
        body: `
            <p>A <strong>redex</strong> (reducible expression) is any subterm of the form <code>((\u03bb (x) body) arg)</code> \u2014 a function directly applied to an argument. It's the place where computation can happen.</p>
            <p>This term has <strong>multiple redexes</strong>. Hold down <kbd>B</kbd> to reveal them all \u2014 each one lights up in <span class="tut-beta">purple</span>.</p>
            <p>Click any highlighted redex and press <strong class="tut-beta">\u03b2</strong> to reduce just that one. Which redex you choose affects the reduction path!</p>
            <p class="tut-note">Try it: hold <kbd>B</kbd>, then click different redexes to see how the result changes.</p>
        `,
        term: '((\u03bb (x) x) ((\u03bb (y) y) z))',
    },
    {
        title: 'Reduction Strategies',
        body: `
            <p>When a term has multiple redexes, which one do you reduce first? This is the <strong>reduction strategy</strong>.</p>
            <ul>
                <li><strong>Normal order</strong> (<kbd>N</kbd>) \u2014 leftmost-outermost first. Always finds the normal form if one exists.</li>
                <li><strong>Applicative</strong> (<kbd>A</kbd>) \u2014 leftmost-innermost first. Evaluates arguments before functions.</li>
                <li><strong>Call-by-Value</strong> (<kbd>V</kbd>) \u2014 like applicative, but doesn't reduce under \u03bb.</li>
                <li><strong>Call-by-Name</strong> (<kbd>L</kbd>) \u2014 like normal, but doesn't reduce under \u03bb.</li>
            </ul>
            <p>Watch the green highlight \u2014 it shows which redex each strategy picks. The dimmed redexes can't fire under this strategy.</p>
            <p class="tut-note">Try pressing <kbd>N</kbd>/<kbd>A</kbd>/<kbd>V</kbd>/<kbd>L</kbd> yourself to see the highlight jump.</p>
        `,
        term: '((\u03bb (x) x) ((\u03bb (y) y) z))',
        strategyDemo: true,
    },
    {
        title: 'The Derivation Tree',
        body: `
            <p>Watch \u2014 each <kbd>Space</kbd> press reduces one step, and the <strong>derivation tree</strong> (bottom-left) records every step.</p>
            <div id="tut-space-indicator" class="tut-space-indicator"></div>
            <p>You can click any node in the tree to jump back, use <kbd>\u2191</kbd><kbd>\u2193</kbd> to navigate, and branch by choosing different redexes.</p>
        `,
        term: '((\u03bb (x y) x) ((\u03bb (a) a) z) w)',
        autoSteps: 4,
    },
    {
        title: 'The Church-Rosser Property',
        body: `
            <p>A deep theorem: <strong>if a term has a normal form, every reduction order will reach it</strong>. This is the <em>Church-Rosser</em> property (confluence).</p>
            <p>Watch the main view \u2014 we're reducing with <strong>Normal order</strong>...</p>
            <div id="tut-cr-status" class="tut-cr-status"></div>
        `,
        term: '((\u03bb (x y) x) ((\u03bb (a) a) z) w)',
        crDemo: true,
    },
    {
        title: '\u03b1-Conversion',
        body: `
            <p><strong>\u03b1-conversion</strong> lets you rename a bound variable without changing the term's meaning. <code>(\u03bb (x) x)</code> and <code>(\u03bb (z) z)</code> are the same function.</p>
            <p>Try it: click on <code class="tut-var-bound">y</code> (or the inner \u03bb), click <strong class="tut-alpha">\u03b1</strong>, and type <code>z</code>. The rename propagates through the body.</p>
        `,
        term: '(\u03bb (x) (\u03bb (y) (x y)))',
    },
    {
        title: 'Variable Capture',
        body: `
            <p>Not all renames are safe! If you rename a variable to a name that's already used, it gets <strong>captured</strong> by a different binder \u2014 completely changing the meaning.</p>
            <p>Try it: click on the inner <code class="tut-var-bound">y</code> and try to \u03b1-rename it to <code>x</code>. The explorer will <strong>refuse</strong> and explain why: the free <code class="tut-var-bound">x</code> in the body would be captured by the outer \u03bb.</p>
            <p class="tut-note">This is one of the trickiest parts of lambda calculus. The tool always checks for you!</p>
        `,
        term: '(\u03bb (x) (\u03bb (y) (x y)))',
    },
    {
        title: 'Capture-Avoiding Substitution',
        body: `
            <p>Variable capture also matters during \u03b2-reduction. Here, naively substituting <code>y</code> for <code>x</code> in <code>(\u03bb (y) x)</code> would produce <code>(\u03bb (y) y)</code> \u2014 the wrong answer!</p>
            <p>Press <kbd>Space</kbd> to reduce. The engine automatically \u03b1-renames the inner <code class="tut-var-bound">y</code> to avoid capture, giving the correct result.</p>
            <p class="tut-note">This is called <strong>capture-avoiding substitution</strong> \u2014 the engine always renames to prevent accidental capture during \u03b2-reduction.</p>
        `,
        term: '((\u03bb (x) (\u03bb (y) x)) y)',
    },
    {
        title: 'Divergence',
        body: `
            <p>Not all terms have a normal form. The <strong>\u03a9 combinator</strong> reduces to itself forever:</p>
            <p><code>((\u03bb (x) (x x)) (\u03bb (x) (x x)))</code> \u2192 <code>((\u03bb (x) (x x)) (\u03bb (x) (x x)))</code> \u2192 ...</p>
            <p>Press <kbd>Space</kbd> to step \u2014 notice the <strong>self-loop arrow</strong> that appears next to the term. The dashed loop shows this term reduces right back to itself. This is the lambda calculus equivalent of an infinite loop.</p>
        `,
        term: '((\u03bb (x) (x x)) (\u03bb (x) (x x)))',
    },
    {
        title: 'Explore!',
        body: `
            <p>You now know the fundamentals of the lambda calculus! Here are some things to try:</p>
            <ul>
                <li>Explore the <strong>Examples</strong> menu for combinators, Booleans, arithmetic, and more</li>
                <li>Press <kbd>R</kbd> to generate a random term</li>
                <li>Type your own terms in the input bar using <code>(\u03bb (x) body)</code> syntax</li>
            </ul>
            <p class="tut-note">For more, see the <a href="https://kmicinski.com/cis352-s26/" target="_blank" rel="noopener">CIS352 course notes</a>.</p>
        `,
    },
];

// --- Derivation Tree (DAG with dedup) ---
class DerivationTree {
    nodes = new Map<number, DTreeNode>();
    displayIndex = new Map<string, number>();
    rootId: number | null = null;
    currentId: number | null = null;
    navHistory: number[] = [];
    nextId = 0;

    setRoot(displayStr: string, renderTreeJson: string | null): number {
        this.nodes.clear();
        this.displayIndex.clear();
        this.navHistory = [];
        this.nextId = 0;
        const id = this.nextId++;
        this.nodes.set(id, { id, displayStr, renderTreeJson, parentIds: [], children: [] });
        this.displayIndex.set(displayStr, id);
        this.rootId = id;
        this.currentId = id;
        return id;
    }

    addChild(displayStr: string, opLabel: string, renderTreeJson: string | null, redexPath: string | undefined): number {
        const parentId = this.currentId!;
        const parentNode = this.nodes.get(parentId)!;

        const existingId = this.displayIndex.get(displayStr);
        if (existingId != null) {
            const existing = this.nodes.get(existingId)!;
            const edgeExists = parentNode.children.some(c => c.id === existingId);
            if (!edgeExists) {
                parentNode.children.push({ id: existingId, opLabel, redexPath: redexPath || '' });
                // Don't add self-references to parentIds (self-loops are tracked via children only)
                if (existingId !== parentId && !existing.parentIds.includes(parentId)) {
                    existing.parentIds.push(parentId);
                }
            }
            if (renderTreeJson) existing.renderTreeJson = renderTreeJson;
            this.navHistory.push(this.currentId!);
            this.currentId = existingId;
            return existingId;
        }

        const id = this.nextId++;
        const node: DTreeNode = { id, displayStr, renderTreeJson, parentIds: [parentId], children: [] };
        this.nodes.set(id, node);
        this.displayIndex.set(displayStr, id);
        parentNode.children.push({ id, opLabel, redexPath: redexPath || '' });
        this.navHistory.push(this.currentId!);
        this.currentId = id;
        return id;
    }

    goTo(id: number): DTreeNode | null {
        if (this.nodes.has(id)) {
            this.navHistory.push(this.currentId!);
            this.currentId = id;
            return this.nodes.get(id)!;
        }
        return null;
    }

    undoNav(): DTreeNode | null {
        if (this.navHistory.length === 0) return null;
        const prevId = this.navHistory.pop()!;
        this.currentId = prevId;
        return this.nodes.get(prevId) ?? null;
    }

    getCurrent(): DTreeNode | null { return this.nodes.get(this.currentId!) ?? null; }

    getParent(): DTreeNode | null {
        if (this.navHistory.length === 0) return null;
        return this.nodes.get(this.navHistory[this.navHistory.length - 1]!) ?? null;
    }

    canUndo(): boolean { return this.navHistory.length > 0; }
    hasEdges(): boolean {
        if (this.nodes.size > 1) return true;
        for (const node of this.nodes.values()) {
            if (node.children.length > 0) return true;
        }
        return false;
    }

    hasBranches(): boolean {
        for (const node of this.nodes.values()) {
            if (node.children.length > 1) return true;
            if (node.parentIds.length > 1) return true;
        }
        return false;
    }
}

// --- State ---
let engine: LambdaEngine;
const dtree = new DerivationTree();
let hoveredEl: HTMLElement | null = null;
let zoom = 1, panX = 0, panY = 0;
let isDragging = false, didDrag = false;
let dragStartX = 0, dragStartY = 0, mouseDownX = 0, mouseDownY = 0;
let stepCount = 0;
let autoRunInterval: ReturnType<typeof setInterval> | null = null;
let contextMenuPath: string | null = null;
let treeCollapsed = false;
let treeNeedsAutoFit = true;

// Main tree layout
const MNODE_W = 480, MNODE_H = 64, MH_GAP = 40, MV_GAP = 80;

// --- DOM Refs ---
function $<T extends HTMLElement>(sel: string): T {
    return document.querySelector(sel) as T;
}

const termInput = $<HTMLInputElement>('#term-input');
const parseBtn = $<HTMLButtonElement>('#parse-btn');
const examplesSelect = $<HTMLSelectElement>('#examples-select');
const randomBtn = $<HTMLButtonElement>('#random-btn');
const closedCheck = $<HTMLInputElement>('#closed-check');
const undoBtn = $<HTMLButtonElement>('#undo-btn');
const viewport = $<HTMLElement>('#viewport');
const termContainer = $<HTMLElement>('#term-container');
const termDisplay = $<HTMLElement>('#term-display');
const zoomInBtn = $<HTMLButtonElement>('#zoom-in');
const zoomOutBtn = $<HTMLButtonElement>('#zoom-out');
const zoomResetBtn = $<HTMLButtonElement>('#zoom-reset');
const zoomLevel = $<HTMLElement>('#zoom-level');
const contextMenu = $<HTMLElement>('#context-menu');
const alphaPopup = $<HTMLElement>('#alpha-popup');
const alphaOld = $<HTMLElement>('#alpha-old');
const alphaNameInput = $<HTMLInputElement>('#alpha-name');
const strategyBtns = document.querySelectorAll<HTMLButtonElement>('.strategy-btn');
const stepBtn = $<HTMLButtonElement>('#step-btn');
const runBtn = $<HTMLButtonElement>('#run-btn');
const stopBtn = $<HTMLButtonElement>('#stop-btn');
const stepCountEl = $<HTMLElement>('#step-count');
const normalFormBadge = $<HTMLElement>('#normal-form-badge');
const statusSize = $<HTMLElement>('#term-size');
const statusFv = $<HTMLElement>('#term-fv');
const statusRedex = $<HTMLElement>('#redex-count');
const statusStrategy = $<HTMLElement>('#strategy-name');
const treePanel = $<HTMLElement>('#tree-panel');
const treeView = $<HTMLElement>('#tree-view');
const treeToggleBtn = $<HTMLElement>('#tree-toggle');
const drawer = $<HTMLElement>('#drawer');
const drawerToggle = $<HTMLButtonElement>('#drawer-toggle');
const drawerClose = $<HTMLButtonElement>('#drawer-close');
const drawerBackdrop = $<HTMLElement>('#drawer-backdrop');

// --- Initialization ---
async function main(): Promise<void> {
    await init();
    engine = new LambdaEngine();
    setupEventListeners();

    const params = new URLSearchParams(window.location.search);
    const urlTerm = params.get('term');
    if (urlTerm) {
        termInput.value = urlTerm;
        loadNewTerm(urlTerm);
    } else {
        termInput.value = EXAMPLES['skk']!;
        loadNewTerm(EXAMPLES['skk']!);
    }

    if (params.has('tutorial')) {
        requestAnimationFrame(startTutorial);
    } else {
        requestAnimationFrame(maybeShowWelcome);
    }
}

main().catch(console.error);

// --- Core actions ---
function loadNewTerm(input: string): void {
    try {
        engine.parse_and_set(input);
        const display = engine.get_display();
        const renderTreeJson = engine.get_render_tree();
        dtree.setRoot(display, renderTreeJson);
        stepCount = 0;
        treeNeedsAutoFit = true;
        renderCurrentTerm();
    } catch (e) {
        flashError(String(e));
    }
}

function performOperation(path: string, op: string, arg: string): void {
    try {
        engine.apply_operation(path, op, arg || '');
        const display = engine.get_display();
        const renderTreeJson = engine.get_render_tree();
        const opLabel = op === 'beta' ? '\u03b2' : op === 'eta' ? '\u03b7' : '\u03b1';
        dtree.addChild(display, opLabel, renderTreeJson, path);
        stepCount++;
        renderCurrentTerm();
    } catch (e) {
        flashError(e);
    }
}

function navigateToNode(id: number): void {
    const node = dtree.goTo(id);
    if (!node) return;
    try {
        engine.parse_and_set(node.displayStr);
        renderCurrentTerm();
    } catch (e) {
        flashError(String(e));
    }
}

function undo(): void {
    const prev = dtree.undoNav();
    if (!prev) return;
    try {
        engine.parse_and_set(prev.displayStr);
        renderCurrentTerm();
    } catch (e) { flashError(String(e)); }
    stepCount = Math.max(0, stepCount - 1);
}

function navigateTree(key: string): void {
    const cur = dtree.getCurrent();
    if (!cur) return;

    if (key === 'ArrowUp') {
        if (cur.parentIds.length > 0) navigateToNode(cur.parentIds[0]!);
    } else if (key === 'ArrowDown') {
        if (cur.children.length > 0) navigateToNode(cur.children[0]!.id);
    } else if (key === 'ArrowLeft') {
        const parent = cur.parentIds.length > 0 ? dtree.nodes.get(cur.parentIds[0]!) : undefined;
        if (parent) {
            const idx = parent.children.findIndex(c => c.id === cur.id);
            if (idx > 0) navigateToNode(parent.children[idx - 1]!.id);
            else navigateToNode(parent.id);
        }
    } else if (key === 'ArrowRight') {
        const parent = cur.parentIds.length > 0 ? dtree.nodes.get(cur.parentIds[0]!) : undefined;
        if (parent) {
            const idx = parent.children.findIndex(c => c.id === cur.id);
            if (idx < parent.children.length - 1) {
                navigateToNode(parent.children[idx + 1]!.id);
                return;
            }
        }
        if (cur.children.length > 0) navigateToNode(cur.children[0]!.id);
    }
}

// --- Drawer visibility ---
function openDrawer(): void {
    drawer.classList.add('open');
    drawerBackdrop.classList.add('visible');
}

function closeDrawer(): void {
    drawer.classList.remove('open');
    drawerBackdrop.classList.remove('visible');
}

function toggleDrawer(): void {
    if (drawer.classList.contains('open')) closeDrawer();
    else openDrawer();
}

drawerToggle.addEventListener('click', toggleDrawer);
drawerClose.addEventListener('click', closeDrawer);
drawerBackdrop.addEventListener('click', closeDrawer);

// --- Event Listeners ---
function setupEventListeners(): void {
    // Parse
    parseBtn.addEventListener('click', () => {
        if (!confirmExitTutorial()) return;
        const input = termInput.value.trim();
        if (input) loadNewTerm(input);
    });
    termInput.addEventListener('keydown', (e: KeyboardEvent) => {
        if (e.key === 'Enter') {
            if (!confirmExitTutorial()) return;
            const input = termInput.value.trim();
            if (input) loadNewTerm(input);
        }
    });
    // Examples
    examplesSelect.addEventListener('change', () => {
        if (!confirmExitTutorial()) { examplesSelect.value = ''; return; }
        const val = examplesSelect.value;
        if (val && EXAMPLES[val]) {
            termInput.value = EXAMPLES[val]!;
            loadNewTerm(EXAMPLES[val]!);
        }
        examplesSelect.value = '';
    });

    // Random
    randomBtn.addEventListener('click', () => {
        if (!confirmExitTutorial()) return;
        engine.random_term(closedCheck.checked);
        const display = engine.get_display();
        const renderTreeJson = engine.get_render_tree();
        dtree.setRoot(display, renderTreeJson);
        stepCount = 0;
        treeNeedsAutoFit = true;
        renderCurrentTerm();
        termInput.value = display;
    });

    // Undo
    undoBtn.addEventListener('click', undo);

    // Zoom
    zoomInBtn.addEventListener('click', () => setZoom(zoom * 1.2));
    zoomOutBtn.addEventListener('click', () => setZoom(zoom / 1.2));
    zoomResetBtn.addEventListener('click', () => { zoom = 1; panX = 0; panY = 0; updateTransform(); });
    viewport.addEventListener('wheel', onWheel, { passive: false });

    // Pan
    viewport.addEventListener('mousedown', onMouseDown);
    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);

    // Term interaction
    termDisplay.addEventListener('mousemove', onTermHover);
    termDisplay.addEventListener('mouseleave', clearHover);
    termDisplay.addEventListener('click', onTermClick);

    // Context menu dismiss
    document.addEventListener('click', (e: MouseEvent) => {
        if (!contextMenu.contains(e.target as Node)) hideContextMenu();
    });
    document.addEventListener('keydown', (e: KeyboardEvent) => {
        if (e.key === 'Escape') {
            hideContextMenu();
            hideAlphaPopup();
            closeDrawer();
        }
        if (e.key === 'Tab' && document.activeElement !== termInput && document.activeElement !== alphaNameInput) {
            e.preventDefault();
            toggleDrawer();
        }
        if (e.key === 'z' && (e.metaKey || e.ctrlKey)) {
            e.preventDefault();
            undo();
        }
        const notTyping = document.activeElement !== termInput && document.activeElement !== alphaNameInput;
        if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key) && notTyping) {
            e.preventDefault();
            if (tutorialActive()) {
                if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
                    if (tutorialStep < TUTORIAL_STEPS.length - 1) { tutorialStep++; showTutorialStep(); }
                } else {
                    if (tutorialStep > 0) { tutorialStep--; showTutorialStep(); }
                }
            } else {
                navigateTree(e.key);
            }
        }
        if (!e.metaKey && !e.ctrlKey && !e.altKey && notTyping) {
            if (e.key === 'b') termDisplay.classList.add('show-redexes');
            if (e.key === 'c') centerOnCurrent(true);
            if (e.key === ' ') {
                e.preventDefault();
                if (stepBtn.disabled) {
                    try {
                        const info: TermInfo = JSON.parse(engine.get_term_info());
                        const strat = engine.get_strategy();
                        const stratNames: Record<string, string> = { normal: 'normal order', applicative: 'applicative order', cbv: 'call-by-value', cbn: 'call-by-name' };
                        if (info.redex_count > 0) {
                            flashInfo(`${stratNames[strat] || strat} normal form \u2014 ${info.redex_count} redex${info.redex_count > 1 ? 'es' : ''} remain under \u03bb`);
                        } else {
                            flashInfo('Normal form \u2014 no more reductions');
                        }
                    } catch (_) {
                        flashInfo('Normal form \u2014 no more reductions');
                    }
                } else {
                    stepBtn.click();
                }
            }
            if (e.key === 'r') randomBtn.click();
            const stratKeys: Record<string, StrategyName> = { n: 'normal', a: 'applicative', v: 'cbv', l: 'cbn' };
            if (stratKeys[e.key]) setStrategy(stratKeys[e.key]!);
        }
    });
    document.addEventListener('keyup', (e: KeyboardEvent) => {
        if (e.key === 'b') {
            termDisplay.classList.remove('show-redexes');
        }
    });

    // Alpha popup
    alphaNameInput.addEventListener('keydown', (e: KeyboardEvent) => {
        if (e.key === 'Enter') { e.preventDefault(); applyAlpha(); }
        if (e.key === 'Escape') hideAlphaPopup();
        e.stopPropagation();
    });
    alphaNameInput.addEventListener('keyup', (e: Event) => e.stopPropagation());
    document.addEventListener('click', (e: MouseEvent) => {
        if (!alphaPopup.contains(e.target as Node)) hideAlphaPopup();
    });

    // Strategy buttons
    strategyBtns.forEach(btn => {
        btn.addEventListener('click', () => setStrategy(btn.dataset['strategy'] as StrategyName));
    });
    stepBtn.addEventListener('click', () => {
        try {
            const preInfo: TermInfo = JSON.parse(engine.get_term_info());
            const redexPath = preInfo.strategy_next || '';
            engine.step_strategy();
            const display = engine.get_display();
            const renderTreeJson = engine.get_render_tree();
            const strat = engine.get_strategy();
            const labels: Record<string, string> = { normal: '\u03b2', applicative: '\u03b2', cbv: '\u03b2', cbn: '\u03b2' };
            dtree.addChild(display, labels[strat] || '\u03b2', renderTreeJson, redexPath);
            stepCount++;
            renderCurrentTerm();
        } catch (e) {
            flashError(String(e));
        }
    });
    runBtn.addEventListener('click', startAutoRun);
    stopBtn.addEventListener('click', stopAutoRun);

    // Tree panel
    if (treeToggleBtn) {
        treeToggleBtn.addEventListener('click', () => {
            treeCollapsed = !treeCollapsed;
            treePanel.classList.toggle('collapsed', treeCollapsed);
        });
    }
}

// --- Render ---
function renderCurrentTerm(): void {
    let treeJson: string, infoJson: string;
    try {
        treeJson = engine.get_render_tree();
        infoJson = engine.get_term_info();
    } catch (_) {
        termDisplay.innerHTML = '<div id="empty-state"><div class="empty-icon">\u03bb</div><p>Enter a term above or choose an example to begin</p></div>';
        termDisplay.style.width = '';
        termDisplay.style.height = '';
        updateStatusBar(null);
        return;
    }

    const renderTree: RenderNode = JSON.parse(treeJson);
    const info: TermInfo = JSON.parse(infoJson);

    const currentDNode = dtree.getCurrent();
    if (currentDNode) currentDNode.renderTreeJson = treeJson;

    const pad = 60;
    const activePath = getActivePath();

    // Redex highlights: which subterm in parent was reduced
    const childHighlights = new Map<number, string>();
    for (const node of dtree.nodes.values()) {
        for (const child of node.children) {
            if (child.id === dtree.currentId && child.redexPath) {
                childHighlights.set(node.id, child.redexPath);
            }
        }
    }

    // Pass 1: Create cards at temp positions for measurement
    termDisplay.innerHTML = '';
    const cardMap = new Map<number, HTMLElement>();
    const ppCharWidth = 12;
    const ppMaxCols = Math.floor(viewport.getBoundingClientRect().width / ppCharWidth);

    for (const [nodeId, dnode] of dtree.nodes) {
        const isCurrent = nodeId === dtree.currentId;
        const onPath = activePath.has(nodeId);

        const card = document.createElement('div');
        card.className = 'main-node-card';
        if (isCurrent) card.classList.add('current');
        else if (onPath) card.classList.add('on-path');
        else card.classList.add('off-path');

        card.style.left = '0px';
        card.style.top = '0px';

        if (isCurrent) {
            card.id = 'current-term-card';
            const rendered = renderNode(renderTree, ppMaxCols, 0);
            rendered.classList.add('term-animate');
            card.appendChild(rendered);
        } else {
            const highlightPath = childHighlights.get(nodeId) || null;
            if (dnode.renderTreeJson) {
                try {
                    const staticTree: RenderNode = JSON.parse(dnode.renderTreeJson);
                    card.appendChild(renderNodeStatic(staticTree, highlightPath, ppMaxCols, 0));
                } catch (_) {
                    card.textContent = truncateDisplay(dnode.displayStr, 50);
                }
            } else {
                card.textContent = truncateDisplay(dnode.displayStr, 50);
            }
            card.addEventListener('click', (e: MouseEvent) => {
                e.stopPropagation();
                navigateToNode(nodeId);
            });
        }
        termDisplay.appendChild(card);
        cardMap.set(nodeId, card);
    }

    // Pass 2: Measure actual dimensions
    const measuredW = new Map<number, number>();
    const measuredH = new Map<number, number>();
    for (const [nodeId, card] of cardMap) {
        measuredW.set(nodeId, card.offsetWidth);
        measuredH.set(nodeId, card.offsetHeight);
    }

    // Pass 3: Layered DAG layout
    const depths = new Map<number, number>();
    function computeDepth(nodeId: number, visiting: Set<number>): number {
        if (depths.has(nodeId)) return depths.get(nodeId)!;
        if (visiting.has(nodeId)) return 0;
        visiting.add(nodeId);
        const node = dtree.nodes.get(nodeId)!;
        if (node.parentIds.length === 0) { depths.set(nodeId, 0); return 0; }
        let maxD = 0;
        for (const pid of node.parentIds) maxD = Math.max(maxD, computeDepth(pid, visiting));
        const d = maxD + 1;
        depths.set(nodeId, d);
        return d;
    }
    for (const id of dtree.nodes.keys()) computeDepth(id, new Set());

    const layers: number[][] = [];
    for (const [id, d] of depths) {
        while (layers.length <= d) layers.push([]);
        layers[d]!.push(id);
    }

    const xPositions = new Map<number, number>();
    if (layers.length > 0) {
        {
            const ids = layers[0]!;
            const totalW = ids.reduce((s, id) => s + (measuredW.get(id) || MNODE_W), 0) + (ids.length - 1) * MH_GAP;
            let x = pad;
            for (const id of ids) {
                const w = measuredW.get(id) || MNODE_W;
                xPositions.set(id, x + w / 2);
                x += w + MH_GAP;
            }
            if (ids.length === 1) {
                xPositions.set(ids[0]!, totalW / 2 + pad);
            }
        }

        for (let d = 1; d < layers.length; d++) {
            const ids = layers[d]!;
            const desired = new Map<number, number>();
            for (const id of ids) {
                const node = dtree.nodes.get(id)!;
                const parentXs = node.parentIds.map(pid => xPositions.get(pid)).filter((x): x is number => x != null);
                desired.set(id, parentXs.length > 0 ? parentXs.reduce((a, b) => a + b, 0) / parentXs.length : pad);
            }
            ids.sort((a, b) => desired.get(a)! - desired.get(b)!);
            let nextLeft = pad;
            for (const id of ids) {
                const w = measuredW.get(id) || MNODE_W;
                const wantedLeft = desired.get(id)! - w / 2;
                const actualLeft = Math.max(nextLeft, wantedLeft);
                xPositions.set(id, actualLeft + w / 2);
                nextLeft = actualLeft + w + MH_GAP;
            }
        }
    }

    const yPositions = new Map<number, number>();
    let yAccum = pad;
    for (let d = 0; d < layers.length; d++) {
        const layerIds = layers[d]!;
        if (layerIds.length === 0) continue;
        const maxH = Math.max(...layerIds.map(id => measuredH.get(id) || MNODE_H));
        for (const id of layerIds) yPositions.set(id, yAccum);
        yAccum += maxH + MV_GAP;
    }

    // Apply final positions
    for (const [nodeId, card] of cardMap) {
        card.style.left = xPositions.get(nodeId) + 'px';
        card.style.top = yPositions.get(nodeId) + 'px';
    }

    // Compute bounds
    let maxX = 0, maxY = 0;
    for (const [nodeId] of cardMap) {
        const cx = xPositions.get(nodeId)!;
        const w = cardMap.get(nodeId)!.offsetWidth;
        const h = measuredH.get(nodeId) || MNODE_H;
        const y = yPositions.get(nodeId)!;
        maxX = Math.max(maxX, cx + w / 2 + pad);
        maxY = Math.max(maxY, y + h + pad);
    }

    // Draw SVG edges
    const NS = 'http://www.w3.org/2000/svg';
    const svg = document.createElementNS(NS, 'svg');
    svg.classList.add('main-edges');
    svg.setAttribute('width', String(maxX));
    svg.setAttribute('height', String(maxY));

    const defs = document.createElementNS(NS, 'defs');
    for (const [id, color] of [['main-arrow', '#FDDCB5'], ['main-arrow-active', '#F76900']] as const) {
        const marker = document.createElementNS(NS, 'marker');
        marker.setAttribute('id', id);
        marker.setAttribute('viewBox', '0 0 10 10');
        marker.setAttribute('refX', '9');
        marker.setAttribute('refY', '5');
        marker.setAttribute('markerWidth', '6');
        marker.setAttribute('markerHeight', '6');
        marker.setAttribute('orient', 'auto');
        const arrow = document.createElementNS(NS, 'path');
        arrow.setAttribute('d', 'M1,2 L9,5 L1,8 Z');
        arrow.setAttribute('fill', color);
        marker.appendChild(arrow);
        defs.appendChild(marker);
    }
    svg.appendChild(defs);

    for (const node of dtree.nodes.values()) {
        if (!xPositions.has(node.id)) continue;
        const parentCx = xPositions.get(node.id)!;
        const parentY = yPositions.get(node.id)!;
        const parentH = measuredH.get(node.id) || MNODE_H;
        const parentW = measuredW.get(node.id) || MNODE_W;

        for (const child of node.children) {
            if (!xPositions.has(child.id)) continue;
            const active = activePath.has(node.id) && activePath.has(child.id);

            // Self-loop
            if (child.id === node.id) {
                const loopR = 50;
                const x0 = parentCx + parentW / 2;
                const y0 = parentY + parentH / 2;
                const path = document.createElementNS(NS, 'path');
                path.setAttribute('d',
                    `M${x0},${y0 + parentH / 2 - 4}` +
                    ` C${x0 + loopR + 20},${y0 + parentH / 2 + loopR}` +
                    ` ${x0 + loopR + 20},${y0 - parentH / 2 - loopR}` +
                    ` ${x0},${y0 - parentH / 2 + 4}`
                );
                path.setAttribute('stroke', '#F76900');
                path.setAttribute('stroke-width', '2.5');
                path.setAttribute('fill', 'none');
                path.setAttribute('stroke-dasharray', '6,3');
                path.setAttribute('marker-end', 'url(#main-arrow-active)');
                svg.appendChild(path);

                if (child.opLabel) {
                    const text = document.createElementNS(NS, 'text');
                    text.setAttribute('x', String(x0 + loopR + 26));
                    text.setAttribute('y', String(y0 + 5));
                    text.setAttribute('text-anchor', 'start');
                    text.setAttribute('class', 'main-self-loop-label');
                    text.setAttribute('fill', '#F76900');
                    text.textContent = child.opLabel + ' \u21bb';
                    svg.appendChild(text);
                }

                maxX = Math.max(maxX, x0 + loopR + 80);
                continue;
            }

            const childCx = xPositions.get(child.id)!;
            const childY = yPositions.get(child.id)!;

            const x1 = parentCx, y1 = parentY + parentH;
            const x2 = childCx, y2 = childY;
            const midY = (y1 + y2) / 2;

            const path = document.createElementNS(NS, 'path');
            path.setAttribute('d', `M${x1},${y1} C${x1},${midY} ${x2},${midY} ${x2},${y2}`);
            const allBranches = termDisplay.classList.contains('show-all-branches');
            const edgeVisible = active || allBranches;
            path.setAttribute('stroke', active ? '#F76900' : allBranches ? '#FDDCB5' : '#FFF2E5');
            path.setAttribute('stroke-width', edgeVisible ? '2.5' : '2');
            path.setAttribute('fill', 'none');
            path.setAttribute('marker-end', `url(#${active ? 'main-arrow-active' : 'main-arrow'})`);
            svg.appendChild(path);

            if (child.opLabel) {
                const text = document.createElementNS(NS, 'text');
                const lx = (x1 + x2) / 2 + (x1 === x2 ? 16 : 0);
                const ly = midY + 5;
                text.setAttribute('x', String(lx));
                text.setAttribute('y', String(ly));
                text.setAttribute('text-anchor', 'middle');
                text.setAttribute('class', 'main-edge-label' + (active ? ' active' : ''));
                text.textContent = child.opLabel;
                svg.appendChild(text);
            }
        }
    }

    svg.setAttribute('width', String(maxX));
    svg.setAttribute('height', String(maxY));

    termDisplay.prepend(svg);
    termDisplay.style.width = maxX + 'px';
    termDisplay.style.height = maxY + 'px';

    // Highlight strategy next redex
    const currentCard = document.getElementById('current-term-card');
    if (info.strategy_next != null && currentCard) {
        const nextEl = currentCard.querySelector(`[data-path="${info.strategy_next}"]`);
        if (nextEl) nextEl.classList.add('strategy-next');
    }

    // Highlight reduction result
    const parentNode = dtree.getParent();
    if (parentNode && currentCard) {
        const edge = parentNode.children.find(c => c.id === dtree.currentId);
        if (edge && edge.redexPath) {
            const resultEl = currentCard.querySelector(`[data-path="${edge.redexPath}"]`);
            if (resultEl) resultEl.classList.add('reduction-result');
        }
    }

    undoBtn.disabled = !dtree.canUndo();
    updateStatusBar(info);
    updateStepInfo(info);
    termInput.value = info.display;

    // Update URL
    const url = new URL(window.location.href);
    url.searchParams.set('term', info.display);
    window.history.replaceState({}, '', url.toString());

    // Center viewport
    if (treeNeedsAutoFit && dtree.rootId != null) {
        treeNeedsAutoFit = false;
        zoom = 1;
        requestAnimationFrame(() => centerOnCurrent(false));
    } else {
        requestAnimationFrame(() => centerOnCurrent(true));
    }

    renderDerivationTree();
}

// --- Pretty-printing ---
function flatWidth(node: RenderNode): number {
    if (node._fw != null) return node._fw;
    switch (node.kind) {
        case 'var': node._fw = (node.name ?? '').length; break;
        case 'abs': node._fw = 7 + (node.param ?? '').length + flatWidth(node.body!); break;
        case 'app': node._fw = 3 + flatWidth(node.func!) + flatWidth(node.arg!); break;
        default: node._fw = 0;
    }
    return node._fw;
}

function ppBreak(col: number): DocumentFragment {
    const frag = document.createDocumentFragment();
    frag.appendChild(document.createElement('br'));
    if (col > 0) {
        const s = document.createElement('span');
        s.className = 'pp-indent';
        s.style.width = col + 'ch';
        frag.appendChild(s);
    }
    return frag;
}

function renderNode(node: RenderNode, maxCols: number, col: number): HTMLSpanElement {
    const span = document.createElement('span');
    span.className = `term term-${node.kind}`;
    span.dataset['path'] = node.path;

    if (node.ops && node.ops.length > 0) {
        span.classList.add('has-op');
        span.dataset['ops'] = JSON.stringify(node.ops);
    }

    const fw = flatWidth(node);
    const fits = fw <= maxCols - col;

    switch (node.kind) {
        case 'var': {
            const v = document.createElement('span');
            v.className = 'var-ref' + (node.is_free ? ' free' : '');
            v.textContent = node.name ?? '';
            if (node.binder_path != null) v.dataset['binderPath'] = node.binder_path;
            v.dataset['varName'] = node.name ?? '';
            span.appendChild(v);
            break;
        }
        case 'abs': {
            if (node.ops.includes('eta')) span.classList.add('eta-convertible');
            const headerW = 6 + (node.param ?? '').length;
            span.appendChild(paren('('));
            span.appendChild(lambdaSym());
            span.appendChild(mkText('\u00a0'));
            span.appendChild(paren('('));
            const param = document.createElement('span');
            param.className = 'var-binding';
            param.textContent = node.param ?? '';
            param.dataset['absPath'] = node.path;
            param.dataset['varName'] = node.param ?? '';
            span.appendChild(param);
            span.appendChild(paren(')'));
            if (fits) {
                span.appendChild(mkText(' '));
                span.appendChild(renderNode(node.body!, maxCols, col + headerW + 1));
            } else {
                span.appendChild(ppBreak(col + 2));
                span.appendChild(renderNode(node.body!, maxCols, col + 2));
            }
            span.appendChild(paren(')'));
            break;
        }
        case 'app': {
            if (node.ops.includes('beta')) span.classList.add('redex');
            span.appendChild(paren('('));
            if (fits) {
                span.appendChild(renderNode(node.func!, maxCols, col + 1));
                span.appendChild(mkText(' '));
                span.appendChild(renderNode(node.arg!, maxCols, col + 1 + flatWidth(node.func!) + 1));
            } else {
                span.appendChild(renderNode(node.func!, maxCols, col + 1));
                span.appendChild(ppBreak(col + 1));
                span.appendChild(renderNode(node.arg!, maxCols, col + 1));
            }
            span.appendChild(paren(')'));
            break;
        }
    }
    return span;
}

function paren(ch: string): HTMLSpanElement {
    const s = document.createElement('span');
    s.className = 'paren';
    s.textContent = ch;
    return s;
}

function lambdaSym(): HTMLSpanElement {
    const s = document.createElement('span');
    s.className = 'lambda-sym';
    s.textContent = '\u03bb';
    return s;
}

function mkText(t: string): Text { return document.createTextNode(t); }

function renderNodeStatic(node: RenderNode, highlightPath: string | null, maxCols: number, col: number): HTMLSpanElement {
    const span = document.createElement('span');
    span.className = `term term-${node.kind}`;
    if (highlightPath != null && node.path === highlightPath) {
        span.classList.add('redex-highlight');
    }

    const fw = flatWidth(node);
    const fits = fw <= maxCols - col;

    switch (node.kind) {
        case 'var': {
            const v = document.createElement('span');
            v.className = 'var-ref' + (node.is_free ? ' free' : '');
            v.textContent = node.name ?? '';
            span.appendChild(v);
            break;
        }
        case 'abs': {
            if (node.ops && node.ops.includes('eta')) span.classList.add('eta-convertible');
            const headerW = 6 + (node.param ?? '').length;
            span.appendChild(paren('('));
            span.appendChild(lambdaSym());
            span.appendChild(mkText('\u00a0'));
            span.appendChild(paren('('));
            const param = document.createElement('span');
            param.className = 'var-binding';
            param.textContent = node.param ?? '';
            span.appendChild(param);
            span.appendChild(paren(')'));
            if (fits) {
                span.appendChild(mkText(' '));
                span.appendChild(renderNodeStatic(node.body!, highlightPath, maxCols, col + headerW + 1));
            } else {
                span.appendChild(ppBreak(col + 2));
                span.appendChild(renderNodeStatic(node.body!, highlightPath, maxCols, col + 2));
            }
            span.appendChild(paren(')'));
            break;
        }
        case 'app': {
            if (node.ops && node.ops.includes('beta')) span.classList.add('redex');
            span.appendChild(paren('('));
            if (fits) {
                span.appendChild(renderNodeStatic(node.func!, highlightPath, maxCols, col + 1));
                span.appendChild(mkText(' '));
                span.appendChild(renderNodeStatic(node.arg!, highlightPath, maxCols, col + 1 + flatWidth(node.func!) + 1));
            } else {
                span.appendChild(renderNodeStatic(node.func!, highlightPath, maxCols, col + 1));
                span.appendChild(ppBreak(col + 1));
                span.appendChild(renderNodeStatic(node.arg!, highlightPath, maxCols, col + 1));
            }
            span.appendChild(paren(')'));
            break;
        }
    }
    return span;
}

// --- Hover ---
function onTermHover(e: MouseEvent): void {
    const target = (e.target as HTMLElement).closest<HTMLElement>('.term.has-op');
    if (target === hoveredEl) return;

    clearHover();
    hoveredEl = target;
    if (!hoveredEl) return;

    hoveredEl.classList.add('hovered');

    const varRef = (e.target as HTMLElement).closest<HTMLElement>('.var-ref');
    if (varRef && varRef.dataset['binderPath'] != null) {
        highlightBinding(varRef.dataset['binderPath']!, varRef);
    }
    const varBinding = (e.target as HTMLElement).closest<HTMLElement>('.var-binding');
    if (varBinding && varBinding.dataset['absPath']) {
        highlightBinding(varBinding.dataset['absPath']!, varBinding);
    }
}

function clearHover(): void {
    if (hoveredEl) {
        hoveredEl.classList.remove('hovered');
        hoveredEl = null;
    }
    const scope = document.getElementById('current-term-card') || termDisplay;
    scope.querySelectorAll('.binding-highlight').forEach(el => {
        el.classList.remove('binding-highlight');
    });
    removeBindingArrow();
}

function highlightBinding(absPath: string, hoveredElement: HTMLElement): void {
    const scope = document.getElementById('current-term-card') || termDisplay;
    scope.querySelectorAll(`.var-binding[data-abs-path="${absPath}"]`).forEach(el => {
        el.classList.add('binding-highlight');
    });
    scope.querySelectorAll(`.var-ref[data-binder-path="${absPath}"]`).forEach(el => {
        el.classList.add('binding-highlight');
    });
    drawBindingArrow(absPath, hoveredElement);
}

// --- Binding Arrow Overlay ---
function drawBindingArrow(absPath: string, hoveredElement: HTMLElement): void {
    removeBindingArrow();
    const card = document.getElementById('current-term-card');
    if (!card) return;

    const binder = card.querySelector<HTMLElement>(`.var-binding[data-abs-path="${absPath}"]`);
    if (!binder) return;
    const refs = card.querySelectorAll<HTMLElement>(`.var-ref[data-binder-path="${absPath}"]`);
    if (refs.length === 0) return;

    const hoveredIsRef = hoveredElement.classList.contains('var-ref');

    const cardRect = card.getBoundingClientRect();
    const NS = 'http://www.w3.org/2000/svg';
    const svg = document.createElementNS(NS, 'svg');
    svg.id = 'binding-arrow-svg';
    svg.classList.add('binding-arrow-overlay');
    svg.style.position = 'absolute';
    svg.style.top = '0';
    svg.style.left = '0';
    svg.style.width = card.scrollWidth + 'px';
    svg.style.height = card.scrollHeight + 'px';
    svg.style.pointerEvents = 'none';
    svg.style.overflow = 'visible';
    svg.style.zIndex = '10';

    const defs = document.createElementNS(NS, 'defs');
    const marker = document.createElementNS(NS, 'marker');
    marker.setAttribute('id', 'binding-arrow-head');
    marker.setAttribute('viewBox', '0 0 8 6');
    marker.setAttribute('refX', '7');
    marker.setAttribute('refY', '3');
    marker.setAttribute('markerWidth', '8');
    marker.setAttribute('markerHeight', '6');
    marker.setAttribute('orient', 'auto');
    const arrowPath = document.createElementNS(NS, 'path');
    arrowPath.setAttribute('d', 'M0,0.5 L7,3 L0,5.5 Z');
    arrowPath.setAttribute('fill', 'rgba(43, 114, 215, 0.55)');
    marker.appendChild(arrowPath);
    defs.appendChild(marker);
    svg.appendChild(defs);

    const binderRect = binder.getBoundingClientRect();
    const bx = binderRect.left + binderRect.width / 2 - cardRect.left;
    const bBinderBot = binderRect.bottom - cardRect.top;

    const targets = hoveredIsRef ? [hoveredElement] : Array.from(refs);

    targets.forEach(ref => {
        const refRect = ref.getBoundingClientRect();
        const rx = refRect.left + refRect.width / 2 - cardRect.left;
        const rBot = refRect.bottom - cardRect.top;

        if (Math.abs(rx - bx) < 2 && Math.abs(rBot - bBinderBot) < 4) return;

        let startX: number, startY: number, endX: number, endY: number;
        if (hoveredIsRef) {
            startX = rx; startY = rBot;
            endX = bx; endY = bBinderBot;
        } else {
            startX = bx; startY = bBinderBot;
            endX = rx; endY = rBot;
        }

        const hDist = Math.abs(endX - startX);
        if (hDist < 1) return;

        const sag = Math.max(10, Math.min(hDist * 0.18, 32));
        const baseY = Math.max(startY, endY);

        const cx1 = startX + (endX - startX) * 0.25;
        const cy1 = baseY + sag;
        const cx2 = startX + (endX - startX) * 0.75;
        const cy2 = baseY + sag;

        const path = document.createElementNS(NS, 'path');
        path.setAttribute('d', `M${startX},${startY} C${cx1},${cy1} ${cx2},${cy2} ${endX},${endY}`);
        path.setAttribute('stroke', 'rgba(43, 114, 215, 0.45)');
        path.setAttribute('stroke-width', '1.8');
        path.setAttribute('fill', 'none');
        path.setAttribute('marker-end', 'url(#binding-arrow-head)');
        path.classList.add('binding-arrow-path');
        svg.appendChild(path);
    });

    if (svg.querySelectorAll('.binding-arrow-path').length > 0) {
        card.appendChild(svg);
    }
}

function removeBindingArrow(): void {
    const existing = document.getElementById('binding-arrow-svg');
    if (existing) existing.remove();
}

// --- Click / Context Menu ---
function onTermClick(e: MouseEvent): void {
    if (didDrag) return;
    hideContextMenu();

    const target = (e.target as HTMLElement).closest<HTMLElement>('.term.has-op');
    if (!target) return;
    e.stopPropagation();

    const ops: string[] = target.dataset['ops'] ? JSON.parse(target.dataset['ops']) : [];
    if (ops.length === 0) return;

    contextMenuPath = target.dataset['path'] ?? null;

    const varRef = (e.target as HTMLElement).closest<HTMLElement>('.var-ref');
    if (varRef && varRef.dataset['binderPath'] != null) {
        contextMenu.dataset['alphaTarget'] = varRef.dataset['binderPath'];
    } else {
        contextMenu.dataset['alphaTarget'] = target.dataset['path'] ?? '';
    }

    showContextMenu(e.clientX, e.clientY, ops);
}

function showContextMenu(x: number, y: number, ops: string[]): void {
    contextMenu.innerHTML = '';

    for (const op of ops) {
        const btn = document.createElement('button');
        btn.className = `ctx-btn ${op}`;
        btn.textContent = op === 'beta' ? '\u03b2' : op === 'eta' ? '\u03b7' : '\u03b1';
        btn.addEventListener('click', (e: MouseEvent) => {
            e.stopPropagation();
            const path = contextMenuPath;
            const alphaTarget = contextMenu.dataset['alphaTarget'] ?? '';
            const rect = contextMenu.getBoundingClientRect();
            const ax = rect.left + rect.width / 2;
            const ay = rect.top;
            hideContextMenu();
            handleOperation(op, path!, alphaTarget, ax, ay);
        });
        contextMenu.appendChild(btn);
    }

    contextMenu.classList.remove('hidden');
    const rect = contextMenu.getBoundingClientRect();
    const vw = window.innerWidth;
    let fx = x - rect.width / 2;
    let fy = y - rect.height - 12;
    if (fx < 8) fx = 8;
    if (fx + rect.width > vw - 8) fx = vw - rect.width - 8;
    if (fy < 8) fy = y + 16;
    contextMenu.style.left = fx + 'px';
    contextMenu.style.top = fy + 'px';
}

function hideContextMenu(): void {
    contextMenu.classList.add('hidden');
    contextMenuPath = null;
}

function handleOperation(op: string, path: string, alphaTarget: string, anchorX: number, anchorY: number): void {
    if (op === 'alpha') {
        showAlphaPopup(alphaTarget || path, anchorX || 0, anchorY || 0);
        return;
    }
    performOperation(path, op, '');
}

// --- Inline Alpha Rename ---
function showAlphaPopup(target: string, anchorX: number, anchorY: number): void {
    alphaNameInput.value = '';
    alphaPopup.dataset['targetPath'] = target;

    const scope = document.getElementById('current-term-card') || termDisplay;
    const absEl = scope.querySelector<HTMLElement>(`[data-path="${target}"]`);
    const varName = absEl ? (absEl.querySelector('.var-binding')?.textContent || '?') : '?';
    alphaOld.textContent = varName;

    alphaPopup.classList.remove('hidden');

    const pw = alphaPopup.offsetWidth;
    const vw = window.innerWidth;
    let fx = anchorX - pw / 2;
    let fy = anchorY - alphaPopup.offsetHeight - 10;
    if (fx < 8) fx = 8;
    if (fx + pw > vw - 8) fx = vw - pw - 8;
    if (fy < 8) fy = anchorY + 16;
    alphaPopup.style.left = fx + 'px';
    alphaPopup.style.top = fy + 'px';

    requestAnimationFrame(() => alphaNameInput.focus());
}

function hideAlphaPopup(): void {
    alphaPopup.classList.add('hidden');
}

function applyAlpha(): void {
    const newName = alphaNameInput.value.trim();
    if (!newName) return;
    const target = alphaPopup.dataset['targetPath'] ?? '';
    hideAlphaPopup();
    performOperation(target, 'alpha', newName);
}

// --- Derivation Tree View ---
function getActivePath(): Set<number> {
    const path = new Set<number>();
    const queue: (number | null)[] = [dtree.currentId];
    while (queue.length > 0) {
        const id = queue.shift();
        if (id == null || path.has(id)) continue;
        path.add(id);
        const node = dtree.nodes.get(id);
        if (node) for (const pid of node.parentIds) queue.push(pid);
    }
    return path;
}

function renderDerivationTree(): void {
    if (!treeView || !treePanel) return;

    if (dtree.hasEdges()) {
        treePanel.classList.remove('hidden');
    } else {
        treePanel.classList.add('hidden');
        return;
    }

    treeView.innerHTML = '';
    if (dtree.rootId == null) return;
    const visited = new Set<number>();
    treeView.appendChild(renderTreeNode(dtree.rootId, visited));

    const currentLabel = treeView.querySelector('.dtree-label.current');
    if (currentLabel) {
        currentLabel.scrollIntoView({ block: 'nearest', inline: 'nearest', behavior: 'smooth' });
    }
}

function renderTreeNode(nodeId: number, visited: Set<number>): HTMLElement {
    const node = dtree.nodes.get(nodeId)!;
    const isCurrent = nodeId === dtree.currentId;
    const alreadySeen = visited.has(nodeId);
    visited.add(nodeId);

    const container = document.createElement('div');
    container.className = 'dtree-node';

    const label = document.createElement('div');
    label.className = 'dtree-label' + (isCurrent ? ' current' : '');
    label.addEventListener('click', () => navigateToNode(nodeId));

    const dot = document.createElement('span');
    dot.className = 'dtree-dot';
    if (node.parentIds.length > 1) dot.classList.add('diamond');
    label.appendChild(dot);

    const text = document.createElement('span');
    text.className = 'dtree-text';
    text.textContent = truncateDisplay(node.displayStr, 40);
    label.appendChild(text);

    if (alreadySeen) {
        const ref = document.createElement('span');
        ref.className = 'dtree-ref';
        ref.textContent = '\u21b5';
        label.appendChild(ref);
    }

    container.appendChild(label);

    const selfLoops = node.children.filter(c => c.id === nodeId);
    if (selfLoops.length > 0) {
        const loopEl = document.createElement('div');
        loopEl.className = 'dtree-self-loop';
        loopEl.innerHTML = `<span class="dtree-loop-arrow">\u21ba</span><span class="dtree-loop-label">${selfLoops[0]!.opLabel} (loops to self)</span>`;
        loopEl.addEventListener('click', () => navigateToNode(nodeId));
        container.appendChild(loopEl);
    }

    const nonSelfChildren = node.children.filter(c => c.id !== nodeId);
    if (!alreadySeen && nonSelfChildren.length > 0) {
        const children = document.createElement('div');
        children.className = 'dtree-children';

        for (const child of nonSelfChildren) {
            const branch = document.createElement('div');
            branch.className = 'dtree-branch';

            const edge = document.createElement('span');
            edge.className = 'dtree-edge';
            edge.textContent = child.opLabel;
            branch.appendChild(edge);

            branch.appendChild(renderTreeNode(child.id, visited));
            children.appendChild(branch);
        }
        container.appendChild(children);
    }

    return container;
}

function truncateDisplay(s: string, max: number): string {
    if (s.length <= max) return s;
    return s.slice(0, max - 1) + '\u2026';
}

// --- Strategy ---
function setStrategy(name: StrategyName): void {
    engine.set_strategy(name);
    strategyBtns.forEach(btn => {
        btn.classList.toggle('active', btn.dataset['strategy'] === name);
    });
    renderCurrentTerm();
}

function startAutoRun(): void {
    runBtn.style.display = 'none';
    stopBtn.style.display = '';
    autoRunInterval = setInterval(() => {
        try {
            const preInfo: TermInfo = JSON.parse(engine.get_term_info());
            const redexPath = preInfo.strategy_next || '';
            engine.step_strategy();
            const display = engine.get_display();
            const renderTreeJson = engine.get_render_tree();
            dtree.addChild(display, '\u03b2', renderTreeJson, redexPath);
            stepCount++;
            renderCurrentTerm();
        } catch (_) {
            stopAutoRun();
        }
    }, 400);
}

function stopAutoRun(): void {
    if (autoRunInterval != null) clearInterval(autoRunInterval);
    autoRunInterval = null;
    runBtn.style.display = '';
    stopBtn.style.display = 'none';
}

function updateStepInfo(info: TermInfo | null): void {
    stepCountEl.textContent = `Steps: ${stepCount}`;
    if (info && info.is_normal_form) {
        normalFormBadge.style.display = '';
        stepBtn.disabled = true;
    } else {
        normalFormBadge.style.display = 'none';
        stepBtn.disabled = false;
    }
}

// --- Zoom & Pan ---
function setZoom(z: number): void {
    zoom = Math.max(0.1, Math.min(5, z));
    updateTransform();
}

function updateTransform(): void {
    termContainer.style.transform = `translate(${panX}px, ${panY}px) scale(${zoom})`;
    zoomLevel.textContent = Math.round(zoom * 100) + '%';
}

function centerOnCurrent(smooth: boolean): void {
    const vpRect = viewport.getBoundingClientRect();
    const currentCard = document.getElementById('current-term-card');
    if (!currentCard) return;
    const cx = currentCard.offsetLeft;
    const cy = currentCard.offsetTop + currentCard.offsetHeight / 2;
    panX = vpRect.width / 2 - cx * zoom;
    panY = vpRect.height / 2 - cy * zoom;
    if (smooth) {
        termContainer.style.transition = 'transform 0.2s cubic-bezier(0.22, 1, 0.36, 1)';
        updateTransform();
        const clear = (): void => { termContainer.style.transition = 'none'; };
        termContainer.addEventListener('transitionend', clear, { once: true });
        setTimeout(clear, 250);
    } else {
        updateTransform();
    }
}

function onWheel(e: WheelEvent): void {
    e.preventDefault();
    const factor = e.deltaY < 0 ? 1.1 : 0.9;
    const rect = viewport.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
    const newZoom = Math.max(0.1, Math.min(5, zoom * factor));
    const scale = newZoom / zoom;
    panX = mx - scale * (mx - panX);
    panY = my - scale * (my - panY);
    zoom = newZoom;
    updateTransform();
}

function onMouseDown(e: MouseEvent): void {
    if (e.button !== 0) return;
    isDragging = true;
    didDrag = false;
    mouseDownX = e.clientX;
    mouseDownY = e.clientY;
    dragStartX = e.clientX - panX;
    dragStartY = e.clientY - panY;
    viewport.classList.add('dragging');
}

function onMouseMove(e: MouseEvent): void {
    if (!isDragging) return;
    const dx = e.clientX - mouseDownX;
    const dy = e.clientY - mouseDownY;
    if (!didDrag && (dx * dx + dy * dy) < 16) return;
    didDrag = true;
    panX = e.clientX - dragStartX;
    panY = e.clientY - dragStartY;
    updateTransform();
}

function onMouseUp(): void {
    isDragging = false;
    viewport.classList.remove('dragging');
}

// --- Status Bar ---
function updateStatusBar(info: TermInfo | null): void {
    if (!info) {
        statusSize.textContent = '';
        statusFv.textContent = '';
        statusRedex.textContent = '';
        statusStrategy.textContent = '';
        return;
    }
    statusSize.textContent = `Size: ${info.size}`;
    statusFv.textContent = info.free_vars.length > 0
        ? `FV: {${info.free_vars.join(', ')}}`
        : 'Closed term';
    statusRedex.textContent = `Redexes: ${info.redex_count}`;
    const strat = engine.get_strategy();
    const names: Record<string, string> = { normal: 'Normal', applicative: 'Applicative', cbv: 'CBV', cbn: 'CBN' };
    statusStrategy.textContent = `Strategy: ${names[strat] || strat}`;
}

// --- Error flash ---
function flashInfo(msg: string): void {
    const el = document.createElement('div');
    el.style.cssText = `
        position: fixed; bottom: 40px; left: 50%; transform: translateX(-50%);
        background: #ecfdf5; color: #065f46; border: 1px solid #a7f3d0;
        padding: 8px 16px; border-radius: 8px; font-size: 0.85em;
        box-shadow: 0 2px 12px rgba(0,0,0,0.08); z-index: 400;
        transition: opacity 0.3s;
    `;
    el.textContent = msg;
    document.body.appendChild(el);
    setTimeout(() => { el.style.opacity = '0'; }, 1500);
    setTimeout(() => { el.remove(); }, 1900);
}

function flashError(raw: unknown): void {
    let msg = String(raw);
    if (msg.startsWith('JsValue(')) msg = msg.slice(8, -1);
    if (msg.startsWith('"') && msg.endsWith('"')) msg = msg.slice(1, -1);

    const el = document.createElement('div');
    el.style.cssText = `
        position: fixed; bottom: 40px; left: 50%; transform: translateX(-50%);
        background: #fef2f2; color: #991b1b; border: 1px solid #fecaca;
        padding: 10px 18px; border-radius: 10px; font-size: 0.85em;
        max-width: calc(100vw - 40px); line-height: 1.5;
        box-shadow: 0 2px 12px rgba(0,0,0,0.1); z-index: 400;
        transition: opacity 0.3s;
    `;
    el.textContent = msg;
    document.body.appendChild(el);
    const displayTime = Math.max(2500, Math.min(msg.length * 40, 6000));
    setTimeout(() => { el.style.opacity = '0'; }, displayTime);
    setTimeout(() => { el.remove(); }, displayTime + 400);
}

// --- Tutorial ---
let tutorialStep = -1;
const tutorialEl = $<HTMLElement>('#tutorial');
const tutorialBody = $<HTMLElement>('#tutorial-body');
const tutorialIndicator = $<HTMLElement>('#tutorial-indicator');
const tutorialDots = $<HTMLElement>('#tutorial-dots');
const tutorialPrev = $<HTMLButtonElement>('#tutorial-prev');
const tutorialNext = $<HTMLButtonElement>('#tutorial-next');
const tutorialCloseBtn = $<HTMLElement>('#tutorial-close');
const tutorialStartBtn = $<HTMLElement>('#tutorial-btn');
const tutorialFab = $<HTMLElement>('#tutorial-fab');

// Strategy demo timer — stored on window to survive across showTutorialStep calls
declare global {
    interface Window {
        _strategyDemoTimer: ReturnType<typeof setInterval> | null;
    }
}
window._strategyDemoTimer = null;

function tutorialActive(): boolean { return tutorialStep >= 0; }

function confirmExitTutorial(): boolean {
    if (!tutorialActive()) return true;
    showTutorialExitPrompt();
    return false;
}

function showTutorialExitPrompt(): void {
    const orig = tutorialBody.innerHTML;
    tutorialBody.innerHTML = `
        <div class="tutorial-title">Exit tutorial?</div>
        <p>You're in the middle of the tutorial. Do you want to leave and explore on your own?</p>
        <div class="tut-exit-actions">
            <button class="btn" id="tut-stay">Stay in tutorial</button>
            <button class="btn btn-primary" id="tut-leave">Exit tutorial</button>
        </div>
    `;
    document.getElementById('tut-stay')!.addEventListener('click', () => {
        tutorialBody.innerHTML = orig;
    });
    document.getElementById('tut-leave')!.addEventListener('click', () => {
        endTutorial();
    });
}

function startTutorial(): void {
    tutorialStep = 0;
    showTutorialStep();
    closeDrawer();
    if (tutorialFab) tutorialFab.classList.add('hidden');
}

function endTutorial(): void {
    tutorialStep = -1;
    tutorialEl.classList.add('hidden');
    showTreeCallout(false);
    termDisplay.classList.remove('show-strategy-redexes');
    if (window._strategyDemoTimer) { clearInterval(window._strategyDemoTimer); window._strategyDemoTimer = null; }
    if (tutorialFab) tutorialFab.classList.remove('hidden');
}


function showTreeCallout(show: boolean): void {
    const el = document.getElementById('tree-callout');
    if (!el) return;
    if (show) {
        const fresh = el.cloneNode(true) as HTMLElement;
        fresh.classList.remove('hidden');
        el.parentNode!.replaceChild(fresh, el);
    } else {
        el.classList.add('hidden');
    }
}

function showTutorialStep(): void {
    if (tutorialStep < 0 || tutorialStep >= TUTORIAL_STEPS.length) { endTutorial(); return; }
    const step = TUTORIAL_STEPS[tutorialStep]!;

    tutorialEl.classList.remove('hidden');
    tutorialIndicator.textContent = `${tutorialStep + 1} / ${TUTORIAL_STEPS.length}`;

    showTreeCallout(false);

    tutorialBody.innerHTML = `<div class="tutorial-title">${step.title}</div>${step.body}`;

    tutorialDots.innerHTML = '';
    for (let i = 0; i < TUTORIAL_STEPS.length; i++) {
        const dot = document.createElement('span');
        dot.className = 'tutorial-dot' + (i === tutorialStep ? ' active' : '');
        dot.addEventListener('click', () => { tutorialStep = i; showTutorialStep(); });
        tutorialDots.appendChild(dot);
    }

    tutorialPrev.style.visibility = tutorialStep === 0 ? 'hidden' : 'visible';
    tutorialNext.textContent = tutorialStep === TUTORIAL_STEPS.length - 1 ? 'Finish' : 'Next \u2192';

    // Clean up demo classes
    termDisplay.classList.remove('show-strategy-redexes');
    termDisplay.classList.remove('show-all-branches');
    if (window._strategyDemoTimer) { clearInterval(window._strategyDemoTimer); window._strategyDemoTimer = null; }

    if (step.term) loadNewTerm(step.term);

    // Strategy demo
    if (step.strategyDemo) {
        termDisplay.classList.add('show-strategy-redexes');
        const strategies: StrategyName[] = ['normal', 'applicative', 'cbv', 'cbn'];
        let idx = 0;
        setStrategy(strategies[idx]!);
        window._strategyDemoTimer = setInterval(() => {
            if (tutorialStep !== TUTORIAL_STEPS.indexOf(step)) {
                clearInterval(window._strategyDemoTimer!);
                window._strategyDemoTimer = null;
                termDisplay.classList.remove('show-strategy-redexes');
                return;
            }
            idx = (idx + 1) % strategies.length;
            setStrategy(strategies[idx]!);
        }, 2500);
    }

    // Auto-stepping
    if (step.autoSteps) {
        const indicator = document.getElementById('tut-space-indicator');
        let count = 0;
        const total = step.autoSteps;
        function doAutoStep(): void {
            if (count >= total || tutorialStep !== TUTORIAL_STEPS.indexOf(step)) return;
            if (indicator) {
                const key = document.createElement('span');
                key.className = 'tut-space-key pressing';
                key.textContent = 'Space';
                indicator.appendChild(key);
                requestAnimationFrame(() => key.classList.add('pressed'));
                setTimeout(() => key.classList.remove('pressed'), 200);
            }
            setTimeout(() => {
                try { stepBtn.click(); } catch (_) { /* ignore */ }
                count++;
                if (count === 1) setTimeout(() => showTreeCallout(true), 200);
                if (count < total) setTimeout(doAutoStep, 1200);
            }, 400);
        }
        setTimeout(doAutoStep, 800);
    }

    // Church-Rosser demo
    if (step.crDemo) {
        setTimeout(() => runChurchRosserDemo(step), 600);
    }
}

// Church-Rosser demo
function runChurchRosserDemo(step: TutorialStep): void {
    const crStepIdx = TUTORIAL_STEPS.indexOf(step);
    const statusElMaybe = document.getElementById('tut-cr-status');
    if (!statusElMaybe) return;
    const statusEl = statusElMaybe;

    function stillActive(): boolean { return tutorialStep === crStepIdx; }

    // Pre-compute normal form (used for verification)
    try {
        const tmp = new LambdaEngine();
        tmp.parse_and_set(step.term!);
        tmp.set_strategy('normal');
        for (let i = 0; i < 20; i++) { try { tmp.step_strategy(); } catch (_) { break; } }
    } catch (_) { /* ignore */ }

    // Phase 1: Normal order
    statusEl.innerHTML = '<span class="tut-cr-badge">Normal order</span> stepping...';
    setStrategy('normal');

    let phase1Count = 0;
    const phase1Max = 10;
    function phase1Step(): void {
        if (!stillActive()) return;
        try {
            const info: TermInfo = JSON.parse(engine.get_term_info());
            if (info.strategy_next == null) {
                statusEl.innerHTML = '<span class="tut-cr-badge">Normal order</span> reached normal form: <code>' + engine.get_display() + '</code>';
                setTimeout(transitionPhase, 1200);
                return;
            }
            stepBtn.click();
            phase1Count++;
            if (phase1Count < phase1Max) {
                setTimeout(phase1Step, 1000);
            } else {
                setTimeout(transitionPhase, 1200);
            }
        } catch (_) {
            setTimeout(transitionPhase, 1200);
        }
    }
    setTimeout(phase1Step, 800);

    function transitionPhase(): void {
        if (!stillActive()) return;
        const nf1 = engine.get_display();

        statusEl.innerHTML = '';
        const prompt = document.createElement('div');
        prompt.className = 'tut-cr-prompt';
        prompt.innerHTML = `
            <p>Normal order reached <code>${nf1}</code>.</p>
            <p>Now let's go back to the start and try <strong>Applicative order</strong> \u2014 will it reach the same result?</p>
            <button class="btn btn-primary btn-small" id="tut-cr-go">Ready \u2014 show me!</button>
        `;
        statusEl.appendChild(prompt);

        document.getElementById('tut-cr-go')!.addEventListener('click', () => {
            if (!stillActive()) return;
            phase2Start(nf1);
        });
    }

    function phase2Start(nf1: string): void {
        if (!stillActive()) return;

        navigateToNode(dtree.rootId!);
        centerOnCurrent(true);

        statusEl.innerHTML = '<span class="tut-cr-badge">Applicative order</span> stepping...';
        setStrategy('applicative');

        let phase2Count = 0;
        const phase2Max = 10;
        function phase2Step(): void {
            if (!stillActive()) return;
            try {
                const info: TermInfo = JSON.parse(engine.get_term_info());
                if (info.strategy_next == null) {
                    phase2Done(nf1);
                    return;
                }
                stepBtn.click();
                phase2Count++;
                if (phase2Count < phase2Max) {
                    setTimeout(phase2Step, 1000);
                } else {
                    phase2Done(nf1);
                }
            } catch (_) {
                phase2Done(nf1);
            }
        }
        setTimeout(phase2Step, 800);
    }

    function phase2Done(nf1: string): void {
        if (!stillActive()) return;
        const nf2 = engine.get_display();
        const converges = nf1 === nf2;

        const result = document.createElement('div');
        result.className = 'tut-cr-result' + (converges ? ' converges' : '');
        result.innerHTML = converges
            ? `Both paths arrived at <strong><code>${nf2}</code></strong> \u2014 Church-Rosser confirmed! \u2714`
            : `Normal order got <code>${nf1}</code>, Applicative got <code>${nf2}</code>.`;

        statusEl.innerHTML = '';
        statusEl.appendChild(result);

        termDisplay.classList.add('show-all-branches');
        renderCurrentTerm();

        requestAnimationFrame(() => {
            const cards = termDisplay.querySelectorAll('.main-node-card');
            if (cards.length === 0) return;
            let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
            cards.forEach(c => {
                const el = c as HTMLElement;
                const x = parseFloat(el.style.left) || 0;
                const y = parseFloat(el.style.top) || 0;
                minX = Math.min(minX, x);
                minY = Math.min(minY, y);
                maxX = Math.max(maxX, x + el.offsetWidth);
                maxY = Math.max(maxY, y + el.offsetHeight);
            });
            const vw = viewport.clientWidth;
            const vh = viewport.clientHeight;
            const dagW = maxX - minX + 120;
            const dagH = maxY - minY + 120;
            const scale = Math.min(vw / dagW, vh / dagH, 1);
            const cx = (minX + maxX) / 2;
            const cy = (minY + maxY) / 2;
            panX = vw / 2 - cx * scale;
            panY = vh / 2 - cy * scale;
            zoom = scale;
            updateTransform();
        });
    }
}

if (tutorialStartBtn) tutorialStartBtn.addEventListener('click', startTutorial);
if (tutorialFab) tutorialFab.addEventListener('click', startTutorial);
if (tutorialPrev) tutorialPrev.addEventListener('click', () => { tutorialStep--; showTutorialStep(); });
if (tutorialNext) tutorialNext.addEventListener('click', () => {
    if (tutorialStep >= TUTORIAL_STEPS.length - 1) endTutorial();
    else { tutorialStep++; showTutorialStep(); }
});
if (tutorialCloseBtn) tutorialCloseBtn.addEventListener('click', endTutorial);

// --- Welcome Nudge ---
function maybeShowWelcome(): void {
    try {
        if (sessionStorage.getItem('lambda_welcomed')) return;
    } catch (_) { /* private browsing */ }
    showWelcomeNudge();
}

function showWelcomeNudge(): void {
    try { sessionStorage.setItem('lambda_welcomed', '1'); } catch (_) { /* ignore */ }

    const el = document.createElement('div');
    el.className = 'welcome-nudge';
    el.innerHTML = `
        <div class="welcome-text">
            <strong>New here?</strong> Take the interactive tutorial to learn the lambda calculus.
        </div>
        <div class="welcome-actions">
            <button class="btn btn-primary btn-small" id="welcome-start">Start tutorial</button>
            <button class="btn btn-small" id="welcome-dismiss">Dismiss</button>
        </div>
    `;
    document.body.appendChild(el);

    requestAnimationFrame(() => el.classList.add('visible'));

    document.getElementById('welcome-start')!.addEventListener('click', () => {
        el.remove();
        startTutorial();
    });
    document.getElementById('welcome-dismiss')!.addEventListener('click', () => dismiss());

    function dismiss(): void {
        el.style.transition = 'opacity 0.8s ease-out, transform 0.8s ease-out';
        el.style.opacity = '0';
        el.style.transform = 'translateY(-6px)';
        setTimeout(() => el.remove(), 850);
    }

    const autoTimer = setTimeout(dismiss, 4000);
    el.addEventListener('mouseenter', () => clearTimeout(autoTimer));
}

