data TreeInt { Leaf(x: Int), Node(left: TreeInt, right: TreeInt) }

//def create(i: Int, n: Int): TreeInt := ifz(n - i, let t: TreeInt = create(i + 1, n) in Node(t, t), Leaf(n));
//
//def lookup(t: TreeInt): Int := case t of { Leaf(v: Int) => v,
//                                           Node(left: TreeInt, right: TreeInt) => lookup(left) };

def create(i: Int, n: Int): TreeInt := let c: Int = n - i in ifz(c, let t: TreeInt = let j: Int = i + 1 in create(j, n) in Node(t, t), Leaf(n));

def lookup(t: TreeInt): Int := t.case { Leaf(v: Int) => v,
                                        Node(left: TreeInt, right: TreeInt) => lookup(left) };
