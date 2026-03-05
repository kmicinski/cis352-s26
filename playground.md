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
.pg-btn-sm { padding: 3px 10px; font-size: 0.78em; }
#pg-examples {
  padding: 5px 8px;
  font-size: 0.82em;
  font-family: inherit;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  background: #fff;
  color: #374151;
  max-width: 240px;
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
    <p>Click the conclusion below to start editing.</p>
  </div>
</div>

<div id="pg-toolbar">
  <h2>Playground</h2>
  <select id="pg-examples">
    <option value="">Load example...</option>
    <option value="add">Add: {} ⊢ (+ 3 5) ⇓ 8</option>
    <option value="neg">Neg+Var: {x ↦ 3} ⊢ (- x) ⇓ -3</option>
    <option value="app">App: {} ⊢ ((λ (x) (+ x 1)) 5) ⇓ 6</option>
    <option value="if0">If0: {} ⊢ (if0 (+ 1 (- 1)) 42 0) ⇓ 42</option>
    <option value="let-closure">Let+Closure: {} ⊢ (let ([x 10]) ((λ (y) (+ x y)) 7)) ⇓ 17</option>
  </select>
  <button class="pg-btn pg-btn-sm" id="pg-sexp-toggle">S-expression</button>
  <button class="pg-btn pg-btn-sm" id="pg-new-btn">New</button>
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

<a id="pg-back" href="{{ '/' | relative_url }}">← Back to CIS 352</a>

<script>
(function() {
  // Make page immersive
  document.body.classList.add('pg-fullpage');

  var canvas = document.getElementById('pg-canvas');
  var toolbar = document.getElementById('pg-toolbar');
  var sexpPanel = document.getElementById('pg-sexp-panel');
  var sexpInput = document.getElementById('pg-sexp-input');
  var emptyMsg = document.getElementById('pg-empty');

  var currentEditor = null;

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

  // ── Example proof trees ─────────────────────────
  var examples = {
    'add': '((Add :right) ((Int :right) --- "{} ⊢ 3 ⇓ 3") ((Int :right) --- "{} ⊢ 5 ⇓ 5") "v = 3 + 5" --- "{} ⊢ (+ 3 5) ⇓ 8")',
    'neg': '((Neg :right) ((Var :right) "{x ↦ 3}(x) = 3" --- "{x ↦ 3} ⊢ x ⇓ 3") "v = -3" --- "{x ↦ 3} ⊢ (- x) ⇓ -3")',
    'app': '((App :right) ((Lam :right) --- "{} ⊢ (λ (x) (+ x 1)) ⇓ ⟨λ (x) (+ x 1) , {}⟩") ((Int :right) --- "{} ⊢ 5 ⇓ 5") ((Add :right) ((Var :right) "{x ↦ 5}(x) = 5" --- "{x ↦ 5} ⊢ x ⇓ 5") ((Int :right) --- "{x ↦ 5} ⊢ 1 ⇓ 1") "v = 5 + 1" --- "{x ↦ 5} ⊢ (+ x 1) ⇓ 6") --- "{} ⊢ ((λ (x) (+ x 1)) 5) ⇓ 6")',
    'if0': '(("If0-True" :right) ((Add :right) ((Int :right) --- "{} ⊢ 1 ⇓ 1") ((Neg :right) ((Int :right) --- "{} ⊢ 1 ⇓ 1") "v = -1" --- "{} ⊢ (- 1) ⇓ -1") "v = 1 + (-1)" --- "{} ⊢ (+ 1 (- 1)) ⇓ 0") ((Int :right) --- "{} ⊢ 42 ⇓ 42") --- "{} ⊢ (if0 (+ 1 (- 1)) 42 0) ⇓ 42")',
    'let-closure': '((Let :right) ((Int :right) --- "{} ⊢ 10 ⇓ 10") ((App :right) ((Lam :right) --- "{x ↦ 10} ⊢ (λ (y) (+ x y)) ⇓ ⟨λ (y) (+ x y) , {x ↦ 10}⟩") ((Int :right) --- "{x ↦ 10} ⊢ 7 ⇓ 7") ((Add :right) ((Var :right) "{x ↦ 10, y ↦ 7}(x) = 10" --- "{x ↦ 10, y ↦ 7} ⊢ x ⇓ 10") ((Var :right) "{x ↦ 10, y ↦ 7}(y) = 7" --- "{x ↦ 10, y ↦ 7} ⊢ y ⇓ 7") "v = 10 + 7" --- "{x ↦ 10, y ↦ 7} ⊢ (+ x y) ⇓ 17") --- "{x ↦ 10} ⊢ ((λ (y) (+ x y)) 7) ⇓ 17") --- "{} ⊢ (let ([x 10]) ((λ (y) (+ x y)) 7)) ⇓ 17")'
  };

  // ── Load proof into editor ──────────────────────
  function loadProof(sexp) {
    if (emptyMsg) { emptyMsg.remove(); emptyMsg = null; }
    if (currentEditor) { currentEditor.destroy(); currentEditor = null; }
    canvas.innerHTML = '';

    try {
      currentEditor = ProofTree.createEditor(canvas, {
        tree: sexp,
        zoom: true,
        onChange: function(tree) {
          var s = ProofTree.toSexp(tree);
          sexpInput.value = s;
          history.replaceState(null, '', '#proof=' + encodeURIComponent(s));
        }
      });
      sexpInput.value = sexp;
      history.replaceState(null, '', '#proof=' + encodeURIComponent(sexp));
    } catch(e) {
      canvas.innerHTML = '<div style="color:#c00;padding:40px;text-align:center;">Error: ' + e.message + '</div>';
    }
  }

  function newProof() {
    if (emptyMsg) { emptyMsg.remove(); emptyMsg = null; }
    if (currentEditor) { currentEditor.destroy(); currentEditor = null; }
    canvas.innerHTML = '';
    currentEditor = ProofTree.createEditor(canvas, {
      zoom: true,
      onChange: function(tree) {
        var s = ProofTree.toSexp(tree);
        sexpInput.value = s;
        history.replaceState(null, '', '#proof=' + encodeURIComponent(s));
      }
    });
    sexpInput.value = '';
    history.replaceState(null, '', window.location.pathname);
  }

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
  document.getElementById('pg-examples').addEventListener('change', function() {
    var key = this.value;
    if (key && examples[key]) {
      loadProof(examples[key]);
      sexpPanel.classList.remove('open');
    }
    this.value = '';
  });

  // ── New button ──────────────────────────────────
  document.getElementById('pg-new-btn').addEventListener('click', newProof);

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
