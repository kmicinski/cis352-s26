use std::collections::HashMap;
use serde::Serialize;
use crate::term::{Term, Strategy, encode_path};

/// Render tree node sent to JS for interactive visualization
#[derive(Serialize, Clone, Debug)]
pub struct RenderNode {
    pub kind: String,
    pub path: String,
    pub ops: Vec<String>,

    // Var fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binder_path: Option<String>,

    // Abs fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Box<RenderNode>>,

    // App fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub func: Option<Box<RenderNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<RenderNode>>,
}

/// Term-level info for the status bar
#[derive(Serialize)]
pub struct TermInfo {
    pub display: String,
    pub size: usize,
    pub free_vars: Vec<String>,
    pub redex_count: usize,
    pub is_normal_form: bool,
    pub strategy_next: Option<String>,
}

/// Build the render tree from a term.
/// `scope` maps variable names to the path of their binding lambda.
pub fn build_render_tree(
    term: &Term,
    path_str: &str,
    scope: &HashMap<String, String>,
    strategy: Option<&Strategy>,
    strategy_next_path: Option<&str>,
) -> RenderNode {
    match term {
        Term::Var(name) => {
            let is_free = !scope.contains_key(name);
            let binder_path = scope.get(name).cloned();
            let mut ops = Vec::new();
            if !is_free {
                // Can alpha-convert the binder
                ops.push("alpha".to_string());
            }
            RenderNode {
                kind: "var".to_string(),
                path: path_str.to_string(),
                ops,
                name: Some(name.clone()),
                is_free: Some(is_free),
                binder_path,
                param: None,
                body: None,
                func: None,
                arg: None,
            }
        }
        Term::Abs(param, body) => {
            let mut new_scope = scope.clone();
            new_scope.insert(param.clone(), path_str.to_string());
            let body_path = format!("{}B", path_str);
            let body_node = build_render_tree(
                body, &body_path, &new_scope, strategy, strategy_next_path,
            );

            let mut ops = vec!["alpha".to_string()];
            if term.is_eta_convertible() {
                ops.push("eta".to_string());
            }

            RenderNode {
                kind: "abs".to_string(),
                path: path_str.to_string(),
                ops,
                name: None,
                is_free: None,
                binder_path: None,
                param: Some(param.clone()),
                body: Some(Box::new(body_node)),
                func: None,
                arg: None,
            }
        }
        Term::App(func, arg) => {
            let func_path = format!("{}L", path_str);
            let arg_path = format!("{}R", path_str);
            let func_node = build_render_tree(
                func, &func_path, &scope, strategy, strategy_next_path,
            );
            let arg_node = build_render_tree(
                arg, &arg_path, &scope, strategy, strategy_next_path,
            );

            let mut ops = Vec::new();
            if term.is_redex() {
                ops.push("beta".to_string());
            }

            RenderNode {
                kind: "app".to_string(),
                path: path_str.to_string(),
                ops,
                name: None,
                is_free: None,
                binder_path: None,
                param: None,
                body: None,
                func: Some(Box::new(func_node)),
                arg: Some(Box::new(arg_node)),
            }
        }
    }
}

pub fn build_term_info(term: &Term, strategy: &Strategy) -> TermInfo {
    let mut fv: Vec<String> = term.free_vars().into_iter().collect();
    fv.sort();
    let next = term.next_redex(strategy);
    TermInfo {
        display: term.display_string(),
        size: term.size(),
        free_vars: fv,
        redex_count: term.all_redex_paths(&[]).len(),
        is_normal_form: next.is_none(),
        strategy_next: next.map(|p| encode_path(&p)),
    }
}
