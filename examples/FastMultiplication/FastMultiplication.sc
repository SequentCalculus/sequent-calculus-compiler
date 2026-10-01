data List[A] { Nil, Cons(x: A, xs: List[A]) }

// Fast multiplication function from the introduction of the paper.
def fmult(l : List[i64]) : i64 { label a { mult(l,a) } }
def mult(l : List[i64], a:cns i64) : i64 { 
  l.case[i64] { 
    Nil => print_i64(-12); println_i64(21); 1, 
    Cons(x , xs) => 
      print_i64(-24); 
      println_i64(42); 
      if x == 0 { 
        goto a (0)
      } else { 
        x * mult(xs, a) 
      }
  }
}

def main() : i64 { println_i64(fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil))))));
0 }
