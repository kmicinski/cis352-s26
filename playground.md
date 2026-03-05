---
layout: default
title: "Proof Tree Playground"
permalink: /playground
proof_tree: true
---

<style>
/* ── Full-page immersive layout ──────────────────── */
.pg-fullpage .wrapper { max-width: none; padding: 0; }
.pg-fullpage .page-content { padding: 0; }
.pg-fullpage .site-footer { display: none; }
.pg-fullpage .site-header { display: none; }

/* ── Floating toolbar ────────────────────────────── */
#pg-toolbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: rgba(255,255,255,0.92);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid rgba(0,0,0,0.08);
  opacity: 0;
  transform: translateY(-100%);
  transition: opacity 0.25s ease, transform 0.25s ease;
  flex-wrap: wrap;
}
#pg-toolbar.visible {
  opacity: 1;
  transform: translateY(0);
}
#pg-toolbar h2 {
  margin: 0;
  font-size: 1em;
  font-weight: 600;
  color: #374151;
  white-space: nowrap;
  letter-spacing: -0.01em;
}
#pg-toolbar .spacer { flex: 1; }

/* ── Toolbar buttons ─────────────────────────────── */
.pg-btn {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 5px 14px;
  font-size: 0.82em;
  font-weight: 500;
  font-family: inherit;
  color: #374151;
  background: #fff;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s;
  white-space: nowrap;
}
.pg-btn:hover { background: #f3f4f6; border-color: #9ca3af; }
.pg-btn-primary { color: #fff; background: #4f46e5; border-color: #4f46e5; }
.pg-btn-primary:hover { background: #4338ca; border-color: #4338ca; }
.pg-btn-check { color: #fff; background: #059669; border-color: #059669; }
.pg-btn-check:hover { background: #047857; border-color: #047857; }
.pg-btn-danger { color: #fff; background: #dc2626; border-color: #dc2626; }
.pg-btn-danger:hover { background: #b91c1c; border-color: #b91c1c; }
.pg-btn-sm { padding: 3px 10px; font-size: 0.78em; }
.pg-select {
  padding: 5px 8px;
  font-size: 0.82em;
  font-family: inherit;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  background: #fff;
  color: #374151;
  max-width: 260px;
}

/* ── S-expression panel (slide-down) ─────────────── */
#pg-sexp-panel {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 99;
  background: rgba(255,255,255,0.96);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-bottom: 1px solid #e5e7eb;
  padding: 50px 20px 14px;
  transform: translateY(-100%);
  transition: transform 0.3s ease;
  box-shadow: 0 4px 24px rgba(0,0,0,0.08);
}
#pg-sexp-panel.open {
  transform: translateY(0);
}
#pg-sexp-input {
  width: 100%;
  font-family: 'SF Mono','Menlo','Consolas',monospace;
  font-size: 0.82em;
  padding: 10px;
  border: 1px solid #d1d5db;
  border-radius: 8px;
  resize: vertical;
  outline: none;
  transition: border-color 0.15s;
}
#pg-sexp-input:focus { border-color: #4f46e5; }
#pg-sexp-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
  align-items: center;
}
#pg-sexp-actions .pg-hint {
  font-size: 0.75em;
  color: #9ca3af;
  margin-left: auto;
}

/* ── The proof editor canvas (full viewport) ─────── */
#pg-canvas {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: #fafafa;
  overflow: hidden;
}
#pg-canvas .proof-tree-viewport {
  width: 100%;
  height: 100%;
}

/* ── Empty state ─────────────────────────────────── */
#pg-empty {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  text-align: center;
  color: #9ca3af;
  pointer-events: none;
}
#pg-empty h3 {
  font-size: 1.2em;
  font-weight: 500;
  margin: 0 0 8px;
  color: #6b7280;
}
#pg-empty p {
  font-size: 0.9em;
  margin: 4px 0;
}

/* ── Check result banner ─────────────────────────── */
#pg-check-result {
  position: fixed;
  bottom: 16px;
  right: 16px;
  z-index: 101;
  padding: 10px 18px;
  border-radius: 8px;
  font-size: 0.85em;
  font-weight: 500;
  box-shadow: 0 2px 12px rgba(0,0,0,0.15);
  transition: opacity 0.3s ease, transform 0.3s ease;
  opacity: 0;
  transform: translateY(10px);
  pointer-events: none;
  max-width: 500px;
}
#pg-check-result.visible {
  opacity: 1;
  transform: translateY(0);
  pointer-events: auto;
}
#pg-check-result.valid {
  background: #ecfdf5;
  color: #065f46;
  border: 1px solid #a7f3d0;
}
#pg-check-result.error {
  background: #fef2f2;
  color: #991b1b;
  border: 1px solid #fecaca;
}
#pg-check-result.incomplete {
  background: #fffbeb;
  color: #92400e;
  border: 1px solid #fde68a;
}

/* ── Rules panel (right drawer) ──────────────────── */
#pg-rules-panel {
  position: fixed;
  top: 0;
  right: 0;
  bottom: 0;
  z-index: 98;
  width: 420px;
  max-width: 100vw;
  background: rgba(255,255,255,0.97);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border-left: 1px solid #e5e7eb;
  box-shadow: -4px 0 24px rgba(0,0,0,0.08);
  transform: translateX(100%);
  transition: transform 0.3s cubic-bezier(0.4,0,0.2,1);
  overflow-y: auto;
  -webkit-overflow-scrolling: touch;
  padding: 0;
}
#pg-rules-panel.open {
  transform: translateX(0);
}
@media (max-width: 900px) {
  #pg-rules-panel { width: 340px; }
}
@media (max-width: 600px) {
  #pg-rules-panel { width: 100vw; }
}

#pg-rules-header {
  position: sticky;
  top: 0;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 18px;
  background: rgba(255,255,255,0.95);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  border-bottom: 1px solid #e5e7eb;
}
#pg-rules-header h3 {
  margin: 0;
  font-size: 0.95em;
  font-weight: 600;
  color: #1f2937;
  flex: 1;
}
#pg-rules-close {
  background: none;
  border: none;
  font-size: 1.3em;
  color: #9ca3af;
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
  line-height: 1;
}
#pg-rules-close:hover { color: #374151; background: #f3f4f6; }

#pg-rules-body {
  padding: 12px 18px 24px;
}
#pg-rules-body .pg-rules-judgement {
  font-size: 0.8em;
  color: #6b7280;
  margin: 0 0 14px;
  padding: 8px 12px;
  background: #f9fafb;
  border-radius: 6px;
  border: 1px solid #f3f4f6;
}
#pg-rules-body .pg-rules-judgement code {
  font-family: 'SF Mono','Menlo','Consolas',monospace;
  font-size: 1.05em;
  color: #4f46e5;
}

