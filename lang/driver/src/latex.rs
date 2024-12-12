use askama::Template;
use printer::PrintCfg;

use crate::paths::{AARCH64_PATH, X86_64_PATH};

pub const LATEX_END: &str = r"\end{alltt}
";

pub fn latex_start(fontsize: &String) -> String {
    let mut latex_start_string = "".to_string();
    latex_start_string.push_str("\\begin{alltt}\n");
    latex_start_string.push_str(&format!("\\{fontsize}"));
    latex_start_string.push_str("\\ttfamily");
    latex_start_string
}

pub const LATEX_PRINT_CFG: PrintCfg = PrintCfg {
    width: 80,
    latex: true,
    omit_decl_sep: true,
    indent: 4,
};

pub enum Arch {
    AARCH64,
    X86_64,
}
pub fn latex_all_template(name: String, backend: &Arch) -> String {
    let backend = match backend {
        Arch::AARCH64 => AARCH64_PATH,
        Arch::X86_64 => X86_64_PATH,
    };
    let all = AllTemplate {
        name: &name,
        backend,
    };
    all.render().unwrap()
}

#[derive(Template)]
#[template(path = "All.tex", escape = "none")]
struct AllTemplate<'a> {
    name: &'a str,
    backend: &'a str,
}
