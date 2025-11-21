data Pair[A, B] { Tup(x:A,y:B) }
codata Fun[A, B] { apply(x: A): B }
codata Stream[A] { head:A,tail:Stream[A] }

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
  let x: i64 = f.apply[i64,i64](1);
  let y: i64 = f.apply[i64,i64](2);
  x + y
}

def const1(): Stream[i64]  { new { tail => const1(), head => 1 } }

def create_invoke_stream(): i64 {
  let s: Stream[i64] = const1();
  let one: i64 = s.head[i64];
  let one_again : i64 = s.tail[i64].head[i64];
  one_again
}

def main(): i64 {
  println_i64(let_switch(1,2));
  0
}
