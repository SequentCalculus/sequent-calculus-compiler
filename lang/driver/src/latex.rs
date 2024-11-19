pub const LATEX_END: &str = r"\end{alltt}
";

pub fn latex_start(fontsize: &String) -> String {
    let mut latex_start_string = "".to_string();
    latex_start_string.push_str("\\begin{alltt}\n");
    latex_start_string.push_str(&format!("\\{fontsize}"));
    latex_start_string.push_str("\\ttfamily");
    latex_start_string
}
