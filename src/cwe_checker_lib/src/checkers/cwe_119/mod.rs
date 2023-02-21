use crate::analysis::pointer_inference::Data;
use crate::prelude::*;
use crate::utils::log::{CweWarning, LogMessage, LogThread};
use crate::CweModule;

mod context;
use context::Context;
mod state;
use state::State;
mod stubs;

/// The module name and version
pub static CWE_MODULE: CweModule = CweModule {
    name: "CWE119",
    version: "0.3",
    run: check_cwe,
};

/// Run the check for CWE-119: Buffer Overflows.
///
/// This function prepares the fixpoint computation that computes the CWE warnings by setting the start states for all function starts.
/// Then the fixpoint computation is executed.
/// Afterwards, the collected logs and CWE warnings are collected from a separate logging thread and returned.
pub fn check_cwe(
    analysis_results: &AnalysisResults,
    _config: &serde_json::Value,
) -> (Vec<LogMessage>, Vec<CweWarning>) {
    let log_thread = LogThread::spawn(LogThread::collect_and_deduplicate);

    let context = Context::new(analysis_results, log_thread.get_msg_sender());

    let mut fixpoint_computation =   //把main里完整的分析结果传进去
        crate::analysis::forward_interprocedural_fixpoint::create_computation(context, None);  

    for (sub_tid, entry_node_of_sub) in
        crate::analysis::graph::get_entry_nodes_of_subs(analysis_results.control_flow_graph)    //遍历每个函数入口的基本块
    {
        if let Some(function_sig) = analysis_results.function_signatures.unwrap().get(&sub_tid) {
            let fn_start_state = State::new(&sub_tid, function_sig, analysis_results.project);
            fixpoint_computation.set_node_value(
                entry_node_of_sub,
                crate::analysis::interprocedural_fixpoint_generic::NodeValue::Value(fn_start_state),
            );
        }
    }

    fixpoint_computation.compute_with_max_steps(100);

    let (logs, mut cwe_warnings) = log_thread.collect();
    cwe_warnings.sort();
    (logs, cwe_warnings)
}
