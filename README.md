# Compiling with the Sequent Calculus

This repository contains a compiler which compiles a high-level functional
programming language to machine code, based on the accompanying paper "Compiling
with the Sequent Calculus". The intermediate languages are based on the sequent
calculus.

To use this project, you need a working installation of Rust 2024.

## Installation

Running

```console
make install
```

will install the compiler into your `cargo`-binary directory as `scc`.

## Documentation

Running

```console
make doc
```

generates the documentation of all crates of this project in `target/doc` in
html.

## Project Structure

```console
.
├── app                       CLI application
├── examples                  Example files
└── lang
    ├── axcut                 Sequent-calculus based language AxCut
    ├── axcut2aarch64         Code generation backend for 64-Bit ARM
    ├── axcut2backend         Abstraction layer for Code generation backend
    ├── axcut2rv64            Code generation backend for 64-Bit RISC-V
    ├── axcut2x86_64          Code generation backend for 64-Bit x86
    ├── core_lang             Sequent-calculus based core language Core
    ├── core2axcut            Compilation of Core to AxCut
    ├── fun                   Surface language Fun
    ├── fun2core              Compilation of Fun to Core
    └── printer               Infrastructure for prettyprinting with colorized
                              terminal and latex output
```

## Using the latex backend

It is possible to generate colorized latex output for code snippets using the
`scc texify` subcommand. The colors used for syntax highlighting are not
predefined but must be declared in the header of the latex file, using, for
example, the following defaults:

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

## The Language Grammar

The Grammar for the surface language **Fun** is defined in `lalrpop` syntax in
the file `lang/fun/src/parser/fun.lalrpop`. Here is the grammar in a more
standard form.

```
Var ::= String 
Covar ::= String 
Name ::= String
Label ::= String
Typevar ::= String

BinOp ::= + | - | * | / | % |
Cmp ::= == | != | < | <= | > | >= 

Ty ::= i64 | Var | Name[Typevar,...]

Clause ::= Name(Var,...) => Term

Term ::= int
    | Var 
    | Name(Term,...)
    | label Label { Term }
    | goto Label ( Term )
    | exit Term
    | if ( Term Cmp Term ) { Term } else { Term }
    | print_i64(Term); Term
    | println_i64(Term); Term
    | let Var : Ty = Term; Term
    | Term.Name[Ty,...](Term,...)
    | Term.case[Ty,...] { Clause,... }
    | new[Ty,...] { Clause,... }
    | Term BinOp Term
    | (Term)

ContextBinding ::= Var : Ty | Covar : Ty

CtorSig ::= Name(ContextBinding,...)
Data ::= data Name[Typevar,...] { CtorSig,... }
DtorSig ::= Name(ContextBinding,...) : Ty
Codata ::= codata Name[Typevar,...] { DtorSig,... } 
Def ::= def Name(ContextBinding,...) : Ty { Term }
Declaration ::= Data | Codata | Def,...
Program ::= Declaration,...
```

The rule `Name(Term,...)` is of note here, as it is both used for Constructor
and Top-Level Calls. In the case of Constructors, the arguments can be left out
completely when there are no arguments, for example `Nil` while for Top-Level
Calls, the parentheses are always needed, i.e. `main()`. For all other rules,
`(Rule,...)` or `[Rule,...]` means the parentheses or brackets can be completely
left out when there are no arguments. This is not the case for `{Rule,...}`, as
this is used for `case/new` and `CtorSig/DtorSig` and in both cases at least one
name/clause needs to be provided.
