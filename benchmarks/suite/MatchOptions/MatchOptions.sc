data Option[A] { None, Some(x: A) }

def attempt(i: i64): Option[i64] { if i == 0 { Some(i) } else { attempt(i - 1).case[i64] { None => None,
                                                                                           Some(x: i64) => Some(x + 1) } } }

def main(n: i64): i64 { println_i64(attempt(n).case[i64] { None => -1,
                                                           Some(x: i64) => x });
                        0 }
