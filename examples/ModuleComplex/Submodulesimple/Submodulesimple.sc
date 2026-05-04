import ModuleComplex::Seperatemodule

module Simplecode

public def sub2(x: i64, y: i64) : i64 {
    Simplecode::sub2(x, y)
}

public def add2(x: i64, y: i64) : i64 {
    Seperatemodule::add2(x, y)
}