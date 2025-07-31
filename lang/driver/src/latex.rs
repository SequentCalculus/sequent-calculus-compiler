//! This module defines some utilities for generating LaTeX code for the different representations
//! of a program through the compilation pipeline.

use askama::Template;
use printer::PrintCfg;

use crate::paths::{AARCH64_PATH, X86_64_PATH};

/// This function generates the beginning of an environment containing a representation. It is
/// parametrized by the font size.
pub fn latex_start(fontsize: &str) -> String {
    let mut latex_start_string = String::new();
    latex_start_string.push_str("\\begin{alltt}\n");
    latex_start_string.push_str(&format!("\\{fontsize}"));
    latex_start_string.push_str("\\ttfamily");
    latex_start_string
}

/// This constant defines the end of an environment containing a representation.
pub const LATEX_END: &str = r"\end{alltt}
";

/// This constant defines some configurations for the layout.
pub const LATEX_PRINT_CFG: PrintCfg = PrintCfg {
    width: 80,
    latex: true,
    omit_decl_sep: true,
    indent: 4,
};

/// This enum encodes which backedn the representations are generated for.
pub enum Arch {
    AARCH64,
    X86_64,
}

/// This struct defines a template for all representations of a program. It consists of the name of
/// the program and the backend for which the representations are generated.
#[derive(Template)]
#[template(path = "All.tex", escape = "none")]
struct AllTemplate<'a> {
    name: &'a str,
    backend: &'a str,
}

/// This function generates latex code for all representations of a program for a given backend.
pub fn latex_all_template(name: &str, backend: &Arch) -> String {
    let backend = match backend {
        Arch::AARCH64 => AARCH64_PATH,
        Arch::X86_64 => X86_64_PATH,
    };
    let all = AllTemplate { name, backend };
    all.render().unwrap()
}
