data List[A] { Nil,Cons(a:A,as:List[A]) }
data Bool { True, False } 
codata Fun[A,B] { Ap(a:A):B }

def eq(i1:i64,i2:i64) : Bool{
  if i1==i2{
    True
  }else{
    False
  }
}

def expand(a:i64, b:i64, c:i64, d:i64, e:i64, f:i64) :i64 {
  (((((f+(e*10))+(d*100))+(c*1000))+(b*10000))+(a*100000))
}

def condition(thirywelvn:List[i64]) : Bool{
  thirywelvn.case[i64]{
    Nil => False,
    Cons(t,ts) => ts.case[i64]{
      Nil => False,
      Cons(h,hs) => hs.case[i64]{
        Nil => False,
        Cons(i,is) => is.case[i64]{
          Nil => False,
          Cons(r,rs) => rs.case[i64]{
            Nil => False,
            Cons(y,ys) => ys.case[i64]{
              Nil => False,
              Cons(w,ws) => ws.case[i64]{
                Nil => False,
                Cons(e,es) => es.case[i64]{
                  Nil => False,
                  Cons(l,ls) => ls.case[i64]{
                    Nil => False,
                    Cons(v,vs) => vs.case[i64]{
                      Nil => False,
                      Cons(n,ns) => ns.case[i64]{
                        Nil => eq((expand(t, h, i, r, t, y) + (5 * expand(t, w, e, l, v, e))), expand(n, i, n, e, t, y)),
                        Cons(i,is) => False
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}

def add_lscomp(p1:List[List[i64]],k:i64): List[List[i64]] {
  p1.case[List[i64]]{
    Nil => Nil,
    Cons(h1,t1) => Cons(Cons(k,h1),add_lscomp(t1,k))
  }
} 

def addj(j:i64, ls:List[i64]): List[List[i64]]{
  ls.case[i64]{
    Nil => Cons(Cons(j,Nil),Nil),
    Cons(k,ks) => Cons(Cons(j, Cons(k, ks)),add_lscomp(addj(j, ks),k))
  }
}


def perm_lscomp2(p2:List[List[i64]],t1:List[List[i64]],j:i64) : List[List[i64]]{
  p2.case[List[i64]]{
    Nil => perm_lscomp1(t1,j),
    Cons(r,t2) => Cons(r, perm_lscomp2(t2,t1,j))
  }
}

def perm_lscomp1(p1:List[List[i64]],j:i64) : List[List[i64]]{
  p1.case[List[i64]]{
    Nil => Nil,
    Cons(pjs,t1) => perm_lscomp2(addj(j, pjs),t1,j)
  }
}

def permutations(ls:List[i64]): List[List[i64]] {
  ls.case[i64]{
    Nil => Cons(Nil,Nil),
    Cons(j,js) => perm_lscomp1(permutations(js),j) 
  }
}

def enum_from_to(from:i64,t:i64) : List[i64]{
  if from==t{
    Nil
  }else{
    Cons(from,enum_from_to(from+1,t))
  }
}

def take(n:i64,l:List[i64]) : List[i64]{
  l.case[i64]{
    Nil => Nil,
    Cons(i,is) => if n==1 { Cons(i,Nil) } else { Cons(i,take(n-1,is)) }
  }
}

def filter(f:Fun[List[i64],Bool],l:List[List[i64]]) : List[List[i64]]{
  l.case[List[i64]]{
    Nil => Nil,
    Cons(l,ls) => f.Ap[List[i64],Bool](l).case{
      True => Cons(l,filter(f,ls)),
      False => filter(f,ls)
    }
  }
}

def map(f:Fun[i64,List[List[i64]]],l:List[i64]) : List[List[List[i64]]]{
  l.case[i64]{
    Nil => Nil,
    Cons(i,is) => Cons(f.Ap[i64,List[List[i64]]](i),map(f,is))
  }
}

def test_cryptarithm_nofib(n:i64) : List[List[List[i64]]]{
  map(new { Ap(i) => 
    let p0: List[i64] = take(10, enum_from_to(0, 9 + i));
    filter(new { Ap(l) => condition(l) }, permutations(p0))
  }, enum_from_to(1, n)) 
}

def first(l:List[List[List[i64]]]) : i64{
  l.case[List[List[i64]]]{
    Nil => -1,
    Cons(i,is) => i.case[List[i64]]{
      Nil => -1,
      Cons(i,is) => i.case[i64]{
        Nil => -1,
        Cons(i,is) => i
      }
    }
  }
}

def main_loop(iters:i64,n:i64) : i64{
  if iters == 1 {
    let res: List[List[List[i64]]] = test_cryptarithm_nofib(n);
    println_i64(first(res));
    0
  }else{
    let res: List[List[List[i64]]] = test_cryptarithm_nofib(n);
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64 {
  main_loop(iters,n)
}
