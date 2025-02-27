data Bool { True, False }
data List[A] { Nil, Cons(a:A,as:List[A]) }
codata Fun[A,B] { Ap(a:A) : B }

def map_listlist(l:List[List[i64]],f:Fun[List[i64],List[i64]]) : List[List[i64]] {
  l.case[List[i64]]{
    Nil => Nil,
    Cons(l,ls) => Cons(f.Ap[List[i64],List[i64]](l),map_listlist(ls,f))
  }
}

def last_listlist(l:List[List[i64]]) : List[i64] {
  l.case[List[i64]] {
    Nil => Nil,
    Cons(l,ls) => ls.case{
      Nil => l,
      Cons(l,lls) => last_listlist(ls)
    }
  }
}

def pascal(n:i64) : List[List[i64]] {
  if n==0{
    Cons(Cons(1,Cons(2,Cons(1,Nil))),Nil)
  } else {
    let prev: List[List[i64]] = pascal(n-1);
    Nil
  }
}

def odd(n:i64) : Bool{
  if n%2==0{
    False
  }else{
    True
  }
}

def bernoulli(n:i64) : i64 {
  if n==0 {
    1
  } else {
    if n==1{
      -1*(1%2)
    }else{
      odd(n).case{
        True => 0,
        False => 0//long list comprehension
      }
    }
  }
}

def main_loop(iters:i64,n:i64) : i64{
  if iters==1{
    let res:i64 = bernoulli(n);
    println_i64(res);
    0
  }else {
    let res:i64 = bernoulli(n);
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64 {
  main_loop(iters,n)
}
