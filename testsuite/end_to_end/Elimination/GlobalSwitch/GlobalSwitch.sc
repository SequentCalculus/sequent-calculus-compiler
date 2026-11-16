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
    let construct: X = B(F(H(39)));
    let prevent_inlining: i64 = global_switch(0, construct, 0);
    let res: i64 = global_switch(1, construct, 2);
    println_i64(res);
    0
}