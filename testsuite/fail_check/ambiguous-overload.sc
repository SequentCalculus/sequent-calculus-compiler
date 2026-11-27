data Bool { True, False }

def f(x: i64): i64 { x }

def f(x: i64): Bool { True }

def main(): i64 {
    let y = f(1);
    0
}
