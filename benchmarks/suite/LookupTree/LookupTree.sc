data Tree[A] { Leaf(x: A), Node(left: Tree[A], right: Tree[A]) }

def create(i: i64, n: i64): Tree[i64] {
  if i<n {
    let t: Tree[i64] = create(i + 1, n);
    Node(t, t)
  } else {
    Leaf(n)
  }
}

def lookup(t: Tree[i64]): i64 {
  t.case[i64] { 
    Leaf(v) => v,
    Node(left, right) => lookup(left) 
  }
}

def main(n: i64): i64 {
  let res : i64 = lookup(create(0, n));
  0
}
