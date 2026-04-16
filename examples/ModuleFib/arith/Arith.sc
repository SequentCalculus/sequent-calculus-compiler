import subtraction

public def add2(x: i64, y: i64) : i64 {
    add3(x,y,0) + add3(0,0,0)
}

def sub3(x: i64, y: i64) : i64 {
    subtraction::sub3(x, y)
}

def add3(x: i64, y: i64, z: i64) : i64 {
    x + sub3(y,0)
}
