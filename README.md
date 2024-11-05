# Compiling through the Sequent Calculus

This repository contains a compiler which compiles a high-level functional programming language to machine code.
The intermediate languages are based on the sequent calculus.

## Installation


## Project Structure

```console
.
├── app                       CLI application
├── examples                  Example files
└── lang
    ├── axcut                 Sequent-calculus based core language AxCut
    ├── axcut2aarch64         Code generation backend for 64-Bit AArch
    ├── core                  Sequent-calculus based core language Core
    ├── core2axcut            Compilation of Core to AxCut
    ├── fun                   Surface language Fun
    ├── fun2core              Compilation of Fun to Core
    ├── parallel_moves        Implementation of algorithm to compute parallel moves
    └── printer               Infrastructure for prettyprinting with colorized
                              terminal and latex output
```

## Using the latex backend

It is possible to generate colorized latex output for code snippets using the `grokking texify` subcommand.
The colors used for syntax highlighting are not predefined but must be declared in the header of the latex file, using, for example, the following defaults:

```latex
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
```