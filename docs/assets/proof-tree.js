/**
 * proof-tree.js — Standalone proof tree component
 *
 * Usage:
 *   ProofTree.renderReadonly(node, container, options)
 *   ProofTree.createEditor(container, options)
 *   ProofTree.prettyFormula(raw)
 *
 * No external dependencies. Pair with proof-tree.css.
 */
var ProofTree = (function() {
    'use strict';

    // ── Formula pretty-printing ──────────────────────────

    function prettyFormula(raw) {
        if (!raw) return '';
        var s = raw;
        s = s.replace(/\\forall/g, '\u2200');
        s = s.replace(/\\exists/g, '\u2203');
        s = s.replace(/\\Gamma/g, '\u0393');
        s = s.replace(/\\Delta/g, '\u0394');
        s = s.replace(/\\Lambda/g, '\u039B');
        s = s.replace(/\\Sigma/g, '\u03A3');
        s = s.replace(/\\lambda/g, '\u03BB');
        s = s.replace(/\\alpha/g, '\u03B1');
        s = s.replace(/\\beta/g, '\u03B2');
        s = s.replace(/\\gamma/g, '\u03B3');
        s = s.replace(/\\delta/g, '\u03B4');
        s = s.replace(/\\epsilon/g, '\u03B5');
        s = s.replace(/\\sigma/g, '\u03C3');
        s = s.replace(/\\tau/g, '\u03C4');
        s = s.replace(/\\phi/g, '\u03C6');
        s = s.replace(/\\psi/g, '\u03C8');
        s = s.replace(/\\omega/g, '\u03C9');
        s = s.replace(/\\bot/g, '\u22A5');
        s = s.replace(/\\top/g, '\u22A4');
        s = s.replace(/\\neg/g, '\u00AC');
        s = s.replace(/\\in/g, '\u2208');
        s = s.replace(/\\notin/g, '\u2209');
        s = s.replace(/\\subset/g, '\u2282');
        s = s.replace(/\\subseteq/g, '\u2286');
        s = s.replace(/\\cup/g, '\u222A');
        s = s.replace(/\\cap/g, '\u2229');
        s = s.replace(/\\vdash/g, '\u22A2');
        s = s.replace(/\\models/g, '\u22A8');
        s = s.replace(/\\implies/g, '\u2192');
        s = s.replace(/\\iff/g, '\u2194');
        s = s.replace(/\\land/g, '\u2227');
        s = s.replace(/\\lor/g, '\u2228');
        s = s.replace(/\\lnot/g, '\u00AC');
        s = s.replace(/\|-/g, '\u22A2');
        s = s.replace(/<->/g, '\u2194');
        s = s.replace(/->/g, '\u2192');
        s = s.replace(/\/\\/g, '\u2227');
        s = s.replace(/\\\//g, '\u2228');
        s = s.replace(/~~/g, '\u2194');
        s = s.replace(/~/g, '\u00AC');
        return s;
    }

    // ── S-expression parser / serializer ────────────────

    function tokenizeSexp(str) {
        var tokens = [];
        var i = 0;
        while (i < str.length) {
            // Skip whitespace
            if (/\s/.test(str[i])) { i++; continue; }

            // Parens
            if (str[i] === '(') { tokens.push({ type: 'LPAREN' }); i++; continue; }
            if (str[i] === ')') { tokens.push({ type: 'RPAREN' }); i++; continue; }

            // Quoted string (double or single quotes)
            if (str[i] === '"' || str[i] === "'") {
                var quote = str[i];
                i++;
                var s = '';
                while (i < str.length && str[i] !== quote) {
                    if (str[i] === '\\' && i + 1 < str.length) {
                        s += str[i + 1]; i += 2;
                    } else {
                        s += str[i]; i++;
                    }
                }
                if (i < str.length) i++; // skip closing quote
                tokens.push({ type: 'STRING', value: s });
                continue;
            }

            // --- separator (must be a standalone token)
            if (str.substr(i, 3) === '---' && (i + 3 >= str.length || /[\s()]/.test(str[i + 3]))) {
                tokens.push({ type: 'SEP' });
                i += 3;
                continue;
            }

            // :keyword
            if (str[i] === ':') {
                i++;
                var kw = '';
                while (i < str.length && !/[\s()]/.test(str[i])) {
                    kw += str[i]; i++;
                }
                tokens.push({ type: 'KEYWORD', value: kw });
                continue;
            }

            // Bare word/symbol
            var word = '';
            while (i < str.length && !/[\s()"']/.test(str[i])) {
                word += str[i]; i++;
            }
            if (word) tokens.push({ type: 'SYMBOL', value: word });
        }
        return tokens;
    }

    function parseSexp(str) {
        var tokens = tokenizeSexp(str.trim());
        var pos = 0;

        function peek() { return pos < tokens.length ? tokens[pos] : null; }
        function peekAt(n) { return (pos + n) < tokens.length ? tokens[pos + n] : null; }
        function next() { return tokens[pos++]; }

        // Detect (name :right) or (name :left) label spec via lookahead
        function isLabelSpec() {
            var t0 = peekAt(0), t1 = peekAt(1), t2 = peekAt(2), t3 = peekAt(3);
            return t0 && t0.type === 'LPAREN' &&
                   t1 && (t1.type === 'SYMBOL' || t1.type === 'STRING') &&
                   t2 && t2.type === 'KEYWORD' && (t2.value === 'right' || t2.value === 'left') &&
                   t3 && t3.type === 'RPAREN';
        }

        function parseNode() {
            var t = peek();
            if (!t) throw new Error('ProofTree.parseSexp: unexpected end of input');

            if (t.type === 'LPAREN') {
                // Could be a label spec — check before consuming
                if (isLabelSpec()) {
                    // This is a label spec at top level — not valid as a standalone node
                    throw new Error('ProofTree.parseSexp: unexpected label spec outside rule');
                }

                next(); // consume (

                // Parse leading label specs: (name :right) or (name :left)
                var ruleName = null;
                var ruleLabelLeft = null;
                while (isLabelSpec()) {
                    next(); // consume inner (
                    var labelValue = next().value;
                    var side = next().value;
                    next(); // consume inner )
                    if (side === 'right') {
                        ruleName = labelValue;
                    } else {
                        ruleLabelLeft = labelValue;
                    }
                }

                // Premises (everything before ---)
                var premises = [];
                while (peek() && peek().type !== 'SEP' && peek().type !== 'RPAREN') {
                    premises.push(parseNode());
                }

                // Expect ---
                if (!peek() || peek().type !== 'SEP') {
                    throw new Error('ProofTree.parseSexp: expected --- separator');
                }
                next(); // consume ---

                // Conclusion
                var conclusionParts = [];
                while (peek() && peek().type !== 'RPAREN' && peek().type !== 'KEYWORD') {
                    var ct = next();
                    conclusionParts.push(ct.value);
                }
                if (conclusionParts.length === 0) {
                    throw new Error('ProofTree.parseSexp: expected conclusion after ---');
                }
                var conclusion = conclusionParts.join(' ');

                // Expect )
                if (!peek() || peek().type !== 'RPAREN') {
                    throw new Error('ProofTree.parseSexp: expected )');
                }
                next(); // consume )

                return {
                    conclusion: conclusion,
                    rule_name: ruleName,
                    rule_label_left: ruleLabelLeft,
                    premises: premises
                };
            } else if (t.type === 'STRING' || t.type === 'SYMBOL') {
                next();
                return {
                    conclusion: t.value,
                    rule_name: null,
                    rule_label_left: null,
                    premises: []
                };
            } else {
                throw new Error('ProofTree.parseSexp: unexpected token ' + t.type);
            }
        }

        var result = parseNode();
        if (pos < tokens.length) {
            throw new Error('ProofTree.parseSexp: unexpected tokens after expression');
        }
        return result;
    }

    function sexpQuote(s) {
        if (!s) return '""';
        if (/[\s()"':;]/.test(s) || s === '---') {
            return '"' + s.replace(/\\/g, '\\\\').replace(/"/g, '\\"') + '"';
        }
        return s;
    }

    function toSexp(node) {
        // Leaf node — just a formula
        if (!node.rule_name && !node.rule_label_left &&
            (!node.premises || node.premises.length === 0)) {
            return sexpQuote(node.conclusion || '');
        }

        var inner = [];

        // Label specs come first
        if (node.rule_name) {
            inner.push('(' + sexpQuote(node.rule_name) + ' :right)');
        }
        if (node.rule_label_left) {
            inner.push('(' + sexpQuote(node.rule_label_left) + ' :left)');
        }

        // Premises
        if (node.premises && node.premises.length > 0) {
            for (var i = 0; i < node.premises.length; i++) {
                inner.push(toSexp(node.premises[i]));
            }
        }

        inner.push('---');
        inner.push(sexpQuote(node.conclusion || ''));

        return '(' + inner.join(' ') + ')';
    }

    // ── Viewport / zoom ──────────────────────────────────

    function fallbackCopy(text, btn) {
        var ta = document.createElement('textarea');
        ta.value = text;
        ta.style.position = 'fixed';
        ta.style.opacity = '0';
        document.body.appendChild(ta);
        ta.select();
        try {
            document.execCommand('copy');
            btn.textContent = 'Copied!';
            setTimeout(function() { btn.textContent = 'Copy'; }, 1500);
        } catch (e) {
            btn.textContent = 'Failed';
            setTimeout(function() { btn.textContent = 'Copy'; }, 1500);
        }
        document.body.removeChild(ta);
    }

    function createViewport(container, vpOptions) {
        vpOptions = vpOptions || {};

        // Orientation hint — visible only on narrow portrait screens via CSS
        var hint = document.createElement('div');
        hint.className = 'proof-tree-orientation-hint';
        hint.textContent = '\u21BB Rotate to landscape for the best experience';
        container.appendChild(hint);

        var viewport = document.createElement('div');
        viewport.className = 'proof-tree-viewport';

        var canvas = document.createElement('div');
        canvas.className = 'proof-tree-canvas';

        var controls = document.createElement('div');
        controls.className = 'proof-tree-zoom-controls';

        var zoomOutBtn = document.createElement('button');
        zoomOutBtn.type = 'button';
        zoomOutBtn.textContent = '\u2212';
        zoomOutBtn.title = 'Zoom out';

        var zoomLabel = document.createElement('span');
        zoomLabel.className = 'proof-tree-zoom-label';
        zoomLabel.textContent = '100%';

        var zoomInBtn = document.createElement('button');
        zoomInBtn.type = 'button';
        zoomInBtn.textContent = '+';
        zoomInBtn.title = 'Zoom in';

        var fitBtn = document.createElement('button');
        fitBtn.type = 'button';
        fitBtn.textContent = 'Fit';
        fitBtn.title = 'Auto-fit to viewport';

        controls.appendChild(zoomOutBtn);
        controls.appendChild(zoomLabel);
        controls.appendChild(zoomInBtn);
        controls.appendChild(fitBtn);

        if (vpOptions.getTree) {
            var copyBtn = document.createElement('button');
            copyBtn.type = 'button';
            copyBtn.textContent = 'Copy';
            copyBtn.title = 'Copy tree as S-expression';
            copyBtn.addEventListener('click', function() {
                var sexp = toSexp(vpOptions.getTree());
                if (navigator.clipboard && navigator.clipboard.writeText) {
                    navigator.clipboard.writeText(sexp).then(function() {
                        copyBtn.textContent = 'Copied!';
                        setTimeout(function() { copyBtn.textContent = 'Copy'; }, 1500);
                    }, function() {
                        fallbackCopy(sexp, copyBtn);
                    });
                } else {
                    fallbackCopy(sexp, copyBtn);
                }
            });
            controls.appendChild(copyBtn);
        }

        viewport.appendChild(canvas);
        viewport.appendChild(controls);
        container.appendChild(viewport);

        var currentZoom = 1;
        var panX = 0, panY = 0;
        var MIN_ZOOM = 0.1;
        var MAX_ZOOM = 4;

        canvas.style.transformOrigin = '0 0';

        function applyTransform() {
            canvas.style.transform = 'translate(' + panX + 'px, ' + panY + 'px) scale(' + currentZoom + ')';
            var natural = canvas.scrollHeight;
            var h = Math.ceil(natural * currentZoom + panY);
            viewport.style.height = Math.max(h, 60) + 'px';
            zoomLabel.textContent = Math.round(currentZoom * 100) + '%';
        }

        function setZoom(z, cx, cy) {
            z = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, z));
            if (cx !== undefined && cy !== undefined) {
                // Zoom toward point (cx, cy) in viewport coords
                var rect = viewport.getBoundingClientRect();
                var vx = cx - rect.left;
                var vy = cy - rect.top;
                // Point in canvas space before zoom
                var canvasX = (vx - panX) / currentZoom;
                var canvasY = (vy - panY) / currentZoom;
                panX = vx - canvasX * z;
                panY = vy - canvasY * z;
            }
            currentZoom = z;
            applyTransform();
        }

        function autoFit() {
            // Temporarily reset to measure natural size
            canvas.style.transform = 'scale(1)';
            canvas.style.transformOrigin = '0 0';
            viewport.style.height = '';
            var cw = canvas.scrollWidth;
            var ch = canvas.scrollHeight;
            var vw = viewport.clientWidth;
            var vh = viewport.clientHeight || window.innerHeight;
            if (cw > 0 && ch > 0) {
                var zx = vw / cw;
                var zy = vh / ch;
                var z = Math.min(zx, zy, 1);
                currentZoom = Math.max(MIN_ZOOM, z);
                // Center the content
                panX = (vw - cw * currentZoom) / 2;
                panY = (vh - ch * currentZoom) / 2;
                if (panY < 0) panY = 10;
            } else {
                currentZoom = 1;
                panX = 0;
                panY = 0;
            }
            applyTransform();
        }

        zoomInBtn.addEventListener('click', function() {
            var rect = viewport.getBoundingClientRect();
            setZoom(currentZoom * 1.2, rect.left + rect.width/2, rect.top + rect.height/2);
        });
        zoomOutBtn.addEventListener('click', function() {
            var rect = viewport.getBoundingClientRect();
            setZoom(currentZoom / 1.2, rect.left + rect.width/2, rect.top + rect.height/2);
        });
        fitBtn.addEventListener('click', autoFit);

        // ── Mouse wheel zoom ──────────────────────────
        viewport.addEventListener('wheel', function(e) {
            if (e.ctrlKey || e.metaKey) {
                e.preventDefault();
                var delta = -e.deltaY * 0.01;
                setZoom(currentZoom * (1 + delta), e.clientX, e.clientY);
            } else {
                // Scroll to pan
                e.preventDefault();
                panX -= e.deltaX;
                panY -= e.deltaY;
                applyTransform();
            }
        }, { passive: false });

        // ── Mouse drag to pan ─────────────────────────
        var isDragging = false;
        var dragStartX = 0, dragStartY = 0;
        var dragStartPanX = 0, dragStartPanY = 0;

        viewport.addEventListener('mousedown', function(e) {
            // Only start drag on viewport/canvas background, not on interactive elements
            if (e.target === viewport || e.target === canvas || e.target.classList.contains('proof-tree-canvas')) {
                isDragging = true;
                dragStartX = e.clientX;
                dragStartY = e.clientY;
                dragStartPanX = panX;
                dragStartPanY = panY;
                viewport.style.cursor = 'grabbing';
                e.preventDefault();
            }
        });
        document.addEventListener('mousemove', function(e) {
            if (!isDragging) return;
            panX = dragStartPanX + (e.clientX - dragStartX);
            panY = dragStartPanY + (e.clientY - dragStartY);
            applyTransform();
        });
        document.addEventListener('mouseup', function() {
            if (isDragging) {
                isDragging = false;
                viewport.style.cursor = '';
            }
        });

        // ── Touch: pinch-to-zoom and drag ─────────────
        var lastTouchDist = 0;
        var lastTouchMidX = 0, lastTouchMidY = 0;
        var touchStartPanX = 0, touchStartPanY = 0;

        viewport.addEventListener('touchstart', function(e) {
            if (e.touches.length === 1) {
                dragStartX = e.touches[0].clientX;
                dragStartY = e.touches[0].clientY;
                touchStartPanX = panX;
                touchStartPanY = panY;
            } else if (e.touches.length === 2) {
                var dx = e.touches[1].clientX - e.touches[0].clientX;
                var dy = e.touches[1].clientY - e.touches[0].clientY;
                lastTouchDist = Math.sqrt(dx*dx + dy*dy);
                lastTouchMidX = (e.touches[0].clientX + e.touches[1].clientX) / 2;
                lastTouchMidY = (e.touches[0].clientY + e.touches[1].clientY) / 2;
                touchStartPanX = panX;
                touchStartPanY = panY;
            }
        }, { passive: true });

        viewport.addEventListener('touchmove', function(e) {
            if (e.touches.length === 1) {
                e.preventDefault();
                panX = touchStartPanX + (e.touches[0].clientX - dragStartX);
                panY = touchStartPanY + (e.touches[0].clientY - dragStartY);
                applyTransform();
            } else if (e.touches.length === 2) {
                e.preventDefault();
                var dx = e.touches[1].clientX - e.touches[0].clientX;
                var dy = e.touches[1].clientY - e.touches[0].clientY;
                var dist = Math.sqrt(dx*dx + dy*dy);
                var midX = (e.touches[0].clientX + e.touches[1].clientX) / 2;
                var midY = (e.touches[0].clientY + e.touches[1].clientY) / 2;
                if (lastTouchDist > 0) {
                    var scale = dist / lastTouchDist;
                    setZoom(currentZoom * scale, midX, midY);
                }
                // Also pan with two-finger drag
                panX += midX - lastTouchMidX;
                panY += midY - lastTouchMidY;
                lastTouchDist = dist;
                lastTouchMidX = midX;
                lastTouchMidY = midY;
                applyTransform();
            }
        }, { passive: false });

        // ── Trackpad pinch (gesturechange for Safari) ──
        viewport.addEventListener('gesturestart', function(e) { e.preventDefault(); }, { passive: false });
        viewport.addEventListener('gesturechange', function(e) {
            e.preventDefault();
            setZoom(currentZoom * e.scale, e.clientX, e.clientY);
        }, { passive: false });

        // Debounced resize listener
        var resizeTimer = null;
        function onResize() {
            clearTimeout(resizeTimer);
            resizeTimer = setTimeout(autoFit, 150);
        }
        window.addEventListener('resize', onResize);

        return {
            canvas: canvas,
            setZoom: setZoom,
            autoFit: autoFit,
            destroy: function() {
                window.removeEventListener('resize', onResize);
                clearTimeout(resizeTimer);
                if (hint.parentNode) hint.parentNode.removeChild(hint);
                if (viewport.parentNode) viewport.parentNode.removeChild(viewport);
            }
        };
    }

    // ── Read-only rendering ──────────────────────────────

    function renderReadonlyNode(node, parentEl) {
        var el = document.createElement('div');
        el.className = 'proof-node proof-readonly';

        var hasPremises = node.premises && node.premises.length > 0;
        var hasRule = node.rule_name || node.rule_label_left;

        if (hasPremises) {
            var premisesEl = document.createElement('div');
            premisesEl.className = 'proof-premises';
            for (var i = 0; i < node.premises.length; i++) {
                renderReadonlyNode(node.premises[i], premisesEl);
            }
            el.appendChild(premisesEl);
        }

        if (hasPremises || hasRule) {
            var inferenceEl = document.createElement('div');
            inferenceEl.className = 'proof-inference';

            if (node.rule_label_left) {
                var leftEl = document.createElement('span');
                leftEl.className = 'proof-rule-label proof-rule-left';
                leftEl.textContent = node.rule_label_left;
                inferenceEl.appendChild(leftEl);
            }

            var lineEl = document.createElement('span');
            lineEl.className = 'proof-line';
            inferenceEl.appendChild(lineEl);

            if (node.rule_name) {
                var rightEl = document.createElement('span');
                rightEl.className = 'proof-rule-label proof-rule-right';
                rightEl.textContent = node.rule_name;
                inferenceEl.appendChild(rightEl);
            }

            el.appendChild(inferenceEl);
        }

        var conclusionEl = document.createElement('div');
        conclusionEl.className = 'proof-conclusion';
        conclusionEl.textContent = prettyFormula(node.conclusion);
        el.appendChild(conclusionEl);

        parentEl.appendChild(el);
    }

    function renderReadonly(node, container, options) {
        if (typeof node === 'string') node = parseSexp(node);
        options = options || {};
        var useZoom = options.zoom !== false;

        if (useZoom) {
            var readonlyNode = node;
            var vp = createViewport(container, { getTree: function() { return readonlyNode; } });
            renderReadonlyNode(node, vp.canvas);
            // Schedule autoFit after the DOM has had a chance to lay out
            setTimeout(function() { vp.autoFit(); }, 0);
            return { destroy: vp.destroy };
        } else {
            var wrapper = document.createElement('div');
            wrapper.className = 'proof-tree-canvas';
            container.appendChild(wrapper);
            renderReadonlyNode(node, wrapper);
            return { destroy: function() { if (wrapper.parentNode) wrapper.parentNode.removeChild(wrapper); } };
        }
    }

    // ── Interactive editor ────────────────────────────────

    function createEditor(container, options) {
        options = options || {};
        var useZoom = options.zoom !== false;
        var onChange = options.onChange || function() {};

        // Closure-scoped state — no globals
        var rawTree = options.tree || null;
        var proofTree = rawTree
            ? (typeof rawTree === 'string' ? parseSexp(rawTree) : rawTree)
            : { conclusion: '', rule_name: null, rule_label_left: null, premises: [] };

        var vp = null;
        var canvasEl;

        if (useZoom) {
            vp = createViewport(container, { getTree: function() { return proofTree; } });
            canvasEl = vp.canvas;
        } else {
            canvasEl = document.createElement('div');
            canvasEl.className = 'proof-tree-canvas';
            container.appendChild(canvasEl);
        }

        function getNodeAtPath(path) {
            var node = proofTree;
            for (var i = 0; i < path.length; i++) {
                node = node.premises[path[i]];
            }
            return node;
        }

        function applyRule(path) {
            var node = getNodeAtPath(path);
            node.rule_name = '';
            node.rule_label_left = null;
            node.premises = [{ conclusion: '', rule_name: null, rule_label_left: null, premises: [] }];
            rerender();
        }

        function addPremise(path) {
            var node = getNodeAtPath(path);
            node.premises.push({ conclusion: '', rule_name: null, rule_label_left: null, premises: [] });
            rerender();
        }

        function removePremise(path) {
            var parentPath = path.slice(0, -1);
            var idx = path[path.length - 1];
            var parent = getNodeAtPath(parentPath);
            parent.premises.splice(idx, 1);
            rerender();
        }

        function clearNode(path) {
            var node = getNodeAtPath(path);
            node.rule_name = null;
            node.rule_label_left = null;
            node.premises = [];
            rerender();
        }

        function canRemove(path) {
            if (path.length === 0) return false;
            var parentNode = getNodeAtPath(path.slice(0, -1));
            return parentNode.premises.length > 1;
        }

        function editConclusion(path, spanEl) {
            var node = getNodeAtPath(path);
            var input = document.createElement('input');
            input.type = 'text';
            input.className = 'proof-formula-edit';
            input.value = node.conclusion;
            input.addEventListener('keydown', function(e) {
                if (e.key === 'Enter') {
                    node.conclusion = input.value;
                    rerender();
                } else if (e.key === 'Escape') {
                    rerender();
                }
            });
            input.addEventListener('blur', function() {
                node.conclusion = input.value;
                rerender();
            });
            spanEl.replaceWith(input);
            input.focus();
            input.select();
        }

        function editRuleLabel(path, side, spanEl) {
            var node = getNodeAtPath(path);
            var current = (side === 'left') ? (node.rule_label_left || '') : (node.rule_name || '');
            var input = document.createElement('input');
            input.type = 'text';
            input.className = 'proof-rule-edit';
            input.value = current;
            input.placeholder = side === 'left' ? 'label' : 'rule';
            function commit() {
                var val = input.value.trim();
                if (side === 'left') {
                    node.rule_label_left = val || null;
                } else {
                    node.rule_name = val || null;
                }
                rerender();
            }
            input.addEventListener('keydown', function(e) {
                if (e.key === 'Enter') commit();
                else if (e.key === 'Escape') rerender();
            });
            input.addEventListener('blur', commit);
            spanEl.replaceWith(input);
            input.focus();
            input.select();
        }

        function makeBtn(label, cls, handler) {
            var btn = document.createElement('button');
            btn.type = 'button';
            btn.className = 'proof-action-btn' + (cls ? ' ' + cls : '');
            btn.textContent = label;
            btn.addEventListener('click', function(e) { e.stopPropagation(); handler(); });
            return btn;
        }

        function renderProofNode(node, parentEl, path) {
            var el = document.createElement('div');
            el.className = 'proof-node';

            var isLeaf = !node.premises || node.premises.length === 0;
            var p = path.slice();

            if (!isLeaf) {
                var premisesEl = document.createElement('div');
                premisesEl.className = 'proof-premises';
                for (var i = 0; i < node.premises.length; i++) {
                    renderProofNode(node.premises[i], premisesEl, path.concat([i]));
                }
                var addBtn = makeBtn('+', 'proof-add-premise', function() { addPremise(p); });
                addBtn.title = 'Add premise';
                premisesEl.appendChild(addBtn);
                el.appendChild(premisesEl);

                var inferenceEl = document.createElement('div');
                inferenceEl.className = 'proof-inference';

                var ruleLeft = document.createElement('span');
                ruleLeft.className = 'proof-rule-label proof-rule-left';
                if (node.rule_label_left) {
                    ruleLeft.textContent = node.rule_label_left;
                } else {
                    ruleLeft.textContent = 'label';
                    ruleLeft.classList.add('proof-rule-placeholder');
                }
                ruleLeft.title = 'Click to add left label';
                (function(pp, rl) {
                    rl.addEventListener('click', function(e) {
                        e.stopPropagation();
                        editRuleLabel(pp, 'left', rl);
                    });
                })(p, ruleLeft);
                inferenceEl.appendChild(ruleLeft);

                var lineEl = document.createElement('span');
                lineEl.className = 'proof-line';
                inferenceEl.appendChild(lineEl);

                var ruleRight = document.createElement('span');
                ruleRight.className = 'proof-rule-label proof-rule-right';
                if (node.rule_name) {
                    ruleRight.textContent = node.rule_name;
                } else {
                    ruleRight.textContent = 'rule';
                    ruleRight.classList.add('proof-rule-placeholder');
                }
                ruleRight.title = 'Click to name this rule';
                (function(pp, rr) {
                    rr.addEventListener('click', function(e) {
                        e.stopPropagation();
                        editRuleLabel(pp, 'right', rr);
                    });
                })(p, ruleRight);
                inferenceEl.appendChild(ruleRight);

                var btnsWrap = document.createElement('span');
                btnsWrap.className = 'proof-line-btns';
                var clearBtn = makeBtn('\u00d7', 'proof-action-clear', function() { clearNode(p); });
                clearBtn.title = 'Clear this rule';
                btnsWrap.appendChild(clearBtn);
                if (canRemove(p)) {
                    var removeBtn = makeBtn('\u00d7', 'proof-action-remove', function() { removePremise(p); });
                    removeBtn.title = 'Remove this branch';
                    btnsWrap.appendChild(removeBtn);
                }
                inferenceEl.appendChild(btnsWrap);

                el.appendChild(inferenceEl);
            }

            if (isLeaf) {
                var leafZone = document.createElement('div');
                leafZone.className = 'proof-leaf-zone';
                var applyBtn = makeBtn('apply rule', '', function() { applyRule(p); });
                leafZone.appendChild(applyBtn);
                if (canRemove(p)) {
                    var rmBtn = makeBtn('\u00d7', 'proof-action-remove', function() { removePremise(p); });
                    rmBtn.title = 'Remove this branch';
                    leafZone.appendChild(rmBtn);
                }
                el.appendChild(leafZone);
            }

            var conclusionEl = document.createElement('div');
            conclusionEl.className = 'proof-conclusion';
            if (node.conclusion) {
                var formulaSpan = document.createElement('span');
                formulaSpan.className = 'proof-formula';
                formulaSpan.textContent = prettyFormula(node.conclusion);
                (function(pp, f) {
                    f.addEventListener('click', function(e) {
                        e.stopPropagation();
                        editConclusion(pp, f);
                    });
                })(p, formulaSpan);
                conclusionEl.appendChild(formulaSpan);
            } else {
                var emptySpan = document.createElement('span');
                emptySpan.className = 'proof-formula-placeholder';
                emptySpan.textContent = 'click to edit';
                (function(pp, f) {
                    f.addEventListener('click', function(e) {
                        e.stopPropagation();
                        editConclusion(pp, f);
                    });
                })(p, emptySpan);
                conclusionEl.appendChild(emptySpan);
            }
            el.appendChild(conclusionEl);

            parentEl.appendChild(el);
        }

        function rerender() {
            canvasEl.innerHTML = '';
            renderProofNode(proofTree, canvasEl, []);
            onChange(proofTree);
            if (vp) {
                setTimeout(function() { vp.autoFit(); }, 0);
            }
        }

        // Initial render
        rerender();

        return {
            getTree: function() { return proofTree; },
            getSexp: function() { return toSexp(proofTree); },
            setTree: function(tree) {
                proofTree = (typeof tree === 'string') ? parseSexp(tree) : tree;
                rerender();
            },
            rerender: rerender,
            destroy: function() {
                if (vp) {
                    vp.destroy();
                } else if (canvasEl.parentNode) {
                    canvasEl.parentNode.removeChild(canvasEl);
                }
            }
        };
    }

    // ── Auto-initialization from HTML attributes ───────

    function autoInit() {
        var elements = document.querySelectorAll('[data-proof-tree]');
        for (var i = 0; i < elements.length; i++) {
            var el = elements[i];
            if (el._proofTreeInit) continue; // already initialized
            var sexp = el.getAttribute('data-proof-tree');
            if (sexp) {
                var zoom = el.getAttribute('data-proof-tree-zoom') !== 'false';
                renderReadonly(sexp, el, { zoom: zoom });
                el._proofTreeInit = true;
            }
        }
    }

    if (typeof document !== 'undefined') {
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', autoInit);
        } else {
            setTimeout(autoInit, 0);
        }
    }

    return {
        prettyFormula: prettyFormula,
        parseSexp: parseSexp,
        toSexp: toSexp,
        renderReadonly: renderReadonly,
        createEditor: createEditor,
        autoInit: autoInit
    };
})();
