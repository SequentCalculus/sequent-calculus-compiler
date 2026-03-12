//Testcode for the module system
import seperatemodule

module submodulesimple
module deepmodule

def add2(x: i64, y: i64) : i64 {
    seperatemodule::add2(x, y)
}

def mul2(x: i64, y: i64) : i64 {
    deepmodule::mul2(x, y)
}

def sub2(x: i64, y: i64) : i64 {
    submodulesimple::sub2(x, y)
}