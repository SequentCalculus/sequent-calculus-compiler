data List[A] { Nil, Cons(x: A, xs: List[A]) }

def head(l: List[i64]): i64 {
    l.case[i64] {
        Nil => 
          println_i64(-1);
          exit -1,
        Cons(x, xs) => x
    }
}

def main(): i64 {
  let l: List[i64] = Nil;
  println_i64(head(l));
  0
}
