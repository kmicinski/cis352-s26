/* tslint:disable */
/* eslint-disable */

/**
 * Main engine exposed to JavaScript
 */
export class LambdaEngine {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Apply an operation at the given path
     * op: "beta", "eta", "alpha"
     * arg: for alpha, the new variable name
     */
    apply_operation(path_str: string, op: string, arg: string): string;
    can_undo(): boolean;
    /**
     * Get display string in CIS352 syntax
     */
    get_display(): string;
    /**
     * Get the render tree as JSON
     */
    get_render_tree(): string;
    get_strategy(): string;
    /**
     * Get term info for the status bar
     */
    get_term_info(): string;
    history_length(): number;
    constructor();
    /**
     * Parse a CIS352-syntax term and set it as current
     */
    parse_and_set(input: string): string;
    /**
     * Generate a random term with depth sampled from a Pareto distribution.
     * Most terms are sizable (depth 5-8), occasionally huge (depth 15+).
     */
    random_term(closed: boolean): string;
    /**
     * Set the reduction strategy
     */
    set_strategy(name: string): void;
    /**
     * Reduce one step according to the current strategy
     */
    step_strategy(): string;
    /**
     * Undo the last operation
     */
    undo(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_lambdaengine_free: (a: number, b: number) => void;
    readonly lambdaengine_apply_operation: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number, number];
    readonly lambdaengine_can_undo: (a: number) => number;
    readonly lambdaengine_get_display: (a: number) => [number, number];
    readonly lambdaengine_get_render_tree: (a: number) => [number, number, number, number];
    readonly lambdaengine_get_strategy: (a: number) => [number, number];
    readonly lambdaengine_get_term_info: (a: number) => [number, number, number, number];
    readonly lambdaengine_history_length: (a: number) => number;
    readonly lambdaengine_new: () => number;
    readonly lambdaengine_parse_and_set: (a: number, b: number, c: number) => [number, number, number, number];
    readonly lambdaengine_random_term: (a: number, b: number) => [number, number];
    readonly lambdaengine_set_strategy: (a: number, b: number, c: number) => [number, number];
    readonly lambdaengine_step_strategy: (a: number) => [number, number, number, number];
    readonly lambdaengine_undo: (a: number) => [number, number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
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
