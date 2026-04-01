(function () {
  "use strict";

  var split = document.getElementById("pe-split");
  var notes = document.getElementById("pe-notes");
  var divider = document.getElementById("pe-divider");
  var explorer = document.getElementById("pe-explorer");
  var toggleBtn = document.getElementById("pe-toggle-explorer");
  var editorWrap = document.getElementById("pe-editor-wrap");
  var checkBtn = document.getElementById("pe-check-btn");
  var clearBtn = document.getElementById("pe-clear-btn");
  var undoBtn = document.getElementById("pe-undo-btn");
  var rulesBtn = document.getElementById("pe-rules-btn");
  var rulesPanel = document.getElementById("pe-rules-panel");
  var rulesClose = document.getElementById("pe-rules-close");
  var rulesContent = document.getElementById("pe-rules-content");
  var statusBar = document.getElementById("pe-status");

  var STORAGE_KEY = "pe-split-ratio";
  var EXPLORER_KEY = "pe-explorer-visible";
  var MIN_NOTES = 280;
  var MIN_EXPLORER = 340;
  var DEFAULT_RATIO = 0.45;

  var editor = null;

  // ── Explorer visibility ──────────────────────────────
  var isMobile = window.innerWidth <= 700;

  function isExplorerVisible() {
    if (isMobile) {
      try { return localStorage.getItem(EXPLORER_KEY + "-mobile") === "true"; }
      catch (e) { return false; }
    }
    try {
      var v = localStorage.getItem(EXPLORER_KEY);
      return v === null ? true : v === "true";
    } catch (e) { return true; }
  }

  function setExplorerVisible(visible) {
    var key = isMobile ? EXPLORER_KEY + "-mobile" : EXPLORER_KEY;
    try { localStorage.setItem(key, visible ? "true" : "false"); }
    catch (e) { /* ignore */ }
  }

  function applyVisibility(visible) {
    if (visible) {
      explorer.classList.remove("pe-hidden");
      divider.classList.remove("pe-hidden");
      notes.classList.remove("pe-full");
      toggleBtn.classList.add("pe-active");
      if (isMobile) split.classList.add("pe-mobile-split");
      applySplitRatio(getSavedRatio());
    } else {
      explorer.classList.add("pe-hidden");
      divider.classList.add("pe-hidden");
      notes.classList.add("pe-full");
      toggleBtn.classList.remove("pe-active");
      if (isMobile) split.classList.remove("pe-mobile-split");
    }
  }

  applyVisibility(isExplorerVisible());

  toggleBtn.addEventListener("click", function () {
    var nowVisible = !isExplorerVisible();
    setExplorerVisible(nowVisible);
    applyVisibility(nowVisible);
  });

  // ── Split ratio persistence ──────────────────────────
  function getSavedRatio() {
    try {
      var r = parseFloat(localStorage.getItem(STORAGE_KEY));
      return isNaN(r) ? DEFAULT_RATIO : Math.max(0.2, Math.min(0.8, r));
    } catch (e) { return DEFAULT_RATIO; }
  }

  function saveRatio(r) {
    try { localStorage.setItem(STORAGE_KEY, r.toFixed(4)); }
    catch (e) { /* ignore */ }
  }

  function applySplitRatio(ratio) {
    if (isMobile) return;
    var totalW = split.clientWidth - divider.offsetWidth;
    var notesW = Math.max(MIN_NOTES, Math.round(totalW * (1 - ratio)));
    var explorerW = totalW - notesW;
    if (explorerW < MIN_EXPLORER) {
      explorerW = MIN_EXPLORER;
      notesW = totalW - explorerW;
    }
    notes.style.width = notesW + "px";
    explorer.style.flex = "0 0 " + explorerW + "px";
  }

  applySplitRatio(getSavedRatio());

  // ── Divider drag ─────────────────────────────────────
  var dragging = false;

  function onPointerDown(e) {
    e.preventDefault();
    dragging = true;
    divider.classList.add("pe-dragging");
    split.classList.add("pe-resizing");
    divider.setPointerCapture(e.pointerId);
  }

  function onPointerMove(e) {
    if (!dragging) return;
    var rect = split.getBoundingClientRect();
    var totalW = rect.width - divider.offsetWidth;
    var notesW = e.clientX - rect.left - divider.offsetWidth / 2;
    notesW = Math.max(MIN_NOTES, Math.min(totalW - MIN_EXPLORER, notesW));
    var ratio = 1 - notesW / totalW;
    notes.style.width = notesW + "px";
    explorer.style.flex = "0 0 " + (totalW - notesW) + "px";
    saveRatio(ratio);
  }

  function onPointerUp() {
    dragging = false;
    divider.classList.remove("pe-dragging");
    split.classList.remove("pe-resizing");
  }

  divider.addEventListener("pointerdown", onPointerDown);
  divider.addEventListener("pointermove", onPointerMove);
  divider.addEventListener("pointerup", onPointerUp);
  divider.addEventListener("pointercancel", onPointerUp);

  divider.addEventListener("dblclick", function () {
    applySplitRatio(DEFAULT_RATIO);
    saveRatio(DEFAULT_RATIO);
  });

  // ── Proof editor initialization ──────────────────────
  function setStatus(text, cls) {
    statusBar.textContent = text;
    statusBar.className = cls || "";
  }

  function initEditor() {
    if (typeof ProofChecker === "undefined" || typeof ProofTree === "undefined") {
      setStatus("Loading proof checker...");
      setTimeout(initEditor, 200);
      return;
    }

    ProofChecker.init().then(function () {
      var config = ProofChecker.theoryConfig("stlc");
      editor = ProofTree.createEditor(editorWrap, {
        tree: '(--- "")',
        zoom: true,
        showLigatureHints: true,
        theoryRules: config.theoryRules,
        onGeneratePremises: config.onGeneratePremises,
        onCheckApplicability: config.onCheckApplicability
      });
      setStatus("Ready. Click a judgement in the notes, or type one here.");
      buildRulesPanel();
    }).catch(function (err) {
      setStatus("Error loading proof checker: " + err, "pe-status-error");
    });
  }

  initEditor();

  // ── Check button ─────────────────────────────────────
  checkBtn.addEventListener("click", function () {
    if (!editor) return;
    ProofChecker.clearAnnotations(editorWrap);
    var result = ProofChecker.check(editor, "stlc");
    ProofChecker.annotate(editorWrap, result);
    if (result.valid) {
      setStatus("Valid! The typing derivation is correct.", "pe-status-valid");
      ProofChecker.flashSuccess(editorWrap);
    } else if (!result.complete) {
      var incomplete = result.diagnostics.filter(function (d) { return d.level === "incomplete"; }).length;
      setStatus(incomplete + " node(s) still need rules or premises.", "pe-status-error");
    } else {
      var errors = result.diagnostics.filter(function (d) { return d.level === "error"; });
      setStatus("Error: " + (errors[0] ? errors[0].message : "invalid derivation"), "pe-status-error");
      ProofChecker.flashError(editorWrap);
    }
  });

  // ── Clear button ─────────────────────────────────────
  clearBtn.addEventListener("click", function () {
    if (!editor) return;
    editor.setTree({ conclusion: "", premises: [], rule_name: null });
    ProofChecker.clearAnnotations(editorWrap);
    setStatus("Cleared. Click a judgement in the notes to start.");
  });

  // ── Undo button ──────────────────────────────────────
  undoBtn.addEventListener("click", function () {
    if (editor) editor.undo();
  });

  // ── Keyboard shortcut: Ctrl+Enter to check ───────────
  document.addEventListener("keydown", function (e) {
    if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
      e.preventDefault();
      checkBtn.click();
    }
  });

  // ── Rules panel toggle ───────────────────────────────
  var rulesVisible = false;

  function toggleRules() {
    rulesVisible = !rulesVisible;
    if (rulesVisible) {
      rulesPanel.classList.remove("pe-panel-hidden");
    } else {
      rulesPanel.classList.add("pe-panel-hidden");
    }
  }

  rulesBtn.addEventListener("click", toggleRules);
  rulesClose.addEventListener("click", toggleRules);

  // ── Build rules reference panel ──────────────────────
  var STLC_RULES = [
    { name: "T-Int", sexp: '(--- "\u0393 \u22A2 n : int")', condition: null },
    { name: "T-Bool", sexp: '(--- "\u0393 \u22A2 b : bool")', condition: null },
    { name: "T-Var", sexp: '(--- "\u0393 \u22A2 x : \u03C4")', condition: "(x : \u03C4) \u2208 \u0393" },
    { name: "T-Lam", sexp: '(("T-Lam" :right) (--- "\u0393, x:\u03C4\u2081 \u22A2 e : \u03C4\u2082") --- "\u0393 \u22A2 (\u03BB (x : \u03C4\u2081) e) : \u03C4\u2081 \u2192 \u03C4\u2082")', condition: null },
    { name: "T-App", sexp: '(("T-App" :right) (--- "\u0393 \u22A2 e\u2081 : \u03C4\u2081 \u2192 \u03C4\u2082") (--- "\u0393 \u22A2 e\u2082 : \u03C4\u2081") --- "\u0393 \u22A2 (e\u2081 e\u2082) : \u03C4\u2082")', condition: null },
    { name: "T-Add", sexp: '(("T-Add" :right) (--- "\u0393 \u22A2 e\u2081 : int") (--- "\u0393 \u22A2 e\u2082 : int") --- "\u0393 \u22A2 (+ e\u2081 e\u2082) : int")', condition: null },
    { name: "T-Neg", sexp: '(("T-Neg" :right) (--- "\u0393 \u22A2 e : int") --- "\u0393 \u22A2 (- e) : int")', condition: null },
    { name: "T-If", sexp: '(("T-If" :right) (--- "\u0393 \u22A2 e\u2081 : int") (--- "\u0393 \u22A2 e\u2082 : \u03C4") (--- "\u0393 \u22A2 e\u2083 : \u03C4") --- "\u0393 \u22A2 (if0 e\u2081 e\u2082 e\u2083) : \u03C4")', condition: null },
    { name: "T-Let", sexp: '(("T-Let" :right) (--- "\u0393 \u22A2 e\u2081 : \u03C4\u2081") (--- "\u0393, x:\u03C4\u2081 \u22A2 e\u2082 : \u03C4\u2082") --- "\u0393 \u22A2 (let ([x e\u2081]) e\u2082) : \u03C4\u2082")', condition: null }
  ];

  function buildRulesPanel() {
    rulesContent.innerHTML = "";
    STLC_RULES.forEach(function (rule) {
      var card = document.createElement("div");
      card.className = "pe-rule-card";

      var nameEl = document.createElement("div");
      nameEl.className = "pe-rule-name";
      nameEl.textContent = rule.name;
      card.appendChild(nameEl);

      var treeContainer = document.createElement("div");
      card.appendChild(treeContainer);
      ProofTree.renderReadonly(rule.sexp, treeContainer, { zoom: false });

      if (rule.condition) {
        var cond = document.createElement("div");
        cond.className = "pe-rule-condition";
        cond.textContent = "where " + rule.condition;
        card.appendChild(cond);
      }

      rulesContent.appendChild(card);
    });
  }

  // ── Clickable judgements in notes ─────────────────────
  function loadJudgement(text) {
    if (!editor) return;
    var judgement = text.trim();
    editor.setTree({ conclusion: judgement, premises: [], rule_name: null });
    ProofChecker.clearAnnotations(editorWrap);
    setStatus("Loaded: " + judgement);

    // Ensure explorer is visible
    if (!isExplorerVisible()) {
      setExplorerVisible(true);
      applyVisibility(true);
    }
  }

  var notesEl = document.getElementById("pe-notes");

  // Explicit .pe-judgement elements
  var judgements = notesEl.querySelectorAll(".pe-judgement");
  judgements.forEach(function (el) {
    el.classList.add("pe-clickable");
    el.addEventListener("click", function (e) {
      e.preventDefault();
      e.stopPropagation();
      loadJudgement(el.textContent);
    });
  });

  // "Try it" buttons with data-judgement attribute
  var tryBtns = notesEl.querySelectorAll("[data-judgement]");
  tryBtns.forEach(function (btn) {
    btn.addEventListener("click", function (e) {
      e.preventDefault();
      loadJudgement(btn.getAttribute("data-judgement"));
    });
  });

  // "Try it" buttons with data-proof attribute (load a partial/complete tree)
  var proofBtns = notesEl.querySelectorAll("[data-proof]");
  proofBtns.forEach(function (btn) {
    btn.addEventListener("click", function (e) {
      e.preventDefault();
      if (!editor) return;
      var sexp = btn.getAttribute("data-proof");
      editor.setTree(sexp);
      ProofChecker.clearAnnotations(editorWrap);
      setStatus("Loaded proof. Click Check to verify.");
      if (!isExplorerVisible()) {
        setExplorerVisible(true);
        applyVisibility(true);
      }
    });
  });
})();
