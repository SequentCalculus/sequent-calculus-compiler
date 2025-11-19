data Pair[A, B] { Tup(x:A,y:B) }
codata Fun[A, B] { apply(x: A): B }

def let_switch(x:i64,y:i64) : i64{
  let tup: Pair[i64,i64] = Tup(x,y);
  tup.case[i64,i64]{
    Tup(a,b) => 
      let tup: Pair[i64,i64] = Tup(a,b);
      tup.case[i64,i64]{
        Tup(a,b) => 
          let tup: Pair[i64,i64] = Tup(a,b);
          tup.case[i64,i64]{
            Tup(a,b) => 
              let tup: Pair[i64,i64] = Tup(a,b);
              tup.case[i64,i64]{
                Tup(a,b) => a
              }
          }
      }
  }
}

def create_invoke(): i64{
  let y: i64 = 2;
  let f: Fun[i64,i64] = new {
    apply(x) => x + y
  };
  f.apply[i64,i64](1)
}

def main(): i64 {
  println_i64(let_switch(1,2));
  0
}
