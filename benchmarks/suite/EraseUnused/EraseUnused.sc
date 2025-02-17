data List[A] { Nil, Cons(x: A, xs: List[A]) }

def useless(i: i64, n: i64, b: List[i64]): i64 {
  if i<n {
    useless(i + 1, n, replicate(0, i, Nil))
  } else {
    i
  }
}

def replicate(v: i64, n: i64, a: List[i64]): List[i64] {
  if n==0 {
    a 
  } else {
    replicate(v, n - 1, Cons(v, a))
  }
}

def main(n: i64): i64 {
  let res : i64 = useless(0, n, Nil);
  0
}
