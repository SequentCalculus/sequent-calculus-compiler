data Bool { True, False }
data Pair { Tup(a: i64, b: Bool) }

def main(): i64 {
    let x = True;
    let x = 1;
    Tup(x, True).case {
        Tup(y, x) => x.case { True => y, False => 0 }
    }
}
