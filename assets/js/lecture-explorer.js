(function () {
  "use strict";

  var split = document.getElementById("le-split");
  var notes = document.getElementById("le-notes");
  var divider = document.getElementById("le-divider");
  var explorer = document.getElementById("le-explorer");
  var iframe = document.getElementById("le-explorer-iframe");
  var toggleBtn = document.getElementById("le-toggle-explorer");

  var STORAGE_KEY = "le-split-ratio";
  var EXPLORER_KEY = "le-explorer-visible";
  var MIN_NOTES = 280;
  var MIN_EXPLORER = 300;
  var DEFAULT_RATIO = 0.4;

  // ── Explorer visibility ──────────────────────────
  var isMobile = window.innerWidth <= 700;

  function isExplorerVisible() {
    if (isMobile) {
      // Off by default on mobile; user can toggle on
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

    if (visible) {
      explorer.classList.remove("le-hidden");
      divider.classList.remove("le-hidden");
      notes.classList.remove("le-full");
      toggleBtn.classList.add("le-active");
      if (isMobile) {
        split.classList.add("le-mobile-split");
      } else {
        applyRatio(getSavedRatio());
      }
    } else {
      explorer.classList.add("le-hidden");
      divider.classList.add("le-hidden");
      notes.classList.add("le-full");
      toggleBtn.classList.remove("le-active");
      split.classList.remove("le-mobile-split");
      notes.style.width = "";
    }
  }

  // ── Ratio management ─────────────────────────────
  function getSavedRatio() {
    try {
      var saved = localStorage.getItem(STORAGE_KEY);
      if (saved !== null) {
        var ratio = parseFloat(saved);
        if (ratio > 0 && ratio < 1) return ratio;
      }
    } catch (e) { /* localStorage unavailable */ }
    return DEFAULT_RATIO;
  }

  function saveRatio(ratio) {
    try { localStorage.setItem(STORAGE_KEY, ratio.toFixed(4)); }
    catch (e) { /* ignore */ }
  }

  function applyRatio(ratio) {
    var totalWidth = split.clientWidth - divider.offsetWidth;
    var notesWidth = Math.max(MIN_NOTES, Math.round(totalWidth * ratio));
    if (totalWidth - notesWidth < MIN_EXPLORER) {
      notesWidth = totalWidth - MIN_EXPLORER;
    }
    if (notesWidth < MIN_NOTES) notesWidth = MIN_NOTES;
    notes.style.width = notesWidth + "px";
  }

  // Initialize
  setExplorerVisible(isExplorerVisible());

  // Toggle button
  toggleBtn.addEventListener("click", function () {
    setExplorerVisible(explorer.classList.contains("le-hidden"));
  });

  // ── Divider drag logic ───────────────────────────
  var isDragging = false;

  function onDragStart(e) {
    e.preventDefault();
    isDragging = true;
    split.classList.add("le-resizing");
    divider.classList.add("le-dragging");
    document.addEventListener("mousemove", onDragMove);
    document.addEventListener("mouseup", onDragEnd);
    document.addEventListener("touchmove", onDragMove, { passive: false });
    document.addEventListener("touchend", onDragEnd);
  }

  function onDragMove(e) {
    if (!isDragging) return;
    if (e.touches) e.preventDefault();
    var clientX = e.touches ? e.touches[0].clientX : e.clientX;
    var rect = split.getBoundingClientRect();
    var offset = clientX - rect.left;
    var totalWidth = split.clientWidth - divider.offsetWidth;
    var notesWidth = Math.max(MIN_NOTES, Math.min(totalWidth - MIN_EXPLORER, offset));
    notes.style.width = notesWidth + "px";
    saveRatio(notesWidth / totalWidth);
  }

  function onDragEnd() {
    isDragging = false;
    split.classList.remove("le-resizing");
    divider.classList.remove("le-dragging");
    document.removeEventListener("mousemove", onDragMove);
    document.removeEventListener("mouseup", onDragEnd);
    document.removeEventListener("touchmove", onDragMove);
    document.removeEventListener("touchend", onDragEnd);
  }

  divider.addEventListener("mousedown", onDragStart);
  divider.addEventListener("touchstart", onDragStart, { passive: false });

  divider.addEventListener("dblclick", function () {
    applyRatio(DEFAULT_RATIO);
    saveRatio(DEFAULT_RATIO);
  });

  // ── Resize handler ───────────────────────────────
  var resizeTimeout;
  window.addEventListener("resize", function () {
    clearTimeout(resizeTimeout);
    resizeTimeout = setTimeout(function () {
      isMobile = window.innerWidth <= 700;
      setExplorerVisible(isExplorerVisible());
    }, 100);
  });

  // ── Lambda term detection & click-to-load ────────

  var LAMBDA_RE = /^\s*\([\s\S]*λ[\s\S]*\)\s*$/;

  function isLambdaTerm(text) {
    var t = text.trim();
    if (!LAMBDA_RE.test(t)) return false;
    if (/[∈←↦⟹≡]|::=/.test(t)) return false;
    return true;
  }

  function normalizeTerm(text) {
    return text.trim().replace(/λ/g, "\\");
  }

  function sendTermToExplorer(term) {
    // If explorer is hidden, show it first
    if (explorer.classList.contains("le-hidden")) {
      setExplorerVisible(true);
    }
    if (iframe && iframe.contentWindow) {
      iframe.contentWindow.postMessage({ type: "loadTerm", term: term }, "*");
    }
  }

  function makeClickable(el) {
    el.classList.add("le-lambda-clickable");
    el.addEventListener("click", function (e) {
      e.preventDefault();
      e.stopPropagation();
      var term = normalizeTerm(el.textContent);
      sendTermToExplorer(term);
    });
  }

  var notesEl = document.getElementById("le-notes");
  var codeEls = notesEl.querySelectorAll("code");
  codeEls.forEach(function (code) {
    if (code.parentElement && code.parentElement.tagName === "PRE") return;
    if (isLambdaTerm(code.textContent)) {
      makeClickable(code);
    }
  });

  var preEls = notesEl.querySelectorAll("pre > code");
  preEls.forEach(function (code) {
    var lines = code.textContent.trim().split("\n");
    if (lines.length === 1 && isLambdaTerm(lines[0])) {
      makeClickable(code.parentElement);
    }
  });

})();
