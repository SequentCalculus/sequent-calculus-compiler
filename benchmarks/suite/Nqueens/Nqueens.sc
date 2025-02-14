data List[A] { Nil,Cons(a:A,as:List[A]) }
data Bool { True, False }

def and(b1:Bool,b2:Bool) : Bool {
  b1.case{
    True => b2,
    False => False 
  }
}

def neq_i(i1:i64,i2:i64) : Bool {
  if i1==i2 {
    False 
  } else {
    True
  }
}

def len_listlist_loop(l:List[List[i64]],acc:i64) : i64{
  l.case[List[i64]]{
    Nil => acc,
    Cons(is,iss) => len_listlist_loop(iss,acc+1)
  }
}

def len_listlist(l:List[List[i64]]) : i64 {
  len_listlist_loop(l,0)
}

def append_listlist(l1:List[List[i64]],l2:List[List[i64]]) : List[List[i64]] {
  l1.case[List[i64]]{
    Nil => l2,
    Cons(is,iss) => Cons(is,append_listlist(iss,l2))
  }
}

def safe(n:i64,m:i64,l:List[i64]) : Bool {
  l.case[i64]{
    Nil => True,
    Cons(i,is) => 
      and(neq_i(n,i),
        and(neq_i(n,i+m),
          and(neq_i(n,i-m),
            safe(n,m+1,is))))
  }
}

def check(l:List[List[i64]],acc:List[List[i64]],n:i64) : List[List[i64]] {
  l.case[List[i64]]{
    Nil => acc,
    Cons(is,iss) => safe(n,1,is).case{
      True => check(iss, Cons(Cons(n,is),acc),n),
      False => check(iss,acc,n)
    }
  }
}

def enumerate(n:i64,acc:List[List[i64]],bs:List[List[i64]]) : List[List[i64]] {
  if n==0{ 
    acc
  }else{
    let res: List[List[i64]] = check(bs,Nil,n);
    enumerate(n-1,append_listlist(res,acc),bs)
  }
}

def gen(n:i64) : List[List[i64]] {
  if n==0{
    Nil
  } else {
    let bs:List[List[i64]] = gen(n-1);
    enumerate(n,Nil,bs)
  }
}

def nsoln(n:i64) : i64 {
  len_listlist(gen(n))
}

def main_loop(iters:i64,n:i64) : i64{
  if iters==0{
    0
  }else{
    let res : i64 = nsoln(n);
    println_i64(res);
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64 {
  main_loop(iters,n)
}
