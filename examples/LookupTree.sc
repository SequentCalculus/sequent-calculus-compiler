data TreeInt { Leaf(x: Int), Node(left: TreeInt, right: TreeInt) }

//def create(i: Int, n: Int): TreeInt := ifl(i, n, let t: TreeInt = create(i + 1, n) in Node(t, t), Leaf(n));
//
//def lookup(t: TreeInt): Int := case t of { Leaf(v: Int) => v,
//                                           Node(left: TreeInt, right: TreeInt) => lookup(left) };

def create(i: Int, n: Int): TreeInt := ifl(i, n, let t: TreeInt = let j: Int = i + 1 in create(j, n) in Node(t, t), Leaf(n));

def lookup(t: TreeInt): Int := t.case { Leaf(v: Int) => v,
                                        Node(left: TreeInt, right: TreeInt) => lookup(left) };

def main(n: Int): Int := lookup(create(0, n));
