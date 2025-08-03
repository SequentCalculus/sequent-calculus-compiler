codata Fun[A, B] { apply(x: A) : B }

def nonValueArguments() : i64 { new { apply(x) => new { apply(y) => y}}.apply[i64, Fun[i64,i64]](1 + 2).apply[i64, i64](3 + 4) }

def higherOrder() : i64 {  new { apply(x) => new { apply(y) => x.apply[i64, i64](y) }}.apply[Fun[i64,i64], Fun[i64,i64]](new { apply(z) => 4 + z}).apply[i64, i64](3 + 1) }

def main() : i64 { println_i64(higherOrder());
                   0 }
