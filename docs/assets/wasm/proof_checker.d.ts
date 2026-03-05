/* tslint:disable */
/* eslint-disable */

/**
 * Check which rules can apply to a given conclusion for a theory.
 * Returns JSON: { "RuleName": { "applicable": true/false, "reason": "..." }, ... }
 */
export function applicable_rules(conclusion: string, theory: string): string;

/**
 * Check a proof tree S-expression against a named theory.
 * Returns JSON: { valid, complete, diagnostics: [{level, path, message}] }
 */
export function check_proof(sexp_str: string, theory: string): string;

/**
 * Given a conclusion and a rule name, generate expected premise strings.
 * Returns JSON: { ok: true, premises: [...] } or { ok: false, error: "..." }
 */
export function generate_premises(conclusion: string, rule: string, theory: string): string;

/**
 * List available theories. Returns JSON array of {id, name} objects.
 */
export function list_theories(): string;

/**
 * Parse a judgement string for debugging. Returns JSON.
 */
export function parse_judgement(s: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly applicable_rules: (a: number, b: number, c: number, d: number) => [number, number];
    readonly check_proof: (a: number, b: number, c: number, d: number) => [number, number];
    readonly generate_premises: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly list_theories: () => [number, number];
    readonly parse_judgement: (a: number, b: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
