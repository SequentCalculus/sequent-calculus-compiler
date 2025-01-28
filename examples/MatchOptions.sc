data OptionI64 { None, Some(x: i64) }

def attempt(i: i64): OptionI64 { ifz(i, Some(i), attempt(i - 1).case { None => None,
                                                                       Some(x: i64) => Some(x + 1) })}

def main(n: i64): i64 { println_i64(attempt(n).case { None => -1,
                                                       Some(x: i64) => x });
                         0 }
