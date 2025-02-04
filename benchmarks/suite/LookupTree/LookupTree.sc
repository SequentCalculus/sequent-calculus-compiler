data TreeI64 { Leaf(x: i64), Node(left: TreeI64, right: TreeI64) }

def create(i: i64, n: i64): TreeI64 { if i < n { let t: TreeI64 = create(i + 1, n); Node(t, t) } else { Leaf(n) } }

def lookup(t: TreeI64): i64 { t.case { Leaf(v: i64) => v,
                                       Node(left: TreeI64, right: TreeI64) => lookup(left) }}

def main(n: i64): i64 { println_i64(lookup(create(0, n)));
                        0 }
