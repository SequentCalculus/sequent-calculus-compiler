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
    ├── core                  Sequent-calculus based core language Core
    ├── core2axcut            Compilation of Core to AxCut
    ├── fun                   Surface language Fun
    └── fun2core              Compilation of Fun to Core
```
