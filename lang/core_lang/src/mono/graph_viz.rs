//! Renders a `ConstraintGraph` as a Graphviz DOT file, for `--debug`/`--viz` inspection.

use std::{
    io::{Error, ErrorKind, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use printer::Print;

use crate::mono::constraint_graph::{ConstraintGraph, Node};
use crate::syntax::Ty;

/// Whether (and where) to render the constraint graph as a Graphviz file, 
/// a `--viz[=PATH]`-style CLI flag naturally parses to (flag absent, present without a value, 
/// present with an explicit path) with cases that name what each one means.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VizOutput {
    /// Do not render anything.
    #[default]
    Disabled,
    /// Render to the default path.
    DefaultPath,
    /// Render to the given path.
    Path(PathBuf),
}

impl VizOutput {
    /// Converts from the `Option<Option<PathBuf>>` a `--viz[=PATH]`-style `clap` flag parses to.
    pub fn from_cli_flag(flag: Option<Option<PathBuf>>) -> Self {
        match flag {
            None => VizOutput::Disabled,
            Some(None) => VizOutput::DefaultPath,
            Some(Some(path)) => VizOutput::Path(path),
        }
    }
}

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
    /// Each node represents a tuple of type parameters (a single
    /// identifier for ordinary declarations, several for multi-parameter
    /// declarations like `Pair[A, B]`). Flat edges are solid arrows.
    /// Constructor edges are dashed arrows labeled with the original
    /// `from` types. Seed nodes appear as rectangles listing every
    /// correlated ground tuple observed for that node.
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

        // Type variable vector nodes.
        for node in &self.nodes {
            out.push_str(&format!(
                "  {} [label=\"{}\"];\n",
                node_dot_id(node),
                node_label(node)
            ));
        }

        out.push('\n');

        // Seed nodes: one rectangle per node vector, listing all correlated
        // ground vectors observed for it.
        for (node, seeds) in &self.seeds {
            out.push_str(&format!(
                "  {seed_id} [label=\"{{{types}}}\", shape=rectangle, \
                 style=filled, fillcolor=lightgrey];\n",
                seed_id = seed_dot_id(node),
                types = seed_label(seeds)
            ));
            out.push_str(&format!(
                "  {} -> {};\n",
                seed_dot_id(node),
                node_dot_id(node)
            ));
        }

        out.push('\n');

        // Edges between node vectors. A single edge may appear under several
        // distinct source nodes if it depends on more than one; each
        // occurrence is drawn as its own arrow, which correctly reflects
        // that the edge genuinely depends on all of those source nodes.
        for (source, edges) in &self.edges {
            for edge in edges {
                let style = if edge.has_constructor_position() {
                    format!(
                        "style=dashed, constraint=false, label=\"{}\"",
                        from_types_label(&edge.from_types())
                    )
                } else {
                    String::new()
                };

                out.push_str(&format!(
                    "  {} -> {} [{}];\n",
                    node_dot_id(source),
                    node_dot_id(&edge.into),
                    style
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

        // Write the DOT string to the standard input of the 'dot' process
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(dot_content.as_bytes())?;
        }

        // Wait for the process to finish and check the exit status
        let status = child.wait()?;
        if !status.success() {
            return Err(Error::other(format!(
                "Graphviz 'dot' process exited with an error code: {}",
                status
            )));
        }

        Ok(())
    }
}

/// Produces a stable, Graphviz-safe identifier for a node vector, joining the
/// numeric ids of every identifier in the vector. Used as the DOT node name.
fn node_dot_id(node: &Node) -> String {
    let ids: Vec<String> = node.iter().map(|id| id.id.to_string()).collect();
    format!("n_{}", ids.join("_"))
}

/// Produces the corresponding seed rectangle's DOT identifier for a node.
fn seed_dot_id(node: &Node) -> String {
    format!("seed_{}", node_dot_id(node))
}

/// Produces a human-readable label for a node vector.
///
/// A singleton node like `[A]` is printed as just `A`. A multi-parameter
/// node like `[A, B]` is printed as `[A, B]`, making the correlation
/// between the bundled type parameters visible in the graph.
fn node_label(node: &Node) -> String {
    match node.as_slice() {
        [single] => single.print_to_string(None),
        many => {
            let parts: Vec<String> = many.iter().map(|id| id.print_to_string(None)).collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

/// Produces a human-readable label for the set of correlated ground vectors
/// seeding a node, joined with `|` so Graphviz renders them as a simple
/// record-style list.
fn seed_label(seeds: &std::collections::HashSet<Vec<Ty>>) -> String {
    let labels: Vec<String> = seeds
        .iter()
        .map(|tuple| tuple_display_name(tuple))
        .collect();
    labels.join(" | ")
}

/// Produces a human-readable label for one correlated vector of types, e.g.
/// `i64` for a singleton vector or `[i64, Bool]` for a pair.
fn tuple_display_name(tuple: &[Ty]) -> String {
    match tuple {
        [single] => ty_display_name(single),
        many => {
            let parts: Vec<String> = many.iter().map(ty_display_name).collect();
            format!("[{}]", parts.join(", "))
        }
    }
}

/// Produces a human-readable label for an edge's original `from` types,
/// reusing the same singleton-vs-vector formatting as [`tuple_display_name`].
fn from_types_label(types: &[Ty]) -> String {
    tuple_display_name(types)
}

/// Produces a short human-readable label for a single type, used in DOT output.
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
