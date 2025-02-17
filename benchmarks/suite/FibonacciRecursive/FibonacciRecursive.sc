def fibonacci(i: i64): i64 {
  if i==0 {
    i
  } else {
    if i==1{
      i
    }else {
      fibonacci(i - 1) + fibonacci(i - 2)
    }
  }
}

def main(n: i64): i64 {
  let res : i64 = fibonacci(n);
  0
}
