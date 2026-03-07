// Types for JSON data returned by the WASM LambdaEngine

export interface RenderNode {
    kind: 'var' | 'abs' | 'app';
    path: string;
    ops: string[];
    // var
    name?: string;
    is_free?: boolean;
    binder_path?: string | null;
    // abs
    param?: string;
    body?: RenderNode;
    // app
    func?: RenderNode;
    arg?: RenderNode;
    // memoized flat width (set at runtime)
    _fw?: number;
}

export interface TermInfo {
    display: string;
    size: number;
    free_vars: string[];
    redex_count: number;
    is_normal_form: boolean;
    strategy_next: string | null;
}

export interface TutorialStep {
    title: string;
    body: string;
    term?: string;
    strategyDemo?: boolean;
    autoSteps?: number;
    crDemo?: boolean;
}

export interface DTreeEdge {
    id: number;
    opLabel: string;
    redexPath: string;
}

export interface DTreeNode {
    id: number;
    displayStr: string;
    renderTreeJson: string | null;
    parentIds: number[];
    children: DTreeEdge[];
}

export type StrategyName = 'normal' | 'applicative' | 'cbv' | 'cbn';
