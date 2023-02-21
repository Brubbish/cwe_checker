//没有误报和漏报
use crate::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{
    intermediate_representation::{ExternSymbol, Program, Sub, Term, Tid},
    utils::{
        log::{CweWarning, LogMessage},
        symbol_utils::get_calls_to_symbols,
    },
};
use serde::{Deserialize, Serialize};

const VERSION: &str = "0.1";

/// The module name and version
pub static CWE_MODULE: crate::CweModule = crate::CweModule {
    name: "CWE676",
    version: VERSION,
    run: check_cwe,
};

/// struct containing dangerous symbols from config.json
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Config {
    symbols: Vec<String>,
}

/// For each subroutine and each found dangerous symbol, check for calls to the corresponding symbol
pub fn get_calls<'a>(
    subfunctions: &'a BTreeMap<Tid, Term<Sub>>,
    dangerous_symbols: &'a HashMap<&'a Tid, &'a str>,
) -> Vec<(&'a str, &'a Tid, &'a str)> {
    let mut calls: Vec<(&str, &Tid, &str)> = Vec::new();
    for sub in subfunctions.values() {  //取value，subfunctions.root.0.node.vals...
        calls.append(&mut get_calls_to_symbols(sub, dangerous_symbols));    //还没看完
    }
    calls
}

/// Generate cwe warnings for potentially dangerous function calls
pub fn generate_cwe_warnings<'a>(
    dangerous_calls: Vec<(&'a str, &'a Tid, &'a str)>,
) -> Vec<CweWarning> {
    let mut cwe_warnings: Vec<CweWarning> = Vec::new();
    for (sub_name, jmp_tid, target_name) in dangerous_calls.iter() {
        let address: &String = &jmp_tid.address;
        let description: String = format!(
            "(Use of Potentially Dangerous Function) {sub_name} ({address}) -> {target_name}"   //所在函数+地址+危险函数，fomat!生成格式化的字符串赋值给左边的变量
        );

        /*
            CweWarning::new方法，
            前三个构建出了warning的框架，
            后面构建出了完整的warning需要的信息
        */
        let cwe_warning = CweWarning::new(  
            String::from(CWE_MODULE.name),
            String::from(CWE_MODULE.version),
            description,
        )
        .addresses(vec![address.clone()])
        .tids(vec![format!("{jmp_tid}")])
        .symbols(vec![String::from(*sub_name)])
        .other(vec![vec![
            String::from("dangerous_function"),
            String::from(*target_name),
        ]]);
        
        cwe_warnings.push(cwe_warning);
    }

    cwe_warnings.sort();
    cwe_warnings
}

/// Filter external symbols by dangerous symbols
pub fn resolve_symbols<'a>(
    external_symbols: &'a BTreeMap<Tid, ExternSymbol>,
    symbols: &'a [String],
) -> HashMap<&'a Tid, &'a str> {
    let dangerous_symbols: HashSet<&'a String> = symbols.iter().collect();
    external_symbols
        .iter()
        .filter_map(|(tid, symbol)| {//对(tid,symbol)进行过滤
            dangerous_symbols
                .get(&symbol.name)  //Option？？//symbol.name是string
                .map(|name| (tid, name.as_str()))
        })
        .collect()//将剩下的键值对（.iter）收集到hashmap并**返回**
}

/// Iterate through all function calls inside the program and flag calls to those functions
/// that are marked as unsafe via the configuration file.
pub fn check_cwe(                               //调用时传入&AnalysisResults结构体（位于results.rs，猜测是几种分析完的结果）和config.json指针
    analysis_results: &AnalysisResults,         //config.json: main.rs, :108
    cwe_params: &serde_json::Value,
) -> (Vec<LogMessage>, Vec<CweWarning>) {
    let project = analysis_results.project;
    let config: Config = serde_json::from_value(cwe_params.clone()).unwrap();   //危险函数集合
    let prog: &Term<Program> = &project.program;

    //subfunctions：所有子函数
    //external symbol：链接的库函数
    //dangerous symbol：对外部函数与config.json进行匹配
    //dangerous call：把程序用到的函数和上面的符号进行对比
    let subfunctions = &prog.term.subs;
    let external_symbols: &BTreeMap<Tid, ExternSymbol> = &prog.term.extern_symbols;
    let dangerous_symbols = resolve_symbols(external_symbols, &config.symbols);
    let dangerous_calls = get_calls(subfunctions, &dangerous_symbols);

    (vec![], generate_cwe_warnings(dangerous_calls))    //构建警告方法，返回（log，warning）
}
