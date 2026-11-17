data Num { Five, Six }
data Either { L(left: Num), R(right: Num) }
codata Fun { apply(x: i64): i64 }

def get(e: Either): Num {
  e.case {
    L(l) => l,
    R(r) => r
  }
}

def main(): i64 {
  let f: Fun = new { apply(x) => if x == 0 { 0 } else { x + 1 }};
  let y: i64 = f.apply(3);
  if y == 5 {
    let z: i64 = f.apply(4);
    exit -1
  } else {
    let e: Either = R(Five);
    get(e).case {
      Six => -1,
      Five =>
        let w: i64 = f.apply(0);
        println_i64(w + 1);
        0
    }
  }
}
