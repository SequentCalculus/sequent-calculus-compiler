data List[A]{ Nil,Cons(x:A,xs:List[A]) }
codata Fun[A,B] { apply(a:A) : B }

def foo(l:List[i64]): List[i64] {
  l.case[i64]{
    Nil => Nil,
    Cons(x, xs) => xs.case[i64] {
      Nil => Nil,
      Cons(y, ys) => if y > 0 {
        foo(Cons(0, ys))
      } else {
        Cons(1, foo(ys))
      }
    }
  }
}

def map(l:List[i64],f:Fun[i64,i64]): List[i64] {
  l.case[i64]{
    Nil => Nil,
    Cons(x,xs) => Cons(f.apply[i64,i64](x),map(xs,f))
  }
}

def foo1(l : List[i64]): i64 {
  l.case[i64]{
    Nil => 1,
    Cons(x,xs) => let l: List[i64] = Cons(x,xs); foo2(l)
  }
}

def foo2(l : List[i64]): i64 {
  l.case[i64]{
    Nil => 0,
    Cons(x,xs) => let l: List[i64] = Cons(x,xs); foo1(l)
  }
}

def main(): i64 {
  let f: Fun[i64,i64] = new { apply(x) => x + 1 };
  let l: List[i64] = Cons(1,Cons(2,Cons(3,Cons(4,Nil))));
  let res: List[i64] = map(foo(l), f);
  println_i64(foo1(res));
  0
}
