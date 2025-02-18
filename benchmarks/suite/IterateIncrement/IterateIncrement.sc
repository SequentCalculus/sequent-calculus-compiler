codata Fun[A,B] { Ap(x:A) : B }

def iterate(i: i64, f: Fun[i64,i64], a: i64): i64 {
  if i==0 {
    a
  } else {
    iterate(i - 1, f, f.Ap[i64,i64](a))
  }
}

def main(n: i64): i64 {
  let res : i64 = iterate(n, new { Ap(x) => x + 1}, 0);
  0
}
