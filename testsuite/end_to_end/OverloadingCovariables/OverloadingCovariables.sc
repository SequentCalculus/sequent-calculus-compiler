// overloads that differ in the number of parameters, one of them a covariable
def jump(x: i64, k: cns i64): i64 { goto k (x + 1) }

def jump(x: i64): i64 { x * 2 }

// overloads that differ only in the chirality of their parameter
def pick(x: i64): i64 { x + 100 }

def pick(k: cns i64): i64 { goto k (7) }

def main(): i64 {
    println_i64(label k { jump(5, k) });
    println_i64(jump(5));
    println_i64(pick(1));
    println_i64(label k { pick(k) });
    0
}
