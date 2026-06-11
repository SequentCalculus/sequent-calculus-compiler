use std::{
    io::{Error, ErrorKind, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use printer::Print;

use crate::mono::constraint_graph::ConstraintGraph;
use crate::syntax::Ty;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Quick raster image for local debugging.
    Png,
    /// Sharp vector graphic for direct inclusion in papers.
    Pdf,
    /// Native TikZ/LaTeX code for seamless document integration.
    Tikz,
}

impl OutputFormat {
    /// Returns the corresponding Graphviz command line argument.
    fn as_dot_arg(&self) -> &'static str {
        match self {
            OutputFormat::Png => "-Tpng",
            OutputFormat::Pdf => "-Tpdf",
            OutputFormat::Tikz => "-Ttikz",
        }
    }

    /// Returns the default file extension for this format.
    fn default_extension(&self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Pdf => "pdf",
            OutputFormat::Tikz => "tex",
        }
    }
}

impl ConstraintGraph {
    /// Renders the graph in Graphviz DOT format.
    ///
    /// Flat edges are solid arrows. Constructor edges are dashed arrows labeled
    /// with the constructor name. Seed values appear as rectangle nodes.
    /// Pipe the output into `dot -Tpng -o graph.png` to visualize.
    pub fn render_dot(&self) -> String {
        let mut out = String::from(
            r#"digraph constraints {
            rankdir=LR;
            splines=true;
            nodesep=0.8;
            ranksep=1.0;
            node [shape=ellipse, fontname=monospace];
            "#,
        );

        // Type variable nodes.
        for node in &self.nodes {
            out.push_str(&format!(
                "  n{} [label=\"{}\"];\n",
                node.id,
                node.print_to_string(None)
            ));
        }

        out.push('\n');

        // Seed nodes: one rectangle per variable showing all seeding ground types.
        for (var, seeds) in &self.seeds {
            let labels: Vec<String> = seeds.iter().map(ty_display_name).collect();
            out.push_str(&format!(
                "  seed_{id} [label=\"{{{types}}}\", shape=rectangle, \
                 style=filled, fillcolor=lightgrey];\n",
                id = var.id,
                types = labels.join(" | ")
            ));
            out.push_str(&format!("  seed_{} -> n{};\n", var.id, var.id));
        }

        out.push('\n');

        // Edges between type variables.
        for (source, edges) in &self.edges {
            for edge in edges {
                let style = if edge.is_constructor() {
                    format!(
                        "style=dashed, constraint=false, label=\"{}\"",
                        ty_display_name(&edge.from)
                    )
                } else {
                    String::new()
                };

                out.push_str(&format!(
                    "  n{} -> n{} [{}];\n",
                    source.id, edge.into.id, style
                ));
            }
        }

        out.push('}');
        out
    }

    /// Generates the Graphviz DOT string and renders it into the specified format.
    /// If `output_path` is `None`, it defaults to "graph.<extension>" in the current directory.
    pub fn render_as<P: AsRef<Path>>(
        &self,
        format: OutputFormat,
        output_path: Option<P>,
    ) -> std::io::Result<()> {
        // Generate the DOT string
        let dot_content = self.render_dot();

        // Determine the output path (fallback to default filename with correct extension)
        let path: PathBuf = match output_path {
            Some(p) => {
                let mut buf = PathBuf::from(p.as_ref());
                buf.set_extension(format.default_extension());
                buf
            }
            None => PathBuf::from(format!("graph.{}", format.default_extension())),
        };

        // Prepare and start the 'dot' process with the dynamic format argument
        let mut child = Command::new("dot")
            .arg(format.as_dot_arg())
            .arg("-o")
            .arg(path)
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| {
                Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Failed to execute 'dot' command. Is Graphviz installed? Error: {}",
                        e
                    ),
                )
            })?;

        // Write the DOT string to the standard input (stdin) of the 'dot' process
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(dot_content.as_bytes())?;
        }

        // Wait for the process to finish and check the exit status
        let status = child.wait()?;
        if !status.success() {
            return Err(Error::other(
                format!(
                    "Graphviz 'dot' process exited with an error code: {}",
                    status
                ),
            ));
        }

        Ok(())
    }
}

/// Produces a short human-readable label for a type, used in DOT output.
fn ty_display_name(ty: &Ty) -> String {
    match ty {
        Ty::I64 => "i64".to_owned(),
        Ty::Var(id) => id.print_to_string(None),
        Ty::Decl { name, type_args } => {
            if type_args.args.is_empty() {
                name.name.clone()
            } else {
                let args: Vec<String> = type_args.args.iter().map(ty_display_name).collect();
                format!("{}[{}]", name.name, args.join(", "))
            }
        }
    }
}
