/**
 * proof-checker-glue.js — Thin wrapper that loads the WASM proof checker
 * and provides DOM integration for proof tree editors.
 *
 * Requires proof-tree.js to be loaded first.
 *
 * Usage:
 *   ProofChecker.init()                         — load WASM (called automatically)
 *   ProofChecker.check(editor, theory)           — check an editor's tree
 *   ProofChecker.annotate(container, result)     — apply CSS classes to nodes
 *   ProofChecker.createCheckButton(container, editor, theory) — full UI
 */
var ProofChecker = (function () {
  'use strict';

  var wasmModule = null;
  var initPromise = null;

  // ── WASM loading ───────────────────────────────────────────────

  // Capture script src at load time (document.currentScript is null later)
  var _scriptSrc = (typeof document !== 'undefined' && document.currentScript)
    ? document.currentScript.src
    : '';

  function init() {
    if (initPromise) return initPromise;
    initPromise = new Promise(function (resolve, reject) {
      // Resolve WASM JS URL relative to this script's location
      var base = _scriptSrc ? _scriptSrc.replace(/[^/]*$/, '') : '';
      var wasmJsUrl = base + 'wasm/proof_checker.js';

      import(wasmJsUrl)
        .then(function (mod) {
          // wasm-pack --target web produces an init() default export
          return mod.default().then(function () {
            wasmModule = mod;
            resolve(mod);
          });
        })
        .catch(function (err) {
          console.error('ProofChecker: failed to load WASM', err);
          reject(err);
        });
    });
    return initPromise;
  }

  // ── Generate premises ────────────────────────────────────────

  function generatePremises(conclusion, rule, theory) {
    if (!wasmModule) return null;
    try {
      var jsonStr = wasmModule.generate_premises(conclusion, rule, theory || '');
      return JSON.parse(jsonStr);
    } catch (e) {
      console.error('ProofChecker.generatePremises error:', e);
      return { ok: false, error: 'Internal error: ' + e.message };
    }
  }

  // ── Applicable rules ────────────────────────────────────────

  function applicableRules(conclusion, theory) {
    if (!wasmModule) return null;
    try {
      var jsonStr = wasmModule.applicable_rules(conclusion, theory || '');
      return JSON.parse(jsonStr);
    } catch (e) {
      console.error('ProofChecker.applicableRules error:', e);
      return {};
    }
  }

  // ── Check ─────────────────────────────────────────────────────

  function check(editor, theory) {
    if (!wasmModule) {
      return {
        valid: false,
        complete: false,
        diagnostics: [
          { level: 'error', path: [], message: 'Proof checker is still loading. Please try again in a moment.' },
        ],
      };
    }
    try {
      var sexp = editor.getSexp();
      var jsonStr = wasmModule.check_proof(sexp, theory || 'big-step');
      return JSON.parse(jsonStr);
    } catch (e) {
      console.error('ProofChecker.check error:', e);
      return {
        valid: false,
        complete: false,
        diagnostics: [
          { level: 'error', path: [], message: 'Failed to check proof: ' + e.message },
        ],
      };
    }
  }

  // ── DOM annotation ────────────────────────────────────────────

  function clearAnnotations(container) {
    var nodes = container.querySelectorAll(
      '.pt-check-valid, .pt-check-error, .pt-check-incomplete'
    );
    for (var i = 0; i < nodes.length; i++) {
      nodes[i].classList.remove(
        'pt-check-valid',
        'pt-check-error',
        'pt-check-incomplete'
      );
    }
    // Remove tooltips
    var tooltips = container.querySelectorAll('.pt-check-tooltip');
    for (var j = 0; j < tooltips.length; j++) {
      tooltips[j].parentNode.removeChild(tooltips[j]);
    }
    // Remove summary
    var summaries = container.querySelectorAll('.pt-check-summary');
    for (var k = 0; k < summaries.length; k++) {
      summaries[k].parentNode.removeChild(summaries[k]);
    }
  }

  // Escape HTML entities to prevent XSS in diagnostic messages
  function escapeHtml(str) {
    return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }

  function annotate(container, result) {
    clearAnnotations(container);

    // Build a map from path → diagnostics
    var pathMap = {};
    for (var i = 0; i < result.diagnostics.length; i++) {
      var d = result.diagnostics[i];
      var key = d.path.join(',');
      if (!pathMap[key]) pathMap[key] = [];
      pathMap[key].push(d);
    }

    // Walk .proof-node elements in DFS order
    var allNodes = [];
    function collectNodes(el, path) {
      allNodes.push({ el: el, path: path.slice() });
      // Find direct child .proof-premises, then its children
      var premisesEl = null;
      for (var c = 0; c < el.children.length; c++) {
        if (el.children[c].classList.contains('proof-premises')) {
          premisesEl = el.children[c];
          break;
        }
      }
      if (premisesEl) {
        var childIdx = 0;
        for (var c2 = 0; c2 < premisesEl.children.length; c2++) {
          var child = premisesEl.children[c2];
          if (child.classList.contains('proof-node')) {
            collectNodes(child, path.concat([childIdx]));
            childIdx++;
          }
        }
      }
    }

    // Find the root .proof-node
    var canvas =
      container.querySelector('.proof-tree-canvas') || container;
    var rootNode = canvas.querySelector('.proof-node');
    if (rootNode) {
      collectNodes(rootNode, []);
    }

    // Apply classes and create inline error tooltips
    for (var n = 0; n < allNodes.length; n++) {
      var info = allNodes[n];
      var key = info.path.join(',');
      var diags = pathMap[key];
      if (!diags) continue;

      // Determine worst level and collect messages
      var hasError = false;
      var hasIncomplete = false;
      var hasValid = false;
      var messages = [];
      for (var dd = 0; dd < diags.length; dd++) {
        if (diags[dd].level === 'error') hasError = true;
        else if (diags[dd].level === 'incomplete') hasIncomplete = true;
        else if (diags[dd].level === 'valid') hasValid = true;
        if (diags[dd].level !== 'valid') {
          messages.push(diags[dd].message);
        }
      }

      if (hasError) {
        info.el.classList.add('pt-check-error');
      } else if (hasIncomplete) {
        info.el.classList.add('pt-check-incomplete');
      } else if (hasValid) {
        info.el.classList.add('pt-check-valid');
      }

      // Show inline tooltip for error/incomplete nodes
      if (messages.length > 0) {
        var tooltip = document.createElement('div');
        tooltip.className = 'pt-check-tooltip';
        if (hasError) {
          tooltip.classList.add('pt-check-tooltip-error');
        } else {
          tooltip.classList.add('pt-check-tooltip-incomplete');
        }
        var html = '';
        for (var m = 0; m < messages.length; m++) {
          if (m > 0) html += '<br>';
          html += escapeHtml(messages[m]);
        }
        tooltip.innerHTML = html;
        // Insert after the conclusion element
        var conclusionEl = info.el.querySelector(':scope > .proof-conclusion');
        if (conclusionEl) {
          conclusionEl.parentNode.insertBefore(tooltip, conclusionEl.nextSibling);
        } else {
          info.el.appendChild(tooltip);
        }
      }
    }

    return result;
  }

  // ── Summary banner ────────────────────────────────────────────

  function createSummary(container, result) {
    // Remove old summary
    var old = container.parentNode.querySelector('.pt-check-summary');
    if (old) old.parentNode.removeChild(old);

    var summary = document.createElement('div');
    summary.className = 'pt-check-summary';
    summary.setAttribute('role', 'status');
    summary.setAttribute('aria-live', 'polite');

    var errorCount = 0;
    var incompleteCount = 0;
    for (var i = 0; i < result.diagnostics.length; i++) {
      if (result.diagnostics[i].level === 'error') errorCount++;
      else if (result.diagnostics[i].level === 'incomplete') incompleteCount++;
    }

    if (result.valid) {
      summary.classList.add('pt-check-summary-valid');
      summary.innerHTML =
        '<span class="pt-check-icon">&#10003;</span> Correct!';
    } else if (errorCount > 0) {
      summary.classList.add('pt-check-summary-error');
      var errorMessages = result.diagnostics
        .filter(function (d) {
          return d.level === 'error';
        })
        .map(function (d) {
          return d.message;
        });
      // Show all errors as a list
      var html =
        '<span class="pt-check-icon">&#10007;</span> ' +
        errorCount +
        ' error' +
        (errorCount > 1 ? 's' : '');
      if (errorMessages.length <= 3) {
        html += ': ';
        for (var e = 0; e < errorMessages.length; e++) {
          if (e > 0) html += ' · ';
          html += escapeHtml(errorMessages[e]);
        }
      } else {
        html += ':<ul class="pt-check-error-list">';
        for (var e2 = 0; e2 < errorMessages.length; e2++) {
          html += '<li>' + escapeHtml(errorMessages[e2]) + '</li>';
        }
        html += '</ul>';
      }
      summary.innerHTML = html;
    } else if (incompleteCount > 0) {
      summary.classList.add('pt-check-summary-incomplete');
      summary.innerHTML =
        '<span class="pt-check-icon">&#9679;</span> ' +
        incompleteCount +
        ' node' +
        (incompleteCount > 1 ? 's' : '') +
        ' not yet filled in';
    }

    // Insert after the container
    container.parentNode.insertBefore(summary, container.nextSibling);
  }

  // ── Check button factory ──────────────────────────────────────

  function createCheckButton(container, editor, theory) {
    var btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'pt-check-btn';
    btn.textContent = 'Check My Proof';

    var checking = false;

    btn.addEventListener('click', function () {
      if (checking) return;
      checking = true;
      btn.textContent = 'Checking\u2026';
      btn.classList.add('pt-check-btn-loading');

      init()
        .then(function () {
          var result = check(editor, theory);
          annotate(container, result);
          createSummary(container, result);

          // Flash green on valid proof
          if (result.valid) {
            flashSuccess(container);
          }
        })
        .catch(function (err) {
          // Show WASM load failure
          var errResult = {
            valid: false,
            complete: false,
            diagnostics: [
              { level: 'error', path: [], message: 'Failed to load proof checker. Please refresh the page.' },
            ],
          };
          annotate(container, errResult);
          createSummary(container, errResult);
        })
        .finally(function () {
          checking = false;
          btn.textContent = 'Check My Proof';
          btn.classList.remove('pt-check-btn-loading');
        });
    });

    // Insert button after the editor container
    container.parentNode.insertBefore(btn, container.nextSibling);

    // Clear annotations when the editor re-renders
    if (typeof MutationObserver !== 'undefined') {
      var observer = new MutationObserver(function () {
        clearAnnotations(container);
        var oldSummary =
          container.parentNode.querySelector('.pt-check-summary');
        if (oldSummary) oldSummary.parentNode.removeChild(oldSummary);
      });
      var canvas =
        container.querySelector('.proof-tree-canvas') || container;
      observer.observe(canvas, { childList: true, subtree: true });
    }

    return btn;
  }

  // ── Green flash on successful proof ────────────────────────────

  function flashSuccess(container) {
    var overlay = document.createElement('div');
    overlay.className = 'pt-success-flash';
    // Insert at document body level for full-viewport effect
    document.body.appendChild(overlay);
    // Force reflow then trigger animation
    overlay.offsetHeight;
    overlay.classList.add('pt-success-flash-active');
    setTimeout(function () {
      overlay.classList.remove('pt-success-flash-active');
      overlay.classList.add('pt-success-flash-out');
      setTimeout(function () {
        if (overlay.parentNode) overlay.parentNode.removeChild(overlay);
      }, 600);
    }, 500);
  }

  // ── Theory config (shared rule lists + helper factories) ─────

  var _theoryRuleNames = {
    'big-step':   ['Int', 'Var', 'Lam', 'Add', 'Neg', 'App', 'If0-True', 'If0-False', 'Let'],
    'small-step': ['Add', 'Neg', 'Beta', 'Add-L', 'Add-R', 'Neg-Step', 'App-L', 'App-R', 'If0-True', 'If0-False', 'If0-Step', 'Let-Step', 'Let'],
    'g3ip':       ['Ax', '\u22A5L', '\u22A4R', '\u2227R', '\u2227L', '\u2228R\u2081', '\u2228R\u2082', '\u2228L', '\u2192R', '\u2192L'],
    'propnd':     ['Ax', '\u2192I', '\u2192E', '\u2227I', '\u2227E\u2081', '\u2227E\u2082', '\u2228I\u2081', '\u2228I\u2082', '\u2228E', '\u22A5E', '\u00ACI', '\u00ACE'],
    'fond':       ['Ax', '\u2192I', '\u2192E', '\u2227I', '\u2227E\u2081', '\u2227E\u2082', '\u2228I\u2081', '\u2228I\u2082', '\u2228E', '\u22A5E', '\u00ACI', '\u00ACE', '\u2200I', '\u2200E', '\u2203I', '\u2203E'],
    'stlc':       ['T-Var', 'T-Int', 'T-Bool', 'T-Lam', 'T-App', 'T-Add', 'T-Neg', 'T-If', 'T-Let'],
    'systemf':    ['T-Var', 'T-Int', 'T-Bool', 'T-Lam', 'T-App', 'T-Add', 'T-Neg', 'T-If', 'T-Let', 'T-TyLam', 'T-TyApp'],
  };

  /**
   * Return editor options for a given theory id.
   * { theoryRules, onGeneratePremises, onCheckApplicability }
   * Usage:  ProofTree.createEditor(el, Object.assign({}, ProofChecker.theoryConfig('big-step'), { ... }))
   */
  function theoryConfig(theoryId) {
    if (!theoryId) return { theoryRules: [], onGeneratePremises: null, onCheckApplicability: null };
    return {
      theoryRules: _theoryRuleNames[theoryId] || [],
      onGeneratePremises: function (conclusion, ruleName) {
        return generatePremises(conclusion, ruleName, theoryId);
      },
      onCheckApplicability: function (conclusion) {
        return applicableRules(conclusion, theoryId);
      },
    };
  }

  // ── Auto-init ─────────────────────────────────────────────────

  // Start loading WASM as soon as this script runs
  if (typeof document !== 'undefined') {
    init().catch(function () {
      // Silently fail — will show error when user clicks check
    });
  }

  return {
    init: init,
    check: check,
    generatePremises: generatePremises,
    applicableRules: applicableRules,
    theoryConfig: theoryConfig,
    theoryRuleNames: _theoryRuleNames,
    annotate: annotate,
    clearAnnotations: clearAnnotations,
    createCheckButton: createCheckButton,
    flashSuccess: flashSuccess,
  };
})();
