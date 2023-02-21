//! ## How the check works
//!
//! 这句话太难懂了....通过数据流分析，寻找执行流，
//! 该执行流的内存访问发生在判断（有可能返回空指针的）函数返回值的条件跳转之前，有则找到positive
//! Using dataflow analysis, we search for an execution path where a memory access using the return value of
//! a symbol happens before the return value is checked through a conditional jump instruction.
//!
//! ### Symbols configurable in config.json
//!
//! The symbols are the functions whose return values are assumed to be potential
//! NULL pointers.
//!
//! ## False Positives
//!
//! - If a possible NULL pointer is temporarily saved in a memory location
//! that the [Pointer Inference analysis](crate::analysis::pointer_inference) could not track,
//! the analysis may miss a correct NULL pointer check and thus generate false positives.
//! - The analysis is intraprocedural.
//! If a parameter to a function is a potential NULL pointer,
//! this gets flagged as a CWE hit even if the function may expect NULL pointers in its parameters.
//! If a function returns a potential NULL pointer this gets flagged as a CWE hit,
//! although the function may be supposed to return potential NULL pointers.
//!
//! ## False Negatives
//!
//! - We do not check whether an access to a potential NULL pointer happens regardless
//! of a prior check.
//! - We do not check whether the conditional jump instruction checks specifically
//! for the return value being NULL or something else
//! - For functions with more than one return value we do not distinguish between
//! the return values.

use crate::analysis::forward_interprocedural_fixpoint::create_computation;
use crate::analysis::forward_interprocedural_fixpoint::Context as _;
use crate::analysis::graph::{Edge, Node};
use crate::analysis::interprocedural_fixpoint_generic::NodeValue;
use crate::intermediate_representation::*;
use crate::prelude::*;
use crate::utils::log::{CweWarning, LogMessage};
use crate::CweModule;
use petgraph::visit::EdgeRef;
use std::collections::BTreeMap;

mod state;
use state::*;

mod taint;
pub use taint::*;

mod context;
use context::*;

/// The module name and version
pub static CWE_MODULE: CweModule = CweModule {
    name: "CWE476",
    version: "0.3",
    run: check_cwe,
};

/// The configuration struct
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Config {
    /// The names of symbols for which the analysis should check
    /// whether the return values are checked for being a Null pointer by the analysed binary.
    symbols: Vec<String>,
}

/// Run the CWE check.
/// We check whether the return values of symbols configurable in the config file are being checked for Null pointers
/// before any memory access (and thus potential Null pointer dereferences) through these values happen.
pub fn check_cwe(
    analysis_results: &AnalysisResults,
    cwe_params: &serde_json::Value,
) -> (Vec<LogMessage>, Vec<CweWarning>) {
    let project = analysis_results.project;
    let pointer_inference_results = analysis_results.pointer_inference.unwrap();

    let (cwe_sender, cwe_receiver) = crossbeam_channel::unbounded();    //?

    let config: Config = serde_json::from_value(cwe_params.clone()).unwrap();
    let symbol_map = crate::utils::symbol_utils::get_symbol_map(project, &config.symbols[..]);
    let general_context = Context::new(project, pointer_inference_results, cwe_sender);


    //这块的分析还有点问题
    for edge in general_context.get_graph().edge_references() {
        if let Edge::ExternCallStub(jmp) = edge.weight() {  //只取外部调用且类型为jmp的边
            if let Jmp::Call { target, .. } = &jmp.term {   //找到被调用的目标函数
                if let Some(symbol) = symbol_map.get(target) {  //获取基本块当前状态

                    let node = edge.target();   //指向的基本块
                    let current_sub = match general_context.get_graph()[node] { //current_sub为被调用函数
                        Node::BlkStart(_blk, sub) => sub,   //（啥？）
                        _ => panic!(),
                    };
                    let mut context = general_context.clone();  //获取上下文
                    context.set_taint_source(jmp, current_sub); //污点源头设为jmp处

                /*
                    将污点源头设置为jmp，并将current_sub设置为被调用的函数
                    利用指针推断结果获取基本块的当前状态
                    为结点设置新值
                    设置最大步骤数进行计算
                */ 
                    let pi_state_at_taint_source =
                        match pointer_inference_results.get_node_value(node) {
                            Some(NodeValue::Value(val)) => Some(val.clone()),
                            _ => None,
                        };
                    let mut computation = create_computation(context, None);
                    computation.set_node_value(
                        node,
                        NodeValue::Value(State::new(symbol, pi_state_at_taint_source.as_ref())),
                    );
                    computation.compute_with_max_steps(100);
                }
            }
        }
    }

    let mut cwe_warnings = BTreeMap::new();
    for cwe in cwe_receiver.try_iter() {
        match &cwe.addresses[..] {
            [taint_source_address, ..] => cwe_warnings.insert(taint_source_address.clone(), cwe),
            _ => panic!(),
        };
    }
    let cwe_warnings = cwe_warnings.into_values().collect();

    (Vec::new(), cwe_warnings)
}
