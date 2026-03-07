const { describe, it, beforeEach } = require('node:test');
const assert = require('node:assert/strict');
const { JSDOM } = require('jsdom');
const fs = require('fs');
const path = require('path');

const proofTreeSrc = fs.readFileSync(
  path.join(__dirname, '..', 'assets', 'proof-tree.js'),
  'utf8'
);

function createEnv() {
  const dom = new JSDOM('<!DOCTYPE html><html><body><div id="editor"></div></body></html>', {
    runScripts: 'dangerously',
    pretendToBeVisual: true,
  });
  // Load proof-tree.js into the jsdom window
  dom.window.eval(proofTreeSrc);
  return dom;
}

function makeTree(conclusion, ruleName, premises) {
  return {
    conclusion: conclusion,
    rule_name: ruleName || null,
    rule_label_left: ruleName ? 'L' : null,
    premises: premises || [],
  };
}

function clickFormulaAndEdit(dom, newValue) {
  const doc = dom.window.document;
  // Click the .proof-formula span to enter edit mode
  const formulaSpan = doc.querySelector('.proof-formula');
  assert.ok(formulaSpan, 'should find a .proof-formula span');
  formulaSpan.click();
  // Now an input should replace the span
  const input = doc.querySelector('.proof-formula-edit');
  assert.ok(input, 'should find .proof-formula-edit input after click');
  input.value = newValue;
  return input;
}

describe('editConclusion clears stale premises', () => {
  it('clears rule and premises when conclusion changes via Enter', () => {
    const dom = createEnv();
    const container = dom.window.document.getElementById('editor');
    const tree = makeTree('A => B', 'ImpI', [
      makeTree('B', null, []),
    ]);
    const editor = dom.window.ProofTree.createEditor(container, {
      zoom: false,
      tree: tree,
    });

    const input = clickFormulaAndEdit(dom, 'C => D');
    input.dispatchEvent(new dom.window.KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));

    const result = editor.getTree();
    assert.equal(result.conclusion, 'C => D');
    assert.equal(result.rule_name, null);
    assert.equal(result.rule_label_left, null);
    assert.deepEqual(result.premises, []);
  });

  it('clears rule and premises when conclusion changes via blur', () => {
    const dom = createEnv();
    const container = dom.window.document.getElementById('editor');
    const tree = makeTree('A => B', 'ImpI', [
      makeTree('B', null, []),
    ]);
    const editor = dom.window.ProofTree.createEditor(container, {
      zoom: false,
      tree: tree,
    });

    const input = clickFormulaAndEdit(dom, 'C => D');
    input.dispatchEvent(new dom.window.Event('blur'));

    const result = editor.getTree();
    assert.equal(result.conclusion, 'C => D');
    assert.equal(result.rule_name, null);
    assert.equal(result.rule_label_left, null);
    assert.deepEqual(result.premises, []);
  });

  it('preserves premises when conclusion is unchanged', () => {
    const dom = createEnv();
    const container = dom.window.document.getElementById('editor');
    const tree = makeTree('A => B', 'ImpI', [
      makeTree('B', null, []),
    ]);
    const editor = dom.window.ProofTree.createEditor(container, {
      zoom: false,
      tree: tree,
    });

    // "Edit" without actually changing the value
    const input = clickFormulaAndEdit(dom, 'A => B');
    input.dispatchEvent(new dom.window.KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));

    const result = editor.getTree();
    assert.equal(result.conclusion, 'A => B');
    assert.equal(result.rule_name, 'ImpI');
    assert.equal(result.rule_label_left, 'L');
    assert.equal(result.premises.length, 1);
  });

  it('works fine on a leaf node with no rule or premises', () => {
    const dom = createEnv();
    const container = dom.window.document.getElementById('editor');
    const tree = makeTree('X', null, []);
    const editor = dom.window.ProofTree.createEditor(container, {
      zoom: false,
      tree: tree,
    });

    const input = clickFormulaAndEdit(dom, 'Y');
    input.dispatchEvent(new dom.window.KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));

    const result = editor.getTree();
    assert.equal(result.conclusion, 'Y');
    assert.equal(result.rule_name, null);
    assert.deepEqual(result.premises, []);
  });
});
