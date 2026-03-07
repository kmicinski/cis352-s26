// ============================================================
// Lambda Calculus Explorer — Main Application
// ============================================================

import init, { LambdaEngine } from './pkg/lambda_viz.js';

// --- Constants ---
const EXAMPLES = {
    id:      '(\u03bb (x) x)',
    omega:   '((\u03bb (x) (x x)) (\u03bb (x) (x x)))',
    K:       '(\u03bb (x y) x)',
    S:       '(\u03bb (x y z) ((x z) (y z)))',
    skk:     '(((\u03bb (x y z) ((x z) (y z))) (\u03bb (x y) x)) (\u03bb (x y) x))',
    church0: '(\u03bb (f x) x)',
    church2: '(\u03bb (f x) (f (f x)))',
    succ:    '(\u03bb (n f x) (f ((n f) x)))',
    Y:       '(\u03bb (f) ((\u03bb (x) (f (x x))) (\u03bb (x) (f (x x)))))',
    true:    '(\u03bb (t f) t)',
    false:   '(\u03bb (t f) f)',
};

// --- Derivation Tree ---
class DerivationTree {
    constructor() {
        this.nodes = new Map();
        this.rootId = null;
        this.currentId = null;
        this.nextId = 0;
    }

    setRoot(displayStr) {
        this.nodes.clear();
        this.nextId = 0;
        const id = this.nextId++;
        this.nodes.set(id, { id, displayStr, parentId: null, children: [] });
        this.rootId = id;
        this.currentId = id;
        return id;
    }

    addChild(displayStr, opLabel) {
        const parentId = this.currentId;
        const id = this.nextId++;
        const node = { id, displayStr, parentId, children: [] };
        this.nodes.set(id, node);
        this.nodes.get(parentId).children.push({ id, opLabel });
        this.currentId = id;
        return id;
    }

    goTo(id) {
        if (this.nodes.has(id)) {
            this.currentId = id;
            return this.nodes.get(id);
        }
        return null;
    }

    getCurrent() { return this.nodes.get(this.currentId); }

    getParent() {
        const cur = this.getCurrent();
        return cur && cur.parentId != null ? this.nodes.get(cur.parentId) : null;
    }

    canUndo() { return this.getParent() != null; }
    hasMultipleNodes() { return this.nodes.size > 1; }

    // Check if tree has any branching (a node with >1 child)
    hasBranches() {
        for (const node of this.nodes.values()) {
            if (node.children.length > 1) return true;
        }
        return false;
    }
}

// --- State ---
let engine = null;
const dtree = new DerivationTree();
let hoveredEl = null;
let zoom = 1, panX = 0, panY = 0;
let isDragging = false, didDrag = false;
let dragStartX = 0, dragStartY = 0, mouseDownX = 0, mouseDownY = 0;
let stepCount = 0;
let autoRunInterval = null;
let toolbarTimeout = null;
let contextMenuPath = null;
let treeCollapsed = false;

// --- DOM Refs ---
const $ = (sel) => document.querySelector(sel);
const toolbar = $('#toolbar');
const trigger = $('#toolbar-trigger');
const termInput = $('#term-input');
const parseBtn = $('#parse-btn');
const examplesSelect = $('#examples-select');
const randomBtn = $('#random-btn');
const closedCheck = $('#closed-check');
const undoBtn = $('#undo-btn');
const viewport = $('#viewport');
const termContainer = $('#term-container');
const termDisplay = $('#term-display');
const zoomIn = $('#zoom-in');
const zoomOut = $('#zoom-out');
const zoomReset = $('#zoom-reset');
const zoomLevel = $('#zoom-level');
const contextMenu = $('#context-menu');
const alphaOverlay = $('#alpha-overlay');
const alphaNameInput = $('#alpha-name');
const alphaInfo = $('#alpha-info');
const alphaApply = $('#alpha-apply');
const alphaCancel = $('#alpha-cancel');
const strategyPanel = $('#strategy-panel');
const strategyToggle = $('#strategy-toggle');
const panelClose = $('#panel-close');
const stepBtn = $('#step-btn');
const runBtn = $('#run-btn');
const stopBtn = $('#stop-btn');
const stepCountEl = $('#step-count');
const normalFormBadge = $('#normal-form-badge');
const statusSize = $('#term-size');
const statusFv = $('#term-fv');
const statusRedex = $('#redex-count');
const statusStrategy = $('#strategy-name');
const treePanel = $('#tree-panel');
const treeView = $('#tree-view');
const treeToggleBtn = $('#tree-toggle');

