//Testcode for the module system
import Seperatemodule

module Submodulesimple
module Deepmodule

def add2(x: i64, y: i64) : i64 {
    Seperatemodule::add2(x, y)
}

def mul2(x: i64, y: i64) : i64 {
    Deepmodule::mul2(x, y)
}

def sub2(x: i64, y: i64) : i64 {
    Submodulesimple::sub2(x, y)
}