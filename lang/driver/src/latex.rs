use printer::PrintCfg;

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

pub const LATEX_ALL_START: &str = r"\documentclass[12pt]{article}
\usepackage[utf8]{inputenc}
\usepackage{alltt}
\usepackage{xcolor}
% Color definitions
\definecolor{polBlack}{rgb}{0,0,0}
\definecolor{polBlue}{rgb}{0.06, 0.2, 0.65}
\definecolor{polGreen}{RGB}{0,155,85}
\definecolor{polRed}{rgb}{0.8,0.4,0.3}
\definecolor{polCyan}{rgb}{0.0, 1.0, 1.0}
\definecolor{polMagenta}{rgb}{0.8, 0.13, 0.13}
\definecolor{polYellow}{rgb}{0.91, 0.84, 0.42}
\definecolor{polWhite}{rgb}{1,1,1}

\newcommand*{\setTT}[1]{\texttt{#1}}
\newcommand{\polType}[1]{\textcolor{polRed}{\setTT{#1}}}
\newcommand{\polCtor}[1]{\textcolor{polBlue}{\setTT{#1}}}
\newcommand{\polDtor}[1]{\textcolor{polGreen}{\setTT{#1}}}
\newcommand{\polKw}[1]{\textcolor{polMagenta}{\setTT{#1}}}
\newcommand{\polVar}[1]{\setTT{#1}}
\newcommand{\polComment}[1]{\textcolor{polCyan}{\setTT{#1}}}
\newcommand*{\polText}[1]{\setTT{#1}}

\title{Grokking SC Rust}
\author{The Grokking SC Team}
\begin{document}

\maketitle";

pub enum Arch {
    AARCH64,
    X86_64,
}
pub fn latex_all_template(name: String, backend: &Arch) -> String {
    let mut string = LATEX_ALL_START.to_string();

    string.push_str(&format!(
        "\n\\section{{Program}}\n
    \\input{{./{name}.tex}}\n"
    ));

    string.push_str(&format!(
        "\\section{{Compiled}}\n
    \\input{{../compiled/{name}.tex}}\n
    "
    ));

    string.push_str(&format!(
        "\\section{{Focused}}\n
    \\input{{../focused/{name}.tex}}\n
    "
    ));

    string.push_str(&format!(
        "\\section{{Shrunk}}\n
    \\input{{../shrunk/{name}.tex}}\n
    "
    ));

    string.push_str(&format!(
        "\\section{{Linearized}}\n
    \\input{{../linearized/{name}.tex}}\n
    "
    ));

    match backend {
        Arch::AARCH64 => {
            string.push_str(&format!(
                "\\section{{Assembly}}\n
            \\input{{../assembly/aarch_64/{name}.tex}}\n
            "
            ));
        }
        Arch::X86_64 => {
            string.push_str(&format!(
                "\\section{{Assembly}}\n
            \\input{{../assembly/x86_64/{name}.tex}}\n
            "
            ));
        }
    }

    string.push_str("\\end{document}");
    string
}
