data X { A, B(c: C) }
data C { D, E, F(g: G) }
data G { H(m: i64) }

def global_switch(l: i64, x: X, r: i64): i64 {
    x.case {
        A => -1,
        B(c) => c.case {
            D => l,
            E => r,
            F(g) => g.case {
                H(m) => (l + m) + r
            }
        }
    }
}

def main(): i64 {
    let construct: X = B(F(H(15)));
    let left: i64 = global_switch(4, construct, 2);
    let right: i64 = global_switch(4, construct, 2);
    println_i64(left + right);
    0
}