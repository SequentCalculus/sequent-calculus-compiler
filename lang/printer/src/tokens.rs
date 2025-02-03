//! This module contains the symbols and keywords of the surface language.
//! These constants are used when we prettyprint source code.

// Symbols
//
//

/// The symbol `=>`
pub const FAT_ARROW: &str = "=>";

/// The symbol `<-`
pub const LEFT_ARROW: &str = "<-";

/// The symbol `,`
pub const COMMA: &str = ",";

/// The symbol `:`
pub const COLON: &str = ":";

/// The symbol `.`
pub const DOT: &str = ".";

/// The symbol `=`
pub const EQ: &str = "=";

/// The symbol `:=`
pub const COLONEQ: &str = ":=";

/// The symbol `;`
pub const SEMI: &str = ";";

/// The symbol `:cns`
pub const CNS: &str = ":cns";

/// The symbol `*`
pub const TIMES: &str = "*";

/// The symbol `/`
pub const DIVIDE: &str = "/";

/// The symbol `%`
pub const MODULO: &str = "%";

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

/// The keyword `type`
pub const TYPE: &str = "type";

/// The keyword `def`
pub const DEF: &str = "def";

/// The keyword `let`
pub const LET: &str = "let";

/// The keyword `leta`
pub const LETA: &str = "leta";

/// The keyword `lit`
pub const LIT: &str = "lit";

/// The keyword `println_i64`
pub const PRINTLN_I64: &str = "println_i64";

/// The keyword `in`
pub const IN: &str = "in";

/// The keyword `case`
pub const CASE: &str = "case";

/// The keyword `cocase`
pub const COCASE: &str = "cocase";

/// The keyword `ife`
pub const IFE: &str = "ife";

/// The keyword `ifl`
pub const IFL: &str = "ifl";

/// The keyword `ifle`
pub const IFLE: &str = "ifle";

/// The keyword `ifz`
pub const IFZ: &str = "ifz";

/// The keyword `goto`
pub const GOTO: &str = "goto";

/// The keyword `label`
pub const LABEL: &str = "label";

/// The keyword `Done`
pub const DONE: &str = "Done";

/// The keyword `i64`
pub const I64: &str = "i64";

/// The keyword `invoke`
pub const INVOKE: &str = "invoke";

/// The keyword `new`
pub const NEW: &str = "new";

/// The keyword `switch`
pub const SWITCH: &str = "switch";

/// The keyword `jump`
pub const JUMP: &str = "jump";

/// The keyword `return`
pub const RETURN: &str = "return";

/// The keyword `substitute`
pub const SUBSTITUTE: &str = "substitute";

/// The keyword `else`
pub const ELSE: &str = "else";
