codata Stream[A] { Hd : A, Tl : Stream[A] }

def zeroes : Stream[i64] {
  print_i64(42);
  new {
    Hd => 0,
    Tl => zeroes()
  }
}

def main() : i64 { 
  let strm: Stream[i64] = zeroes();
  0
}