/* ── Individual inference rule card ───────────────── */
.pg-rule {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: 0 0 16px;
  padding: 10px 14px;
  background: #fafafa;
  border: 1px solid #f0f0f0;
  border-radius: 8px;
  transition: border-color 0.15s;
}
.pg-rule:hover { border-color: #d1d5db; }

.pg-rule-body {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
  min-width: 0;
}
.pg-rule-premises {
  display: flex;
  gap: 16px;
  justify-content: center;
  flex-wrap: wrap;
  padding-bottom: 5px;
  min-height: 1.3em;
  font-family: 'SF Mono','Menlo','Consolas',monospace;
  font-size: 0.78em;
  color: #374151;
}
.pg-rule-line {
  width: 100%;
  height: 1px;
  background: #374151;
  margin: 2px 0;
}
.pg-rule-conclusion {
  padding-top: 5px;
  font-family: 'SF Mono','Menlo','Consolas',monospace;
  font-size: 0.78em;
  color: #374151;
  text-align: center;
  word-break: break-word;
}
.pg-rule-name {
  font-size: 0.75em;
  font-weight: 600;
  color: #4f46e5;
  white-space: nowrap;
  min-width: 50px;
  text-align: right;
  font-family: inherit;
}
.pg-rule-condition {
  font-size: 0.72em;
  color: #9ca3af;
  text-align: center;
  margin-top: 2px;
  font-style: italic;
}
.pg-rules-empty {
  text-align: center;
  color: #9ca3af;
  padding: 40px 20px;
  font-size: 0.9em;
}

/* ── Back link ───────────────────────────────────── */
#pg-back {
  position: fixed;
  bottom: 16px;
  left: 16px;
  z-index: 100;
  font-size: 0.78em;
  color: #9ca3af;
  text-decoration: none;
  opacity: 0.5;
  transition: opacity 0.2s;
}
#pg-back:hover { opacity: 1; color: #4f46e5; }
</style>

<div id="pg-canvas">
  <div id="pg-empty">
    <h3>Proof Tree Playground</h3>
    <p>Move your mouse to the top to load or paste a proof.</p>
    <p>Select a theory and example, or click the conclusion below to start.</p>
  </div>
</div>

<div id="pg-toolbar">
  <h2>Playground</h2>
  <select id="pg-theory" class="pg-select">
    <option value="">Freestyle (no checking)</option>
    <option value="big-step">Big-Step Semantics</option>
    <option value="small-step">Small-Step Semantics</option>
    <option value="g3ip">G3ip Sequent Calculus</option>
    <option value="propnd">Natural Deduction</option>
    <option value="stlc">Simply-Typed Lambda Calculus</option>
    <option value="systemf">System F</option>
  </select>
  <select id="pg-examples" class="pg-select">
    <option value="">Load example...</option>
  </select>
  <button class="pg-btn pg-btn-check pg-btn-sm" id="pg-check-btn" style="display:none;">Check</button>
  <button class="pg-btn pg-btn-sm" id="pg-rules-btn" style="display:none;">Rules</button>
  <button class="pg-btn pg-btn-sm" id="pg-sexp-toggle">S-expression</button>
  <button class="pg-btn pg-btn-sm" id="pg-new-btn">New</button>
  <button class="pg-btn pg-btn-sm pg-btn-danger" id="pg-reset-btn" style="display:none;">&#128465; Reset</button>
  <span class="spacer"></span>
  <button class="pg-btn pg-btn-sm" id="pg-copy-btn">Copy</button>
  <a href="{{ '/' | relative_url }}" class="pg-btn pg-btn-sm" style="text-decoration:none;">Back to course</a>
</div>

<div id="pg-sexp-panel">
  <textarea id="pg-sexp-input" rows="4" placeholder="Paste an S-expression here and click Load, or edit the proof tree directly..."></textarea>
  <div id="pg-sexp-actions">
    <button class="pg-btn pg-btn-primary pg-btn-sm" id="pg-load-btn">Load into editor</button>
    <button class="pg-btn pg-btn-sm" id="pg-sexp-close">Close</button>
    <span class="pg-hint">Ctrl/Cmd+Enter to load</span>
  </div>
</div>

<div id="pg-check-result"></div>

<div id="pg-rules-panel">
  <div id="pg-rules-header">
    <h3 id="pg-rules-title">Rules</h3>
    <button id="pg-rules-close">&times;</button>
  </div>
  <div id="pg-rules-body"></div>
</div>

<a id="pg-back" href="{{ '/' | relative_url }}">&#8592; Back to CIS 352</a>

