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

BinOp ::= + | - | * | / | % |
Cmp ::= == | != | < | <= | > | >= 

ContextBinding ::= Var : Ty | Covar : Ty
TypingContext ::= ContextBinding*

Ty ::= i64 | Var | Name | Name[Var+]

Clause ::= Name => Term | Name(Var+) => Term

Term ::= int 
    | Var 
    | Name
    | Name(Term+)
    | label Label { Term }
    | goto Label ( Term )
    | exit Term
    | if ( Term Cmp Term ) { Term } else { Term }
    | print_i64(Term); Term
    | println_i64(Term); Term
    | let Var : Ty = Term; Term
    | Term.Name
    | Term.Name[Ty+]
    | Term.Name(Term+)
    | Term.Name[Ty+](Term+)
    | Term.case { Clause+ }
    | Term.case[Ty+] { Clause+ }
    | new { Clause+ }
    | Term BinOp Term
    | (Term)
    
    
CtorSig ::= Name | Name(TypingContext)
Data ::= data Name { CtorSig+ }
    | data Name[Var+] { CtorSig+ }

DtorSig ::= Name : Ty | Name(TypingContext) : Ty
Codata ::= codata Name { DtorSig+ } 
    | codata Name[Var+] { DtorSig+ }

Def ::= def Name(TypingContext) : Ty { Term }

Declaration ::= Data | Codata | Def 

Program ::= Declaration+
```
