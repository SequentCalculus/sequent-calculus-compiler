//! This module contains the symbols and keywords of the surface language.
//! These constants are used when we prettyprint source code.

// Symbols
//
//

/// The symbol `=>`
pub const FAT_ARROW: &str = "=>";

/// The symbol `,`
pub const COMMA: &str = ",";

/// The symbol `:`
pub const COLON: &str = ":";

/// The symbol `.`
pub const DOT: &str = ".";

/// The symbol `'`
pub const TICK: &str = "'";

/// The symbol `=`
pub const EQ: &str = "=";

/// The symbol `:=`
pub const COLONEQ: &str = ":=";

/// The symbol `;`
pub const SEMI: &str = ";";

/// The symbol `:cnt`
pub const CNT: &str = ":cnt";

/// The symbol `*`
pub const TIMES: &str = "*";

/// The symbol `+`
pub const PLUS: &str = "+";

/// The symbol `-`
pub const MINUS: &str = "-";

/// The symbol `<`
pub const LANGLE: &str = "<";

/// The symbol `>`
pub const RANGLE: &str = ">";

/// The symbol `|`
pub const PIPE: &str = "|";

// Keywords
//
//

/// The keyword `data`
pub const DATA: &str = "data";

/// The keyword `codata`
pub const CODATA: &str = "codata";

/// The keyword `def`
pub const DEF: &str = "def";

/// The keyword `let`
pub const LET: &str = "let";

/// The keyword `in`
pub const IN: &str = "in";

/// The keyword `case`
pub const CASE: &str = "case";

/// The keyword `cocase`
pub const COCASE: &str = "cocase";

/// The keyword `ifz`
pub const IFZ: &str = "ifz";

/// The keyword `goto`
pub const GOTO: &str = "goto";

/// The keyword `label`
pub const LABEL: &str = "label";

/// The keyword `Done`
pub const DONE: &str = "Done";

/// The keyword `Int`
pub const INT: &str = "Int";