// --- Initialization ---
async function main() {
    await init();
    engine = new LambdaEngine();
    setupEventListeners();
    showToolbar();

    // Check URL params for linked terms
    const params = new URLSearchParams(window.location.search);
    const urlTerm = params.get('term');
    if (urlTerm) {
        termInput.value = urlTerm;
        loadNewTerm(urlTerm);
    } else {
        termInput.value = EXAMPLES.skk;
        loadNewTerm(EXAMPLES.skk);
    }
}

main().catch(console.error);

// --- Core actions ---
function loadNewTerm(input) {
    try {
        engine.parse_and_set(input);
        const display = engine.get_display();
        dtree.setRoot(display);
        stepCount = 0;
        renderCurrentTerm();
    } catch (e) {
        flashError(String(e));
    }
}

function performOperation(path, op, arg) {
    try {
        engine.apply_operation(path, op, arg || '');
        const display = engine.get_display();
        const opLabel = op === 'beta' ? '\u03b2' : op === 'eta' ? '\u03b7' : '\u03b1';
        dtree.addChild(display, opLabel);
        stepCount++;
        renderCurrentTerm();
    } catch (e) {
        flashError(String(e));
    }
}

function navigateToNode(id) {
    const node = dtree.goTo(id);
    if (!node) return;
    try {
        engine.parse_and_set(node.displayStr);
        renderCurrentTerm();
    } catch (e) {
        flashError(String(e));
    }
}

function undo() {
    if (!dtree.canUndo()) return;
    const parent = dtree.getParent();
    navigateToNode(parent.id);
    stepCount = Math.max(0, stepCount - 1);
}

// --- Toolbar visibility ---
function showToolbar() {
    toolbar.classList.add('visible');
    clearTimeout(toolbarTimeout);
}

function hideToolbar() {
    toolbarTimeout = setTimeout(() => {
        if (document.activeElement === termInput) return;
        toolbar.classList.remove('visible');
    }, 600);
}

trigger.addEventListener('mouseenter', showToolbar);
toolbar.addEventListener('mouseenter', () => {
    clearTimeout(toolbarTimeout);
    toolbar.classList.add('visible');
});
toolbar.addEventListener('mouseleave', hideToolbar);

