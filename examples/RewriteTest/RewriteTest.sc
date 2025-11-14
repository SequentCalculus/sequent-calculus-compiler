data Pair[A, B] { Tup(x:A,y:B) }

def let_switch(x:i64,y:i64) : i64{
  let tup: Pair[i64,i64] = Tup(x,y);
  tup.case[i64,i64]{
    Tup(a,b) => a
  }
}

def main(): i64 {
  println_i64(let_switch(1,2));
  0
}