<script>
(function() {
  // Make page immersive
  document.body.classList.add('pg-fullpage');

  var canvas = document.getElementById('pg-canvas');
  var toolbar = document.getElementById('pg-toolbar');
  var sexpPanel = document.getElementById('pg-sexp-panel');
  var sexpInput = document.getElementById('pg-sexp-input');
  var emptyMsg = document.getElementById('pg-empty');
  var theorySelect = document.getElementById('pg-theory');
  var examplesSelect = document.getElementById('pg-examples');
  var checkBtn = document.getElementById('pg-check-btn');
  var checkResult = document.getElementById('pg-check-result');

  var currentEditor = null;
  var checkTimeout = null;

  // ── Rule names and helpers from shared config ──
  function getTheoryConfig() {
    return ProofChecker.theoryConfig(theorySelect.value);
  }

  // ── Per-theory examples ─────────────────────────

  var theoryExamples = {
    '': [
      { label: 'Big-Step: {} \u22A2 (+ 3 5) \u21D3 8', key: 'bs-add' },
      { label: 'Sequent: P \u2227 Q \u21D2 Q \u2227 P', key: 'g3-swap' },
      { label: 'ND: P, P \u2192 Q \u22A2 Q', key: 'nd-mp' },
      { label: 'STLC: \u22A2 (\u03BB (x : int) x) : int \u2192 int', key: 'stlc-id' },
      { label: 'Small-Step: ((+ 1 2)) \u27F6 3', key: 'ss-add' },
    ],
    'big-step': [
      { label: 'Add: {} \u22A2 (+ 3 5) \u21D3 8', key: 'bs-add' },
      { label: 'Neg+Var: {x \u21A6 3} \u22A2 (- x) \u21D3 -3', key: 'bs-neg' },
      { label: 'App: {} \u22A2 ((\u03BB (x) (+ x 1)) 5) \u21D3 6', key: 'bs-app' },
      { label: 'If0: {} \u22A2 (if0 (+ 1 (- 1)) 42 0) \u21D3 42', key: 'bs-if0' },
      { label: 'Let+Closure: {} \u22A2 (let ([x 10]) ((\u03BB (y) (+ x y)) 7)) \u21D3 17', key: 'bs-let' },
    ],
    'small-step': [
      { label: 'Add: (+ 3 5) \u27F6 8', key: 'ss-add' },
      { label: 'Add nested: (+ (+ 1 2) 5) \u27F6* 8', key: 'ss-add-nested' },
      { label: 'Beta: ((\u03BB (x) x) 5) \u27F6 5', key: 'ss-beta' },
      { label: 'Neg: (- (+ 1 2)) \u27F6* -3', key: 'ss-neg' },
      { label: 'If0-True: (if0 0 1 2) \u27F6 1', key: 'ss-if0' },
    ],
    'g3ip': [
      { label: 'Identity: P \u21D2 P', key: 'g3-id' },
      { label: '\u2227-swap: P \u2227 Q \u21D2 Q \u2227 P', key: 'g3-swap' },
      { label: '\u2228-comm: P \u2228 Q \u21D2 Q \u2228 P', key: 'g3-or-comm' },
      { label: 'Modus Ponens: P, P \u2192 Q \u21D2 Q', key: 'g3-mp' },
      { label: '\u2192-intro: \u21D2 P \u2192 P', key: 'g3-imp-id' },
    ],
    'propnd': [
      { label: 'Modus Ponens: P, P \u2192 Q \u22A2 Q', key: 'nd-mp' },
      { label: '\u2227-intro: P, Q \u22A2 P \u2227 Q', key: 'nd-and-i' },
      { label: '\u2228-elim: P \u2228 Q, P \u2192 R, Q \u2192 R \u22A2 R', key: 'nd-or-e' },
      { label: '\u2192-intro: \u22A2 P \u2192 P', key: 'nd-imp-id' },
      { label: 'Contrapositive idea: P \u2192 Q, \u00ACQ \u22A2 \u00ACP', key: 'nd-contra' },
    ],
    'stlc': [
      { label: 'Identity: \u22A2 (\u03BB (x : int) x) : int \u2192 int', key: 'stlc-id' },
      { label: 'Const fn: \u22A2 (\u03BB (x : int) 42) : int \u2192 int', key: 'stlc-const' },
      { label: 'Application: \u22A2 ((\u03BB (x : int) x) 5) : int', key: 'stlc-app' },
      { label: 'Add: x : int \u22A2 (+ x 1) : int', key: 'stlc-add' },
      { label: 'Let: \u22A2 (let ([x 5]) (+ x 1)) : int', key: 'stlc-let' },
    ],
    'systemf': [
      { label: 'Poly id: \u22A2 (\u039B\u03B1. \u03BB (x : \u03B1) x) : \u2200\u03B1. \u03B1 \u2192 \u03B1', key: 'sf-poly-id' },
      { label: 'Type app: \u22A2 (\u039B\u03B1. \u03BB (x : \u03B1) x) [int] : int \u2192 int', key: 'sf-tyapp' },
      { label: 'Const: \u22A2 (\u039B\u03B1. \u039B\u03B2. \u03BB (x : \u03B1) (\u03BB (y : \u03B2) x)) : \u2200\u03B1. \u2200\u03B2. \u03B1 \u2192 \u03B2 \u2192 \u03B1', key: 'sf-const' },
    ],
  };

  // ── Proof tree S-expressions for each example ────

  var exampleData = {
    // Big-step
    'bs-add': '((Add :right) ((Int :right) --- "{} \u22A2 3 \u21D3 3") ((Int :right) --- "{} \u22A2 5 \u21D3 5") "v = 3 + 5" --- "{} \u22A2 (+ 3 5) \u21D3 8")',
    'bs-neg': '((Neg :right) ((Var :right) "{x \u21A6 3}(x) = 3" --- "{x \u21A6 3} \u22A2 x \u21D3 3") "v = -3" --- "{x \u21A6 3} \u22A2 (- x) \u21D3 -3")',
    'bs-app': '((App :right) ((Lam :right) --- "{} \u22A2 (\u03BB (x) (+ x 1)) \u21D3 \u27E8\u03BB (x) (+ x 1) , {}\u27E9") ((Int :right) --- "{} \u22A2 5 \u21D3 5") ((Add :right) ((Var :right) "{x \u21A6 5}(x) = 5" --- "{x \u21A6 5} \u22A2 x \u21D3 5") ((Int :right) --- "{x \u21A6 5} \u22A2 1 \u21D3 1") "v = 5 + 1" --- "{x \u21A6 5} \u22A2 (+ x 1) \u21D3 6") --- "{} \u22A2 ((\u03BB (x) (+ x 1)) 5) \u21D3 6")',
    'bs-if0': '(("If0-True" :right) ((Add :right) ((Int :right) --- "{} \u22A2 1 \u21D3 1") ((Neg :right) ((Int :right) --- "{} \u22A2 1 \u21D3 1") "v = -1" --- "{} \u22A2 (- 1) \u21D3 -1") "v = 1 + (-1)" --- "{} \u22A2 (+ 1 (- 1)) \u21D3 0") ((Int :right) --- "{} \u22A2 42 \u21D3 42") --- "{} \u22A2 (if0 (+ 1 (- 1)) 42 0) \u21D3 42")',
    'bs-let': '((Let :right) ((Int :right) --- "{} \u22A2 10 \u21D3 10") ((App :right) ((Lam :right) --- "{x \u21A6 10} \u22A2 (\u03BB (y) (+ x y)) \u21D3 \u27E8\u03BB (y) (+ x y) , {x \u21A6 10}\u27E9") ((Int :right) --- "{x \u21A6 10} \u22A2 7 \u21D3 7") ((Add :right) ((Var :right) "{x \u21A6 10, y \u21A6 7}(x) = 10" --- "{x \u21A6 10, y \u21A6 7} \u22A2 x \u21D3 10") ((Var :right) "{x \u21A6 10, y \u21A6 7}(y) = 7" --- "{x \u21A6 10, y \u21A6 7} \u22A2 y \u21D3 7") "v = 10 + 7" --- "{x \u21A6 10, y \u21A6 7} \u22A2 (+ x y) \u21D3 17") --- "{x \u21A6 10} \u22A2 ((\u03BB (y) (+ x y)) 7) \u21D3 17") --- "{} \u22A2 (let ([x 10]) ((\u03BB (y) (+ x y)) 7)) \u21D3 17")',

    // Small-step
    'ss-add': '((Add :right) --- "(+ 3 5) \u27F6 8")',
    'ss-add-nested': '(("Add-L" :right) ((Add :right) --- "(+ 1 2) \u27F6 3") --- "(+ (+ 1 2) 5) \u27F6 (+ 3 5)")',
    'ss-beta': '((Beta :right) --- "((\u03BB (x) x) 5) \u27F6 5")',
    'ss-neg': '(("Neg-Step" :right) ((Add :right) --- "(+ 1 2) \u27F6 3") --- "(- (+ 1 2)) \u27F6 (- 3)")',
    'ss-if0': '(("If0-True" :right) --- "(if0 0 1 2) \u27F6 1")',

    // G3ip sequent calculus
    'g3-id': '((Ax :right) --- "P \u21D2 P")',
    'g3-swap': '(("\u2227R" :right) (("\u2227L" :right) ((Ax :right) --- "P, Q \u21D2 Q") --- "P \u2227 Q \u21D2 Q") (("\u2227L" :right) ((Ax :right) --- "P, Q \u21D2 P") --- "P \u2227 Q \u21D2 P") --- "P \u2227 Q \u21D2 Q \u2227 P")',
    'g3-or-comm': '(("\u2228L" :right) (("\u2228R\u2082" :right) ((Ax :right) --- "P \u21D2 P") --- "P \u21D2 Q \u2228 P") (("\u2228R\u2081" :right) ((Ax :right) --- "Q \u21D2 Q") --- "Q \u21D2 Q \u2228 P") --- "P \u2228 Q \u21D2 Q \u2228 P")',
    'g3-mp': '(("\u2192L" :right) ((Ax :right) --- "P \u21D2 P") ((Ax :right) --- "Q \u21D2 Q") --- "P, P \u2192 Q \u21D2 Q")',
    'g3-imp-id': '(("\u2192R" :right) ((Ax :right) --- "P \u21D2 P") --- "\u21D2 P \u2192 P")',

    // Propositional natural deduction
    'nd-mp': '(("\u2192E" :right) ((Ax :right) --- "P, P \u2192 Q \u22A2 P \u2192 Q") ((Ax :right) --- "P, P \u2192 Q \u22A2 P") --- "P, P \u2192 Q \u22A2 Q")',
    'nd-and-i': '(("\u2227I" :right) ((Ax :right) --- "P, Q \u22A2 P") ((Ax :right) --- "P, Q \u22A2 Q") --- "P, Q \u22A2 P \u2227 Q")',
    'nd-or-e': '(("\u2228E" :right) ((Ax :right) --- "P \u2228 Q, P \u2192 R, Q \u2192 R \u22A2 P \u2228 Q") (("\u2192E" :right) ((Ax :right) --- "P, P \u2192 R, Q \u2192 R \u22A2 P \u2192 R") ((Ax :right) --- "P, P \u2192 R, Q \u2192 R \u22A2 P") --- "P, P \u2192 R, Q \u2192 R \u22A2 R") (("\u2192E" :right) ((Ax :right) --- "Q, P \u2192 R, Q \u2192 R \u22A2 Q \u2192 R") ((Ax :right) --- "Q, P \u2192 R, Q \u2192 R \u22A2 Q") --- "Q, P \u2192 R, Q \u2192 R \u22A2 R") --- "P \u2228 Q, P \u2192 R, Q \u2192 R \u22A2 R")',
    'nd-imp-id': '(("\u2192I" :right) ((Ax :right) --- "P \u22A2 P") --- "\u22A2 P \u2192 P")',
    'nd-contra': '(("\u00ACI" :right) (("\u00ACE" :right) ((Ax :right) --- "P, P \u2192 Q, \u00ACQ \u22A2 \u00ACQ") (("\u2192E" :right) ((Ax :right) --- "P, P \u2192 Q, \u00ACQ \u22A2 P \u2192 Q") ((Ax :right) --- "P, P \u2192 Q, \u00ACQ \u22A2 P") --- "P, P \u2192 Q, \u00ACQ \u22A2 Q") --- "P, P \u2192 Q, \u00ACQ \u22A2 \u22A5") --- "P \u2192 Q, \u00ACQ \u22A2 \u00ACP")',

    // STLC
    'stlc-id': '(("T-Lam" :right) (("T-Var" :right) --- "x : int \u22A2 x : int") --- "\u22A2 (\u03BB (x : int) x) : int \u2192 int")',
    'stlc-const': '(("T-Lam" :right) (("T-Int" :right) --- "x : int \u22A2 42 : int") --- "\u22A2 (\u03BB (x : int) 42) : int \u2192 int")',
    'stlc-app': '(("T-App" :right) (("T-Lam" :right) (("T-Var" :right) --- "x : int \u22A2 x : int") --- "\u22A2 (\u03BB (x : int) x) : int \u2192 int") (("T-Int" :right) --- "\u22A2 5 : int") --- "\u22A2 ((\u03BB (x : int) x) 5) : int")',
    'stlc-add': '(("T-Add" :right) (("T-Var" :right) --- "x : int \u22A2 x : int") (("T-Int" :right) --- "x : int \u22A2 1 : int") --- "x : int \u22A2 (+ x 1) : int")',
    'stlc-let': '(("T-Let" :right) (("T-Int" :right) --- "\u22A2 5 : int") (("T-Add" :right) (("T-Var" :right) --- "x : int \u22A2 x : int") (("T-Int" :right) --- "x : int \u22A2 1 : int") --- "x : int \u22A2 (+ x 1) : int") --- "\u22A2 (let ([x 5]) (+ x 1)) : int")',

    // System F
    'sf-poly-id': '(("T-TyLam" :right) (("T-Lam" :right) (("T-Var" :right) --- "x : \u03B1 \u22A2 x : \u03B1") --- "\u22A2 (\u03BB (x : \u03B1) x) : \u03B1 \u2192 \u03B1") --- "\u22A2 (\u039B\u03B1. \u03BB (x : \u03B1) x) : \u2200\u03B1. \u03B1 \u2192 \u03B1")',
    'sf-tyapp': '(("T-TyApp" :right) (("T-TyLam" :right) (("T-Lam" :right) (("T-Var" :right) --- "x : \u03B1 \u22A2 x : \u03B1") --- "\u22A2 (\u03BB (x : \u03B1) x) : \u03B1 \u2192 \u03B1") --- "\u22A2 (\u039B\u03B1. \u03BB (x : \u03B1) x) : \u2200\u03B1. \u03B1 \u2192 \u03B1") --- "\u22A2 (\u039B\u03B1. \u03BB (x : \u03B1) x) [int] : int \u2192 int")',
    'sf-const': '(("T-TyLam" :right) (("T-TyLam" :right) (("T-Lam" :right) (("T-Lam" :right) (("T-Var" :right) --- "x : \u03B1, y : \u03B2 \u22A2 x : \u03B1") --- "x : \u03B1 \u22A2 (\u03BB (y : \u03B2) x) : \u03B2 \u2192 \u03B1") --- "\u22A2 (\u03BB (x : \u03B1) (\u03BB (y : \u03B2) x)) : \u03B1 \u2192 \u03B2 \u2192 \u03B1") --- "\u22A2 (\u039B\u03B2. \u03BB (x : \u03B1) (\u03BB (y : \u03B2) x)) : \u2200\u03B2. \u03B1 \u2192 \u03B2 \u2192 \u03B1") --- "\u22A2 (\u039B\u03B1. \u039B\u03B2. \u03BB (x : \u03B1) (\u03BB (y : \u03B2) x)) : \u2200\u03B1. \u2200\u03B2. \u03B1 \u2192 \u03B2 \u2192 \u03B1")',
  };

  // ── Per-theory rules reference ────────────────────

  var rulesBtn = document.getElementById('pg-rules-btn');
  var rulesPanel = document.getElementById('pg-rules-panel');
  var rulesTitle = document.getElementById('pg-rules-title');
  var rulesBody = document.getElementById('pg-rules-body');

  var theoryRules = {
    'big-step': {
      title: 'Big-Step Semantics',
      judgement: '\u03C1 \u22A2 e \u21D3 v',
      rules: [
        { name: 'Int', premises: [], conclusion: '\u03C1 \u22A2 n \u21D3 n' },
        { name: 'Var', premises: [], conclusion: '\u03C1 \u22A2 x \u21D3 v', condition: '\u03C1(x) = v' },
        { name: 'Lam', premises: [], conclusion: '\u03C1 \u22A2 (\u03BB (x) e) \u21D3 \u27E8\u03BB (x) e, \u03C1\u27E9' },
        { name: 'Add', premises: ['\u03C1 \u22A2 e\u2081 \u21D3 v\u2081', '\u03C1 \u22A2 e\u2082 \u21D3 v\u2082'], conclusion: '\u03C1 \u22A2 (+ e\u2081 e\u2082) \u21D3 v\u2083', condition: 'v\u2083 = v\u2081 + v\u2082' },
        { name: 'Neg', premises: ['\u03C1 \u22A2 e \u21D3 v'], conclusion: '\u03C1 \u22A2 (- e) \u21D3 v\u2032', condition: 'v\u2032 = \u2212v' },
        { name: 'App', premises: ['\u03C1 \u22A2 e\u2081 \u21D3 \u27E8\u03BB (x) e, \u03C1\u2032\u27E9', '\u03C1 \u22A2 e\u2082 \u21D3 v\u2082', '\u03C1\u2032[x\u21A6v\u2082] \u22A2 e \u21D3 v'], conclusion: '\u03C1 \u22A2 (e\u2081 e\u2082) \u21D3 v' },
        { name: 'If0-True', premises: ['\u03C1 \u22A2 e\u2081 \u21D3 0', '\u03C1 \u22A2 e\u2082 \u21D3 v'], conclusion: '\u03C1 \u22A2 (if0 e\u2081 e\u2082 e\u2083) \u21D3 v' },
        { name: 'If0-False', premises: ['\u03C1 \u22A2 e\u2081 \u21D3 v\u2081', '\u03C1 \u22A2 e\u2083 \u21D3 v'], conclusion: '\u03C1 \u22A2 (if0 e\u2081 e\u2082 e\u2083) \u21D3 v', condition: 'v\u2081 \u2260 0' },
        { name: 'Let', premises: ['\u03C1 \u22A2 e\u2081 \u21D3 v\u2081', '\u03C1[x\u21A6v\u2081] \u22A2 e\u2082 \u21D3 v'], conclusion: '\u03C1 \u22A2 (let ([x e\u2081]) e\u2082) \u21D3 v' },
      ]
    },
    'small-step': {
      title: 'Small-Step Semantics',
      judgement: 'e \u27F6 e\u2032',
      rules: [
        { name: 'Beta', premises: [], conclusion: '((\u03BB (x) e) v) \u27F6 e[x := v]' },
        { name: 'App-L', premises: ['e\u2081 \u27F6 e\u2081\u2032'], conclusion: '(e\u2081 e\u2082) \u27F6 (e\u2081\u2032 e\u2082)' },
        { name: 'App-R', premises: ['e\u2082 \u27F6 e\u2082\u2032'], conclusion: '(v e\u2082) \u27F6 (v e\u2082\u2032)' },
        { name: 'Add-L', premises: ['e\u2081 \u27F6 e\u2081\u2032'], conclusion: '(+ e\u2081 e\u2082) \u27F6 (+ e\u2081\u2032 e\u2082)' },
        { name: 'Add-R', premises: ['e\u2082 \u27F6 e\u2082\u2032'], conclusion: '(+ v e\u2082) \u27F6 (+ v e\u2082\u2032)' },
        { name: 'Add', premises: [], conclusion: '(+ n\u2081 n\u2082) \u27F6 n\u2083', condition: 'n\u2083 = n\u2081 + n\u2082' },
        { name: 'Neg-Step', premises: ['e \u27F6 e\u2032'], conclusion: '(- e) \u27F6 (- e\u2032)' },
        { name: 'Neg', premises: [], conclusion: '(- n) \u27F6 \u2212n' },
        { name: 'If0-Step', premises: ['e \u27F6 e\u2032'], conclusion: '(if0 e e\u2082 e\u2083) \u27F6 (if0 e\u2032 e\u2082 e\u2083)' },
        { name: 'If0-True', premises: [], conclusion: '(if0 0 e\u2082 e\u2083) \u27F6 e\u2082' },
        { name: 'If0-False', premises: [], conclusion: '(if0 n e\u2082 e\u2083) \u27F6 e\u2083', condition: 'n \u2260 0' },
        { name: 'Let-Step', premises: ['e \u27F6 e\u2032'], conclusion: '(let ([x e]) e\u2082) \u27F6 (let ([x e\u2032]) e\u2082)' },
        { name: 'Let', premises: [], conclusion: '(let ([x v]) e) \u27F6 e[x := v]' },
      ]
    },
    'g3ip': {
      title: 'G3ip Sequent Calculus',
      judgement: '\u0393 \u21D2 C',
      rules: [
        { name: 'Ax', premises: [], conclusion: 'P, \u0393 \u21D2 P', condition: 'P atomic' },
        { name: '\u22A5L', premises: [], conclusion: '\u22A5, \u0393 \u21D2 C' },
        { name: '\u22A4R', premises: [], conclusion: '\u0393 \u21D2 \u22A4' },
        { name: '\u2227R', premises: ['\u0393 \u21D2 A', '\u0393 \u21D2 B'], conclusion: '\u0393 \u21D2 A \u2227 B' },
        { name: '\u2227L', premises: ['A, B, \u0393 \u21D2 C'], conclusion: 'A \u2227 B, \u0393 \u21D2 C' },
        { name: '\u2228R\u2081', premises: ['\u0393 \u21D2 A'], conclusion: '\u0393 \u21D2 A \u2228 B' },
        { name: '\u2228R\u2082', premises: ['\u0393 \u21D2 B'], conclusion: '\u0393 \u21D2 A \u2228 B' },
        { name: '\u2228L', premises: ['A, \u0393 \u21D2 C', 'B, \u0393 \u21D2 C'], conclusion: 'A \u2228 B, \u0393 \u21D2 C' },
        { name: '\u2192R', premises: ['A, \u0393 \u21D2 B'], conclusion: '\u0393 \u21D2 A \u2192 B' },
        { name: '\u2192L', premises: ['\u0393 \u21D2 A', 'B, \u0393 \u21D2 C'], conclusion: 'A \u2192 B, \u0393 \u21D2 C' },
      ]
    },
    'propnd': {
      title: 'Natural Deduction',
      judgement: '\u0393 \u22A2 A',
      rules: [
        { name: 'Ax', premises: [], conclusion: '\u0393 \u22A2 A', condition: 'A \u2208 \u0393' },
        { name: '\u2192I', premises: ['\u0393, A \u22A2 B'], conclusion: '\u0393 \u22A2 A \u2192 B' },
        { name: '\u2192E', premises: ['\u0393 \u22A2 A \u2192 B', '\u0393 \u22A2 A'], conclusion: '\u0393 \u22A2 B' },
        { name: '\u2227I', premises: ['\u0393 \u22A2 A', '\u0393 \u22A2 B'], conclusion: '\u0393 \u22A2 A \u2227 B' },
        { name: '\u2227E\u2081', premises: ['\u0393 \u22A2 A \u2227 B'], conclusion: '\u0393 \u22A2 A' },
        { name: '\u2227E\u2082', premises: ['\u0393 \u22A2 A \u2227 B'], conclusion: '\u0393 \u22A2 B' },
        { name: '\u2228I\u2081', premises: ['\u0393 \u22A2 A'], conclusion: '\u0393 \u22A2 A \u2228 B' },
        { name: '\u2228I\u2082', premises: ['\u0393 \u22A2 B'], conclusion: '\u0393 \u22A2 A \u2228 B' },
        { name: '\u2228E', premises: ['\u0393 \u22A2 A \u2228 B', '\u0393, A \u22A2 C', '\u0393, B \u22A2 C'], conclusion: '\u0393 \u22A2 C' },
        { name: '\u22A5E', premises: ['\u0393 \u22A2 \u22A5'], conclusion: '\u0393 \u22A2 A' },
        { name: '\u00ACI', premises: ['\u0393, A \u22A2 \u22A5'], conclusion: '\u0393 \u22A2 \u00ACA' },
        { name: '\u00ACE', premises: ['\u0393 \u22A2 \u00ACA', '\u0393 \u22A2 A'], conclusion: '\u0393 \u22A2 \u22A5' },
      ]
    },
    'stlc': {
      title: 'Simply-Typed \u03BB-Calculus',
      judgement: '\u0393 \u22A2 e : \u03C4',
      rules: [
        { name: 'T-Var', premises: [], conclusion: '\u0393 \u22A2 x : \u03C4', condition: '(x : \u03C4) \u2208 \u0393' },
        { name: 'T-Int', premises: [], conclusion: '\u0393 \u22A2 n : int' },
        { name: 'T-Bool', premises: [], conclusion: '\u0393 \u22A2 b : bool' },
        { name: 'T-Lam', premises: ['\u0393, x:\u03C4\u2081 \u22A2 e : \u03C4\u2082'], conclusion: '\u0393 \u22A2 (\u03BB (x : \u03C4\u2081) e) : \u03C4\u2081 \u2192 \u03C4\u2082' },
        { name: 'T-App', premises: ['\u0393 \u22A2 e\u2081 : \u03C4\u2081 \u2192 \u03C4\u2082', '\u0393 \u22A2 e\u2082 : \u03C4\u2081'], conclusion: '\u0393 \u22A2 (e\u2081 e\u2082) : \u03C4\u2082' },
        { name: 'T-Add', premises: ['\u0393 \u22A2 e\u2081 : int', '\u0393 \u22A2 e\u2082 : int'], conclusion: '\u0393 \u22A2 (+ e\u2081 e\u2082) : int' },
        { name: 'T-Neg', premises: ['\u0393 \u22A2 e : int'], conclusion: '\u0393 \u22A2 (- e) : int' },
        { name: 'T-If', premises: ['\u0393 \u22A2 e\u2081 : int', '\u0393 \u22A2 e\u2082 : \u03C4', '\u0393 \u22A2 e\u2083 : \u03C4'], conclusion: '\u0393 \u22A2 (if0 e\u2081 e\u2082 e\u2083) : \u03C4' },
        { name: 'T-Let', premises: ['\u0393 \u22A2 e\u2081 : \u03C4\u2081', '\u0393, x:\u03C4\u2081 \u22A2 e\u2082 : \u03C4\u2082'], conclusion: '\u0393 \u22A2 (let ([x e\u2081]) e\u2082) : \u03C4\u2082' },
      ]
    },
    'systemf': {
      title: 'System F',
      judgement: '\u0393 \u22A2 e : \u03C4',
      rules: [
        { name: 'T-Var', premises: [], conclusion: '\u0393 \u22A2 x : \u03C4', condition: '(x : \u03C4) \u2208 \u0393' },
        { name: 'T-Int', premises: [], conclusion: '\u0393 \u22A2 n : int' },
        { name: 'T-Bool', premises: [], conclusion: '\u0393 \u22A2 b : bool' },
        { name: 'T-Lam', premises: ['\u0393, x:\u03C4\u2081 \u22A2 e : \u03C4\u2082'], conclusion: '\u0393 \u22A2 (\u03BB (x : \u03C4\u2081) e) : \u03C4\u2081 \u2192 \u03C4\u2082' },
        { name: 'T-App', premises: ['\u0393 \u22A2 e\u2081 : \u03C4\u2081 \u2192 \u03C4\u2082', '\u0393 \u22A2 e\u2082 : \u03C4\u2081'], conclusion: '\u0393 \u22A2 (e\u2081 e\u2082) : \u03C4\u2082' },
        { name: 'T-Add', premises: ['\u0393 \u22A2 e\u2081 : int', '\u0393 \u22A2 e\u2082 : int'], conclusion: '\u0393 \u22A2 (+ e\u2081 e\u2082) : int' },
        { name: 'T-Neg', premises: ['\u0393 \u22A2 e : int'], conclusion: '\u0393 \u22A2 (- e) : int' },
        { name: 'T-If', premises: ['\u0393 \u22A2 e\u2081 : int', '\u0393 \u22A2 e\u2082 : \u03C4', '\u0393 \u22A2 e\u2083 : \u03C4'], conclusion: '\u0393 \u22A2 (if0 e\u2081 e\u2082 e\u2083) : \u03C4' },
        { name: 'T-Let', premises: ['\u0393 \u22A2 e\u2081 : \u03C4\u2081', '\u0393, x:\u03C4\u2081 \u22A2 e\u2082 : \u03C4\u2082'], conclusion: '\u0393 \u22A2 (let ([x e\u2081]) e\u2082) : \u03C4\u2082' },
        { name: 'T-TyLam', premises: ['\u0393 \u22A2 e : \u03C4'], conclusion: '\u0393 \u22A2 (\u039B\u03B1. e) : \u2200\u03B1. \u03C4' },
        { name: 'T-TyApp', premises: ['\u0393 \u22A2 e : \u2200\u03B1. \u03C4'], conclusion: '\u0393 \u22A2 e [\u03C4\u2032] : \u03C4[\u03B1 := \u03C4\u2032]' },
      ]
    },
  };

  function renderRules(theory) {
    var data = theoryRules[theory];
    if (!data) {
      rulesBody.innerHTML = '<div class="pg-rules-empty">Select a theory to see its rules.</div>';
      rulesTitle.textContent = 'Rules';
      return;
    }
    rulesTitle.textContent = data.title;
    var html = '<div class="pg-rules-judgement">Judgement form: <code>' + data.judgement + '</code></div>';
    for (var i = 0; i < data.rules.length; i++) {
      var r = data.rules[i];
      html += '<div class="pg-rule">';
      html += '<div class="pg-rule-body">';
      html += '<div class="pg-rule-premises">';
      if (r.premises.length === 0) {
        html += '&nbsp;';
      } else {
        for (var j = 0; j < r.premises.length; j++) {
          html += '<span>' + esc(r.premises[j]) + '</span>';
        }
      }
      html += '</div>';
      html += '<div class="pg-rule-line"></div>';
      html += '<div class="pg-rule-conclusion">' + esc(r.conclusion) + '</div>';
      if (r.condition) {
        html += '<div class="pg-rule-condition">' + esc(r.condition) + '</div>';
      }
      html += '</div>';
      html += '<div class="pg-rule-name">' + esc(r.name) + '</div>';
      html += '</div>';
    }
    rulesBody.innerHTML = html;
  }

  function esc(s) {
    var d = document.createElement('div');
    d.textContent = s;
    return d.innerHTML;
  }

  rulesBtn.addEventListener('click', function() {
    var theory = theorySelect.value;
    renderRules(theory);
    rulesPanel.classList.toggle('open');
  });
  document.getElementById('pg-rules-close').addEventListener('click', function() {
    rulesPanel.classList.remove('open');
  });

  // ── Show toolbar on mouse near top ──────────────
  var toolbarTimer = null;
  document.addEventListener('mousemove', function(e) {
    if (e.clientY < 60 || sexpPanel.classList.contains('open')) {
      toolbar.classList.add('visible');
      clearTimeout(toolbarTimer);
    } else {
      clearTimeout(toolbarTimer);
      toolbarTimer = setTimeout(function() {
        if (!sexpPanel.classList.contains('open')) {
          toolbar.classList.remove('visible');
        }
      }, 1500);
    }
  });
  // Show toolbar on touch at top
  document.addEventListener('touchstart', function(e) {
    var touch = e.touches[0];
    if (touch && touch.clientY < 60) {
      toolbar.classList.add('visible');
    }
  });

  // ── Theory selection → update examples dropdown ──
  function updateExamples() {
    var theory = theorySelect.value;
    var list = theoryExamples[theory] || [];
    examplesSelect.innerHTML = '<option value="">Load example...</option>';
    for (var i = 0; i < list.length; i++) {
      var opt = document.createElement('option');
      opt.value = list[i].key;
      opt.textContent = list[i].label;
      examplesSelect.appendChild(opt);
    }
    // Show/hide check + rules buttons
    checkBtn.style.display = theory ? '' : 'none';
    rulesBtn.style.display = theory ? '' : 'none';
    hideCheckResult();
    // Update rules panel if open
    if (rulesPanel.classList.contains('open')) {
      if (theory) { renderRules(theory); } else { rulesPanel.classList.remove('open'); }
    }
  }

  theorySelect.addEventListener('change', function() {
    updateExamples();
    if (currentEditor) {
      var cfg = getTheoryConfig();
      currentEditor.setTheoryRules(cfg.theoryRules, cfg.onGeneratePremises, cfg.onCheckApplicability);
    }
  });
  updateExamples(); // init

  // ── Load proof into editor ──────────────────────
  function loadProof(sexp) {
    if (emptyMsg) { emptyMsg.remove(); emptyMsg = null; }
    if (currentEditor) { currentEditor.destroy(); currentEditor = null; }
    canvas.innerHTML = '';
    hideCheckResult();

    try {
      var loadCfg = getTheoryConfig();
      currentEditor = ProofTree.createEditor(canvas, {
        tree: sexp,
        zoom: true,
        showLigatureHints: true,
        theoryRules: loadCfg.theoryRules,
        onGeneratePremises: loadCfg.onGeneratePremises,
        onCheckApplicability: loadCfg.onCheckApplicability,
        onChange: function(tree) {
          var s = ProofTree.toSexp(tree);
          sexpInput.value = s;
          history.replaceState(null, '', '#proof=' + encodeURIComponent(s));
          hideCheckResult();
        }
      });
      sexpInput.value = sexp;
      resetBtn.style.display = '';
      history.replaceState(null, '', '#proof=' + encodeURIComponent(sexp));
    } catch(e) {
      canvas.innerHTML = '<div style="color:#c00;padding:40px;text-align:center;">Error: ' + e.message + '</div>';
    }
  }

  function newProof() {
    if (emptyMsg) { emptyMsg.remove(); emptyMsg = null; }
    if (currentEditor) { currentEditor.destroy(); currentEditor = null; }
    canvas.innerHTML = '';
    hideCheckResult();
    var newCfg = getTheoryConfig();
    currentEditor = ProofTree.createEditor(canvas, {
      zoom: true,
      showLigatureHints: true,
      theoryRules: newCfg.theoryRules,
      onGeneratePremises: newCfg.onGeneratePremises,
      onCheckApplicability: newCfg.onCheckApplicability,
      onChange: function(tree) {
        var s = ProofTree.toSexp(tree);
        sexpInput.value = s;
        history.replaceState(null, '', '#proof=' + encodeURIComponent(s));
        hideCheckResult();
      }
    });
    resetBtn.style.display = '';
    sexpInput.value = '';
    history.replaceState(null, '', window.location.pathname);
  }

  // ── Check result display ───────────────────────
  function showCheckResult(result) {
    checkResult.classList.remove('valid', 'error', 'incomplete');

    var errorCount = 0;
    var incompleteCount = 0;
    for (var i = 0; i < result.diagnostics.length; i++) {
      if (result.diagnostics[i].level === 'error') errorCount++;
      else if (result.diagnostics[i].level === 'incomplete') incompleteCount++;
    }

    if (result.valid) {
      checkResult.classList.add('valid');
      checkResult.innerHTML = '\u2713 Correct!';
    } else if (errorCount > 0) {
      checkResult.classList.add('error');
      var msgs = result.diagnostics
        .filter(function(d) { return d.level === 'error'; })
        .map(function(d) { return d.message; });
      checkResult.innerHTML = '\u2717 ' + errorCount + ' error' + (errorCount > 1 ? 's' : '') + ': ' + msgs[0];
      if (msgs.length > 1) {
        checkResult.innerHTML += ' (+' + (msgs.length - 1) + ' more)';
      }
    } else if (incompleteCount > 0) {
      checkResult.classList.add('incomplete');
      checkResult.innerHTML = '\u25CF ' + incompleteCount + ' node' + (incompleteCount > 1 ? 's' : '') + ' not yet filled in';
    }

    checkResult.classList.add('visible');
    clearTimeout(checkTimeout);
    checkTimeout = setTimeout(hideCheckResult, 8000);
  }

  function hideCheckResult() {
    checkResult.classList.remove('visible');
    clearTimeout(checkTimeout);
  }

  // ── Check button ──────────────────────────────
  checkBtn.addEventListener('click', function() {
    if (!currentEditor) return;
    var theory = theorySelect.value;
    if (!theory) return;

    ProofChecker.init().then(function() {
      var result = ProofChecker.check(currentEditor, theory);
      ProofChecker.annotate(canvas, result);
      showCheckResult(result);
    });
  });

  // ── S-expression panel toggle ───────────────────
  document.getElementById('pg-sexp-toggle').addEventListener('click', function() {
    sexpPanel.classList.toggle('open');
    if (sexpPanel.classList.contains('open')) {
      // Sync current tree to textarea
      if (currentEditor) {
        sexpInput.value = currentEditor.getSexp();
      }
      setTimeout(function() { sexpInput.focus(); }, 300);
    }
  });
  document.getElementById('pg-sexp-close').addEventListener('click', function() {
    sexpPanel.classList.remove('open');
  });

  // ── Load from textarea ──────────────────────────
  document.getElementById('pg-load-btn').addEventListener('click', function() {
    var sexp = sexpInput.value.trim();
    if (sexp) {
      loadProof(sexp);
      sexpPanel.classList.remove('open');
    }
  });
  sexpInput.addEventListener('keydown', function(e) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
      e.preventDefault();
      document.getElementById('pg-load-btn').click();
    }
  });

  // ── Examples dropdown ───────────────────────────
  var _exampleTheoryMap = {
    'bs-': 'big-step', 'ss-': 'small-step', 'g3-': 'g3ip',
    'nd-': 'propnd', 'stlc-': 'stlc', 'sf-': 'systemf',
  };

  examplesSelect.addEventListener('change', function() {
    var key = this.value;
    if (key && exampleData[key]) {
      // Auto-select matching theory if currently on freestyle
      if (!theorySelect.value) {
        for (var prefix in _exampleTheoryMap) {
          if (key.indexOf(prefix) === 0) {
            theorySelect.value = _exampleTheoryMap[prefix];
            updateExamples();
            break;
          }
        }
      }
      loadProof(exampleData[key]);
      sexpPanel.classList.remove('open');
    }
    this.value = '';
  });

  // ── New button ──────────────────────────────────
  document.getElementById('pg-new-btn').addEventListener('click', newProof);

  // ── Reset button ────────────────────────────────
  var resetBtn = document.getElementById('pg-reset-btn');
  resetBtn.addEventListener('click', function() {
    if (!currentEditor) return;
    if (!confirm('Reset this proof tree? All progress will be lost.')) return;
    // Get the current conclusion from the root node
    var tree = currentEditor.getTree();
    var rootConclusion = tree ? (tree.conclusion || '') : '';
    if (currentEditor) { currentEditor.destroy(); currentEditor = null; }
    canvas.innerHTML = '';
    hideCheckResult();
    var resetCfg = getTheoryConfig();
    currentEditor = ProofTree.createEditor(canvas, {
      tree: '(--- "' + rootConclusion.replace(/\\/g, '\\\\').replace(/"/g, '\\"') + '")',
      zoom: true,
      showLigatureHints: true,
      theoryRules: resetCfg.theoryRules,
      onGeneratePremises: resetCfg.onGeneratePremises,
      onCheckApplicability: resetCfg.onCheckApplicability,
      onChange: function(tree) {
        var s = ProofTree.toSexp(tree);
        sexpInput.value = s;
        history.replaceState(null, '', '#proof=' + encodeURIComponent(s));
        hideCheckResult();
      }
    });
    var s = currentEditor.getSexp();
    sexpInput.value = s;
    history.replaceState(null, '', '#proof=' + encodeURIComponent(s));
  });

  // ── Copy button ─────────────────────────────────
  document.getElementById('pg-copy-btn').addEventListener('click', function() {
    var btn = this;
    var sexp = currentEditor ? currentEditor.getSexp() : '';
    if (!sexp) return;
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(sexp).then(function() {
        btn.textContent = 'Copied!';
        setTimeout(function() { btn.textContent = 'Copy'; }, 1500);
      });
    } else {
      var ta = document.createElement('textarea');
      ta.value = sexp;
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      document.execCommand('copy');
      document.body.removeChild(ta);
      btn.textContent = 'Copied!';
      setTimeout(function() { btn.textContent = 'Copy'; }, 1500);
    }
  });

  // ── Load from URL hash on page load ─────────────
  if (window.location.hash) {
    var match = window.location.hash.match(/^#proof=(.+)/);
    if (match) {
      var sexp = decodeURIComponent(match[1]);
      setTimeout(function() { loadProof(sexp); }, 100);
    }
  }
})();
</script>
