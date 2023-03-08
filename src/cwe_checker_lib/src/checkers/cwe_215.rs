//! This module implements a check for CWE-215: Information Exposure Through Debug Information.
//!
//! Sensitive debugging information can be leveraged to get a better understanding
//! of a binary in less time.
//!
//! See <https://cwe.mitre.org/data/definitions/215.html> for a detailed description.
//!
//! ## How the check works
//!
//! For ELF binaries we check whether they contain sections containing debug information.
//! Other binary formats are currently not supported by this check.
//!
//! ## False Positives
//!
//! None known.
//!
//! ## False Negatives
//!
//! None known.

use crate::prelude::*;
use crate::utils::log::{CweWarning, LogMessage};
use crate::CweModule;

/// The module name and version
pub static CWE_MODULE: CweModule = CweModule {
    name: "CWE215",
    version: "0.2",
    run: check_cwe,
};

/// Run the check.
///
/// We simply check whether the ELF file still contains sections whose name starts with `.debug`.
/// Binary formats other than ELF files are currently not supported by this check.
pub fn check_cwe(
    analysis_results: &AnalysisResults,
    _cwe_params: &serde_json::Value,
) -> (Vec<LogMessage>, Vec<CweWarning>) {
    let binary = analysis_results.binary;

    match goblin::Object::parse(binary) {       //用goblin库直接分析二进制码
        Ok(goblin::Object::Elf(elf_binary)) => {    //如果是elf文件，创建一个elf_binary对象并赋给分析的结果
            for section_header in elf_binary.section_headers {  //所以从ghidra里没有提取出节区信息
                if let Some(section_name) = elf_binary.shdr_strtab.get_at(section_header.sh_name) { //shdr_strtab字符串表
                    if section_name.starts_with(".debug") {
                        let cwe_warning = CweWarning::new(
                            CWE_MODULE.name,
                            CWE_MODULE.version,
                            "(Information Exposure Through Debug Information) The binary contains debug symbols."
                        );
                        return (Vec::new(), vec![cwe_warning]);
                    }
                }
            }
            (Vec::new(), Vec::new())
        }
        Ok(_) => {
            let info_log = LogMessage::new_info(
                "File type not supported. Currently this check only supports ELF files.",
            )
            .source(CWE_MODULE.name);
            (vec![info_log], Vec::new())
        }
        Err(err) => {
            let err_log = LogMessage::new_error(format!("Error while parsing binary: {err}"))
                .source(CWE_MODULE.name);
            (vec![err_log], Vec::new())
        }
    }
}