// --- Event Listeners ---
function setupEventListeners() {
    // Parse
    parseBtn.addEventListener('click', () => {
        const input = termInput.value.trim();
        if (input) loadNewTerm(input);
    });
    termInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            const input = termInput.value.trim();
            if (input) loadNewTerm(input);
        }
    });
    termInput.addEventListener('focus', showToolbar);

    // Examples
    examplesSelect.addEventListener('change', () => {
        const val = examplesSelect.value;
        if (val && EXAMPLES[val]) {
            termInput.value = EXAMPLES[val];
            loadNewTerm(EXAMPLES[val]);
        }
        examplesSelect.value = '';
    });

    // Random
    randomBtn.addEventListener('click', () => {
        engine.random_term(4, closedCheck.checked);
        const display = engine.get_display();
        dtree.setRoot(display);
        stepCount = 0;
        renderCurrentTerm();
        termInput.value = display;
    });

    // Undo
    undoBtn.addEventListener('click', undo);

    // Zoom
    zoomIn.addEventListener('click', () => setZoom(zoom * 1.2));
    zoomOut.addEventListener('click', () => setZoom(zoom / 1.2));
    zoomReset.addEventListener('click', () => { zoom = 1; panX = 0; panY = 0; updateTransform(); });
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
    document.addEventListener('click', (e) => {
        if (!contextMenu.contains(e.target)) hideContextMenu();
    });
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            hideContextMenu();
            hideAlphaDialog();
            closeStrategyPanel();
        }
        if (e.key === 'z' && (e.metaKey || e.ctrlKey)) {
            e.preventDefault();
            undo();
        }
    });

    // Alpha dialog
    alphaCancel.addEventListener('click', hideAlphaDialog);
    alphaApply.addEventListener('click', applyAlpha);
    alphaNameInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') applyAlpha();
        if (e.key === 'Escape') hideAlphaDialog();
    });

    // Strategy panel
    strategyToggle.addEventListener('click', toggleStrategyPanel);
    panelClose.addEventListener('click', closeStrategyPanel);
    document.querySelectorAll('input[name="strategy"]').forEach(radio => {
        radio.addEventListener('change', () => {
            engine.set_strategy(radio.value);
            renderCurrentTerm();
        });
    });
    stepBtn.addEventListener('click', () => {
        try {
            engine.step_strategy();
            const display = engine.get_display();
            const strat = engine.get_strategy();
            const labels = { normal: '\u03b2', applicative: '\u03b2', cbv: '\u03b2', cbn: '\u03b2' };
            dtree.addChild(display, labels[strat] || '\u03b2');
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
function renderCurrentTerm() {
    let treeJson, infoJson;
    try {
        treeJson = engine.get_render_tree();
        infoJson = engine.get_term_info();
    } catch (e) {
        termDisplay.innerHTML = '<div id="empty-state"><div class="empty-icon">\u03bb</div><p>Enter a term above or choose an example to begin</p></div>';
        updateStatusBar(null);
        return;
    }

    const renderTree = JSON.parse(treeJson);
    const info = JSON.parse(infoJson);

    termDisplay.innerHTML = '';
    const rendered = renderNode(renderTree);
    rendered.classList.add('term-animate');
    termDisplay.appendChild(rendered);

    // Highlight strategy next redex
    if (info.strategy_next) {
        const nextEl = termDisplay.querySelector(`[data-path="${info.strategy_next}"]`);
        if (nextEl) nextEl.classList.add('strategy-next');
    }

    undoBtn.disabled = !dtree.canUndo();
    updateStatusBar(info);
    updateStepInfo(info);
    termInput.value = info.display;

    // Update URL without reload
    const url = new URL(window.location);
    url.searchParams.set('term', info.display);
    window.history.replaceState({}, '', url);

    // Render derivation tree
    renderDerivationTree();
}

function renderNode(node) {
    const span = document.createElement('span');
    span.className = `term term-${node.kind}`;
    span.dataset.path = node.path;

    if (node.ops && node.ops.length > 0) {
        span.classList.add('has-op');
        span.dataset.ops = JSON.stringify(node.ops);
    }

    switch (node.kind) {
        case 'var': renderVar(span, node); break;
        case 'abs': renderAbs(span, node); break;
        case 'app': renderApp(span, node); break;
    }
    return span;
}

function renderVar(span, node) {
    const v = document.createElement('span');
    v.className = 'var-ref' + (node.is_free ? ' free' : '');
    v.textContent = node.name;
    if (node.binder_path != null) v.dataset.binderPath = node.binder_path;
    v.dataset.varName = node.name;
    span.appendChild(v);
}

function renderAbs(span, node) {
    if (node.ops.includes('eta')) span.classList.add('eta-convertible');
    span.appendChild(paren('('));
    span.appendChild(lambdaSym());
    span.appendChild(mkText('\u00a0'));
    span.appendChild(paren('('));
    const param = document.createElement('span');
    param.className = 'var-binding';
    param.textContent = node.param;
    param.dataset.absPath = node.path;
    param.dataset.varName = node.param;
    span.appendChild(param);
    span.appendChild(paren(')'));
    span.appendChild(mkText(' '));
    span.appendChild(renderNode(node.body));
    span.appendChild(paren(')'));
}

function renderApp(span, node) {
    if (node.ops.includes('beta')) span.classList.add('redex');
    span.appendChild(paren('('));
    span.appendChild(renderNode(node.func));
    span.appendChild(mkText(' '));
    span.appendChild(renderNode(node.arg));
    span.appendChild(paren(')'));
}

function paren(ch) {
    const s = document.createElement('span');
    s.className = 'paren';
    s.textContent = ch;
    return s;
}

function lambdaSym() {
    const s = document.createElement('span');
    s.className = 'lambda-sym';
    s.textContent = '\u03bb';
    return s;
}

function mkText(t) { return document.createTextNode(t); }

// --- Hover ---
// ONLY highlight terms that have actionable operations
function onTermHover(e) {
    // Find innermost .term.has-op under cursor
    const target = e.target.closest('.term.has-op');
    if (target === hoveredEl) return;

    clearHover();
    hoveredEl = target;
    if (!hoveredEl) return;

    hoveredEl.classList.add('hovered');

    // Highlight variable bindings
    const varRef = e.target.closest('.var-ref');
    if (varRef && varRef.dataset.binderPath != null) {
        highlightBinding(varRef.dataset.binderPath);
    }
    const varBinding = e.target.closest('.var-binding');
    if (varBinding && varBinding.dataset.absPath) {
        highlightBinding(varBinding.dataset.absPath);
    }
}

function clearHover() {
    if (hoveredEl) {
        hoveredEl.classList.remove('hovered');
        hoveredEl = null;
    }
    termDisplay.querySelectorAll('.binding-highlight').forEach(el => {
        el.classList.remove('binding-highlight');
    });
}

function highlightBinding(absPath) {
    termDisplay.querySelectorAll(`.var-binding[data-abs-path="${absPath}"]`).forEach(el => {
        el.classList.add('binding-highlight');
    });
    termDisplay.querySelectorAll(`.var-ref[data-binder-path="${absPath}"]`).forEach(el => {
        el.classList.add('binding-highlight');
    });
}

// --- Click / Context Menu ---
function onTermClick(e) {
    if (didDrag) return;
    hideContextMenu();

    const target = e.target.closest('.term.has-op');
    if (!target) return;
    e.stopPropagation();

    const ops = target.dataset.ops ? JSON.parse(target.dataset.ops) : [];
    if (ops.length === 0) return;

    contextMenuPath = target.dataset.path;

    // For bound variables, alpha targets the binder
    const varRef = e.target.closest('.var-ref');
    if (varRef && varRef.dataset.binderPath != null) {
        contextMenu.dataset.alphaTarget = varRef.dataset.binderPath;
    } else {
        contextMenu.dataset.alphaTarget = target.dataset.path;
    }

    showContextMenu(e.clientX, e.clientY, ops);
}

function showContextMenu(x, y, ops) {
    contextMenu.innerHTML = '';

    for (const op of ops) {
        const btn = document.createElement('button');
        btn.className = `ctx-btn ${op}`;
        btn.textContent = op === 'beta' ? '\u03b2' : op === 'eta' ? '\u03b7' : '\u03b1';
        btn.addEventListener('click', (e) => {
            e.stopPropagation();
            const path = contextMenuPath;
            const alphaTarget = contextMenu.dataset.alphaTarget;
            hideContextMenu();
            handleOperation(op, path, alphaTarget);
        });
        contextMenu.appendChild(btn);
    }

    // Position
    contextMenu.classList.remove('hidden');
    const rect = contextMenu.getBoundingClientRect();
    const vw = window.innerWidth;
    const vh = window.innerHeight;
    let fx = x - rect.width / 2;
    let fy = y - rect.height - 12;
    if (fx < 8) fx = 8;
    if (fx + rect.width > vw - 8) fx = vw - rect.width - 8;
    if (fy < 8) fy = y + 16;
    contextMenu.style.left = fx + 'px';
    contextMenu.style.top = fy + 'px';
}

function hideContextMenu() {
    contextMenu.classList.add('hidden');
    contextMenuPath = null;
}

function handleOperation(op, path, alphaTarget) {
    if (op === 'alpha') {
        showAlphaDialog(alphaTarget || path);
        return;
    }
    performOperation(path, op, '');
}

// --- Alpha Dialog ---
function showAlphaDialog(target) {
    alphaOverlay.classList.remove('hidden');
    alphaNameInput.value = '';
    alphaOverlay.dataset.targetPath = target;

    // Show current variable name
    const absEl = termDisplay.querySelector(`[data-path="${target}"]`);
    if (absEl) {
        const param = absEl.querySelector('.var-binding');
        if (param) {
            alphaInfo.innerHTML = `<span class="old-name">${param.textContent}</span> <span class="arrow">\u2192</span>`;
        }
    }

    // Focus after animation frame so it works reliably
    requestAnimationFrame(() => alphaNameInput.focus());
}

function hideAlphaDialog() {
    alphaOverlay.classList.add('hidden');
}

function applyAlpha() {
    const newName = alphaNameInput.value.trim();
    if (!newName) return;
    const target = alphaOverlay.dataset.targetPath;
    hideAlphaDialog();
    performOperation(target, 'alpha', newName);
}

// --- Derivation Tree View ---
function renderDerivationTree() {
    if (!treeView || !treePanel) return;

    // Show panel when there's history
    if (dtree.hasMultipleNodes()) {
        treePanel.classList.remove('hidden');
    } else {
        treePanel.classList.add('hidden');
        return;
    }

    treeView.innerHTML = '';
    if (dtree.rootId == null) return;
    treeView.appendChild(renderTreeNode(dtree.rootId));
}

function renderTreeNode(nodeId) {
    const node = dtree.nodes.get(nodeId);
    const isCurrent = nodeId === dtree.currentId;

    const container = document.createElement('div');
    container.className = 'dtree-node';

    const label = document.createElement('div');
    label.className = 'dtree-label' + (isCurrent ? ' current' : '');
    label.addEventListener('click', () => navigateToNode(nodeId));

    const dot = document.createElement('span');
    dot.className = 'dtree-dot';
    label.appendChild(dot);

    const text = document.createElement('span');
    text.className = 'dtree-text';
    text.textContent = truncateDisplay(node.displayStr, 40);
    label.appendChild(text);

    container.appendChild(label);

    if (node.children.length > 0) {
        const children = document.createElement('div');
        children.className = 'dtree-children';

        for (const child of node.children) {
            const branch = document.createElement('div');
            branch.className = 'dtree-branch';

            const edge = document.createElement('span');
            edge.className = 'dtree-edge';
            edge.textContent = child.opLabel;
            branch.appendChild(edge);

            branch.appendChild(renderTreeNode(child.id));
            children.appendChild(branch);
        }
        container.appendChild(children);
    }

    return container;
}

function truncateDisplay(s, max) {
    if (s.length <= max) return s;
    return s.slice(0, max - 1) + '\u2026';
}

// --- Strategy Panel ---
function toggleStrategyPanel() {
    strategyPanel.classList.toggle('panel-open');
    strategyPanel.classList.toggle('panel-closed');
}

function closeStrategyPanel() {
    strategyPanel.classList.remove('panel-open');
    strategyPanel.classList.add('panel-closed');
}

function startAutoRun() {
    runBtn.style.display = 'none';
    stopBtn.style.display = '';
    autoRunInterval = setInterval(() => {
        try {
            engine.step_strategy();
            const display = engine.get_display();
            dtree.addChild(display, '\u03b2');
            stepCount++;
            renderCurrentTerm();
        } catch (_) {
            stopAutoRun();
        }
    }, 400);
}

function stopAutoRun() {
    clearInterval(autoRunInterval);
    autoRunInterval = null;
    runBtn.style.display = '';
    stopBtn.style.display = 'none';
}

function updateStepInfo(info) {
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
function setZoom(z) {
    zoom = Math.max(0.1, Math.min(5, z));
    updateTransform();
}

function updateTransform() {
    termContainer.style.transform = `translate(${panX}px, ${panY}px) scale(${zoom})`;
    zoomLevel.textContent = Math.round(zoom * 100) + '%';
}

function onWheel(e) {
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

function onMouseDown(e) {
    if (e.button !== 0) return;
    isDragging = true;
    didDrag = false;
    mouseDownX = e.clientX;
    mouseDownY = e.clientY;
    dragStartX = e.clientX - panX;
    dragStartY = e.clientY - panY;
    viewport.classList.add('dragging');
}

function onMouseMove(e) {
    if (!isDragging) return;
    const dx = e.clientX - mouseDownX;
    const dy = e.clientY - mouseDownY;
    if (!didDrag && (dx * dx + dy * dy) < 16) return;
    didDrag = true;
    panX = e.clientX - dragStartX;
    panY = e.clientY - dragStartY;
    updateTransform();
}

function onMouseUp() {
    isDragging = false;
    viewport.classList.remove('dragging');
}

// --- Status Bar ---
function updateStatusBar(info) {
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
    const names = { normal: 'Normal', applicative: 'Applicative', cbv: 'CBV', cbn: 'CBN' };
    statusStrategy.textContent = `Strategy: ${names[strat] || strat}`;
}

// --- Error flash ---
function flashError(msg) {
    const el = document.createElement('div');
    el.style.cssText = `
        position: fixed; bottom: 40px; left: 50%; transform: translateX(-50%);
        background: #fef2f2; color: #991b1b; border: 1px solid #fecaca;
        padding: 8px 16px; border-radius: 8px; font-size: 0.85em;
        box-shadow: 0 2px 12px rgba(0,0,0,0.1); z-index: 400;
        transition: opacity 0.3s;
    `;
    el.textContent = msg;
    document.body.appendChild(el);
    setTimeout(() => { el.style.opacity = '0'; }, 2000);
    setTimeout(() => { el.remove(); }, 2400);
}
