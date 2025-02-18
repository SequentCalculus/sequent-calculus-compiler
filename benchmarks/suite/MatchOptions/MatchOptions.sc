data Option[A] { None, Some(x: A) }

def attempt(i: i64): Option[i64] {
  if i==0{
    Some(i)
  } else {
    attempt(i - 1).case[i64]{
      None => None, 
      Some(x) => Some(x + 1)
    }
  }
}

def main_loop(iters:i64,n:i64) : i64 {
  if iters == 0 {
    0
  }else{
  let res : i64 = attempt(n).case[i64] { 
    None => -1, 
    Some(x) => x 
  };
  main_loop(iters-1,n)
  }
}

def main(iters:i64, n: i64): i64 {
  main_loop(iters,n)
}
